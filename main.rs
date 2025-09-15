use std::{fs, time::Duration};
use serde::Deserialize;
use reqwest::Client;
use tokio::time::sleep;
use chrono::Local;

// ========= 設定 =========
#[derive(Debug, Deserialize)]
struct Config {
    wallet_id: u32,
    rpc_url: String,
    identity_p12_path: String,
    identity_p12_password: String,
    check_interval_sec: u64,
    notify_pending: bool,
    webhook: WebhookConfig, // ← GAS 用
    #[serde(default = "default_min_delta")]
    min_delta_xch: f64, // 既定 1e-9 XCH
}
fn default_min_delta() -> f64 {
    1e-9
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebhookConfig {
    pub url: String,
    pub token: String,
    pub to: String,
}

// ========= GAS Webhook送信 =========
async fn send_webhook(
    cfg: &WebhookConfig,
    subject: &str,
    body: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let payload = serde_json::json!({
        "token": cfg.token,
        "to":    cfg.to,
        "subject": subject,
        "body":    body,
    });

    let resp = reqwest::Client::new()
        .post(&cfg.url)
        .json(&payload)
        .send()
        .await?;

    let status = resp.status();
    let text = resp.text().await?;
    if !status.is_success() {
        return Err(format!("GAS webhook failed: {status} {text}").into());
    }
    println!("[webhook] OK: {text}");
    Ok(())
}

// ========= 残高取得 =========
async fn check_balance(
    config: &Config,
    client: &Client,
) -> Result<f64, Box<dyn std::error::Error>> {
    let url = format!("{}/get_wallet_balance", config.rpc_url);
    let body = serde_json::json!({ "wallet_id": config.wallet_id });

    println!("[RPC] POST {url}  body={}", body);

    let res = client.post(&url).json(&body).send().await?;
    let status = res.status();
    let text = res.text().await?;
    println!("[RPC] status={status} body={text}");

    if !status.is_success() {
        return Err(format!("wallet RPC failed: {status}").into());
    }

    let v: serde_json::Value = serde_json::from_str(&text)?;
    let confirmed_mojos = v["wallet_balance"]["confirmed_wallet_balance"]
        .as_u64()
        .unwrap_or(0);
    let balance = (confirmed_mojos as f64) / 1e12; // mojos → XCH
    Ok(balance)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&config_str)?;
    let send_disabled = std::env::var("PICCORO_NO_SEND").ok().as_deref() == Some("1");

    println!("Starting Wallet Piccoro...");

    // RPCクライアント
    let p12 = fs::read(&config.identity_p12_path)?;
    let identity =
        reqwest::Identity::from_pkcs12_der(&p12, &config.identity_p12_password)?;

    let client = Client::builder()
        .danger_accept_invalid_certs(true) // Chia RPCは自己署名
        .identity(identity)
        .timeout(Duration::from_secs(30))
        .build()?;

    // 監視ループ
    let mut prev_balance: Option<f64> = None;
    let mut last_notified_balance: Option<f64> = None;

    loop {
        let mut success = false;

        for _ in 0..3 {
            match check_balance(&config, &client).await {
                Ok(balance) => {
                    println!(
                        "[OK] Wallet {} balance: {} XCH",
                        config.wallet_id, balance
                    );

                    // エッジ検出: 前回より増えた時だけ
                    let increased = match prev_balance {
                        Some(p) => balance > p + config.min_delta_xch,
                        None => balance > 0.0 + config.min_delta_xch, // 初回は「正の残高」
                    };

                    // 同じ残高には二度送らない
                    let not_already_sent = match last_notified_balance {
                        Some(n) => balance > n + config.min_delta_xch,
                        None => true,
                    };

                    if config.notify_pending && increased && not_already_sent {
                        let subject = "Chia Wallet Update";
                        let body = format!(
                            "Balance: {} XCH\nwallet_id: {}\nchecked_at: {}",
                            balance,
                            config.wallet_id,
                            Local::now().to_rfc3339()
                        );

                        if send_disabled {
                            println!("[mail] notify skipped (disabled)");
                        } else if let Err(e) =
                            send_webhook(&config.webhook, &subject, &body).await
                        {
                            eprintln!("[mail] notify via GAS FAILED: {e:?}");
                        } else {
                            println!("[mail] notify via GAS OK");
                            last_notified_balance = Some(balance);
                        }
                    }

                    prev_balance = Some(balance);
                    success = true;
                    break;
                }
                Err(e) => {
                    eprintln!("[ERR] RPC failed: {e}");
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }

        if !success {
            eprintln!("[FATAL] RPC failed after 3 retries.");
        }

        sleep(Duration::from_secs(config.check_interval_sec)).await;
    }
}
