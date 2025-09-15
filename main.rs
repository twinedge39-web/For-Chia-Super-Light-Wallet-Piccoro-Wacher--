use std::{fs, time::Duration};
use serde::Deserialize;
use reqwest::Client;
use tokio::time::sleep;
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;

#[derive(Debug, Deserialize)]
struct Config {
    wallet_id: u32,
    rpc_url: String,
    // PKCS#12 (PFX) を使う
    identity_p12_path: String,
    identity_p12_password: String, // 空文字でもOK
    check_interval_sec: u64,
    notify_pending: bool,
    smtp: SmtpConfig,
}

#[derive(Debug, Deserialize)]
struct SmtpConfig {
    host: String,
    port: u16,
    user: String,
    pass: String,
    to: String,
}

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
    let confirmed = v["wallet_balance"]["confirmed_wallet_balance"].as_f64().unwrap_or(0.0);
    Ok(confirmed / 1e12) // mojos → XCH
}

fn send_mail(config: &Config, subject: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from(config.smtp.user.parse()?)
        .to(config.smtp.to.parse()?)
        .subject(subject)
        .body(body.to_string())?;

    let creds = Credentials::new(config.smtp.user.clone(), config.smtp.pass.clone());

    let mailer = SmtpTransport::relay(&config.smtp.host)?
        .port(config.smtp.port)
        .credentials(creds)
        .build();

    mailer.send(&email)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // config.json を読む
    let config_str = fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&config_str)?;

    println!("Starting Wallet Piccoro...");

    // .p12 を読み込んで Identity にする（cert+key連結は不要）
    let p12 = fs::read(&config.identity_p12_path)?;
    let identity = reqwest::Identity::from_pkcs12_der(&p12, &config.identity_p12_password)?;

    let client = Client::builder()
        .danger_accept_invalid_certs(true) // Chia RPC は自己署名
        .identity(identity)
        .timeout(Duration::from_secs(10)) // タイムアウト
        .build()?;

    loop {
        match check_balance(&config, &client).await {
            Ok(balance) => {
                println!("[OK] Wallet {} balance: {} XCH", config.wallet_id, balance);
                if balance > 0.0 && config.notify_pending {
                    let _ = send_mail(
                        &config,
                        "Chia Wallet Update",
                        &format!("Balance: {} XCH", balance),
                    );
                }
            }
            Err(e) => eprintln!("[ERR] {e}"),
        }

        sleep(Duration::from_secs(config.check_interval_sec)).await;
    }
}
