use tauri::{Manager, tray::TrayIconBuilder};
use serde::{Serialize, Deserialize};
use std::{collections::HashSet, sync::Mutex};
use tokio::sync::mpsc;

#[derive(Serialize, Deserialize, Clone)]
struct Config {
  wallet_id: u32,
  interval_sec: u64,
  notify_pending: bool,
  smtp_host: String,
  smtp_port: u16,
  smtp_user: String,
  smtp_pass: String, // アプリパス推奨
  mail_to: String,
  cert_path: String,
  key_path: String,
  rpc_url: String, // 既定: https://127.0.0.1:9256
}

struct AppState {
  cfg: Mutex<Config>,
  seen: Mutex<HashSet<String>>,
}

#[tauri::command]
fn save_config(state: tauri::State<AppState>, cfg: Config) -> Result<(), String> {
  *state.cfg.lock().unwrap() = cfg.clone();
  // AppConfigDirに保存
  let app = tauri::AppHandle::current().unwrap();
  let path = app.path().app_config_dir().unwrap().join("config.json");
  std::fs::create_dir_all(path.parent().unwrap()).ok();
  std::fs::write(path, serde_json::to_vec(&cfg).unwrap()).map_err(|e| e.to_string())
}

#[tauri::command]
fn load_config(app: tauri::AppHandle) -> Option<Config> {
  let path = app.path().app_config_dir().ok()?.join("config.json");
  std::fs::read(path).ok().and_then(|b| serde_json::from_slice(&b).ok())
}

#[tokio::main]
async fn main() {
  tauri::Builder::default()
    .setup(|app| {
      // トレイ
      let tray = TrayIconBuilder::new()
        .on_tray_icon_event(|app, _e| {
          let _ = app.get_webview_window("settings")
              .map(|w| w.show().ok()).unwrap_or_default();
        })
        .build(app)?;
      // 状態
      let cfg = load_config(app.handle()).unwrap_or(Config {
        wallet_id: 1, interval_sec: 5, notify_pending: false,
        smtp_host: "smtp.gmail.com".into(), smtp_port: 465,
        smtp_user: "".into(), smtp_pass: "".into(),
        mail_to: "".into(),
        cert_path: default_cert_path(), key_path: default_key_path(),
        rpc_url: "https://127.0.0.1:9256".into(),
      });
      app.manage(AppState { cfg: Mutex::new(cfg), seen: Mutex::new(HashSet::new()) });

      // 監視タスク起動
      let handle = app.handle().clone();
      tauri::async_runtime::spawn(async move { watch_loop(handle).await });

      Ok(())
    })
    .plugin(tauri_plugin_notification::init())
    .run(tauri::generate_context!())
    .expect("error");
}

async fn watch_loop(app: tauri::AppHandle) {
  loop {
    let (cfg, mut new_lines) = {
      let state: tauri::State<AppState> = app.state();
      (state.cfg.lock().unwrap().clone(), Vec::new())
    };
    if let Ok(txs) = get_transactions(&cfg).await {
      let state: tauri::State<AppState> = app.state();
      let mut seen = state.seen.lock().unwrap();
      let mut news = vec![];
      for t in txs {
        let incoming = t.amount > 0;
        let ok = incoming && (t.confirmed || cfg.notify_pending);
        if ok && !seen.contains(&t.id) {
          news.push(t);
        }
      }
      if !news.is_empty() {
        for t in &news { seen.insert(t.id.clone()); }
        let body = format_incomings(&news);
        // OS通知
        let _ = app.notification().builder().title("Chia 入金").body(&body).show();
        // メール
        let _ = send_mail(&cfg, "[Chia] 入金検知", &body).await;
        // トレイツールチップ更新（残高も必要なら取得して表示）
        let _ = app.tray_by_id("main").map(|tray| tray.set_tooltip(Some("入金あり")));
      }
    }
    let state: tauri::State<AppState> = app.state();
    let sec = state.cfg.lock().unwrap().interval_sec;
    tokio::time::sleep(std::time::Duration::from_secs(sec)).await;
  }
}

// … get_transactions(), send_mail() 実装（reqwest/lettreで） …
