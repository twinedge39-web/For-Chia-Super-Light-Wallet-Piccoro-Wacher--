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
async fn check_balance(config: &Config, client: &Client) -> Result<f64, Box<dyn std::error::Error>> {
    let url = format!("{}/get_wallet_balance", config.rpc_url);
    let body = serde_json::json!({ "wallet_id": config.wallet_id });

    println!("[RPC] POST {url}  body={}", body);

    let res = client.post(&url)
        .json(&body)
        .send()
        .await?;

    let status = res.status();
    let text = res.text().await?;
    println!("[RPC] status={status} body={text}");

    if !status.is_success() {
        return Err(format!("wallet RPC failed: {status}").into());
    }

    let v: serde_json::Value = serde_json::from_str(&text)?;
    let confirmed_mojos = v["wallet_balance"]["confirmed_wallet_balance"].as_u64().unwrap_or(0);
    let balance = (confirmed_mojos as f64) / 1e12; // mojos → XCH
    Ok(balance)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&config_str)?;

    println!("Starting Wallet Piccoro...");

    // 起動直後のテスト送信（GAS経由）
    if std::env::var("PICCORO_TEST").ok().as_deref() == Some("1") {
        let subject = "[Piccoro] GAS webhook test";
        let body = format!(
            "Wallet Piccoro test (GAS)\nwallet_id: {}\ncheck_time: {}",
            config.wallet_id,
            Local::now().to_rfc3339()
        );
        match send_webhook(&config.webhook, &subject, &body).await {
            Ok(_) => println!("[mail] GAS test send OK"),
            Err(e) => eprintln!("[mail] GAS test send FAILED: {e:?}"),
        }
        return Ok(()); // テストだけで終了
    }

    // RPCクライアント
    let p12 = fs::read(&config.identity_p12_path)?;
    let identity = reqwest::Identity::from_pkcs12_der(&p12, &config.identity_p12_password)?;

    let client = Client::builder()
        .danger_accept_invalid_certs(true) // Chia RPCは自己署名
        .identity(identity)
        .timeout(Duration::from_secs(20))
        .build()?;

    // 監視ループ
    loop {
        let mut success = false;

        for _ in 0..3 {
            match check_balance(&config, &client).await {
                Ok(balance) => {
                    println!("[OK] Wallet {} balance: {} XCH", config.wallet_id, balance);

                    if balance > 0.0 && config.notify_pending {
                        let subject = "Chia Wallet Update";
                        let body = format!(
                            "Balance: {} XCH\nwallet_id: {}\nchecked_at: {}",
                            balance, config.wallet_id, Local::now().to_rfc3339()
                        );
                        match send_webhook(&config.webhook, &subject, &body).await {
                            Ok(_) => println!("[mail] notify via GAS OK"),
                            Err(e) => eprintln!("[mail] notify via GAS FAILED: {e:?}"),
                        }
                    }

                    success = true;
                    break;
                }
                Err(e) => {
                    eprintln!("[ERR] RPC failed: {e}");
                    sleep(Duration::from_secs(5)).await; // 5秒待って再試行
                }
            }
        }

        if !success {
            eprintln!("[FATAL] RPC failed after 3 retries.");
        }

        sleep(Duration::from_secs(config.check_interval_sec)).await;
    }
}
