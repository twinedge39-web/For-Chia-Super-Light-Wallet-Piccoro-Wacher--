# Wallet Piccoro

Chia Wallet RPC を監視し、残高が更新されたら通知する超軽量ウォッチャー。  
Rust + Tokio + Reqwest で実装。送金機能や鍵操作は一切含まれません。  

---

## 環境要件
- **OS**: Windows 10+, macOS 12+, Linux (Ubuntu 20.04+ 推奨)
- **ランタイム**
  - Rust (rustup 推奨)
  - Node.js 18 以上（Tauri GUI 用）
- **ビルド環境**
  - Windows: Visual Studio Build Tools, Windows 10 SDK
  - macOS: Xcode Command Line Tools
  - Linux: `build-essential libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev`

---

## 機能
- Chia Wallet RPC (`https://127.0.0.1:9256`) に定期リクエスト
- バランス変化を検出して通知
- 証明書は `.p12` (PKCS#12) 形式で読み込み
- 設定は `config.json` で管理
- 通知方式は2系統：
  - **SMTPメール**（実装未定）  
  - **Webhook (Google Apps Script, Discord, LINE Notify など)**


---
## Chia ノード要件
- ローカルで **Chia Wallet RPC** が動作していること
  - URL: `https://127.0.0.1:9256`
- 証明書ファイル（リポジトリには含めない）
  - `.p12` ファイル（例: `wallet_identity.p12`）
- 対象ウォレットID（通常は 1）
---

### SMTPメール
- SMTP対応アカウント（例: Gmail, Outlook）
- 推奨: 2FA有効化 + アプリパスワード  
- 必要情報: `host`, `port`, `user`, `pass`, `to`

### Webhook (推奨: GAS)
- Google Apps Script をデプロイして Webhook URL を発行
- 設定項目: `url`, `token`, `to`

---

## 推奨ファイル構成
- `config.json` （.gitignore対象）
  - 設定例は `config.example.json` を参照
- `assets/` にロゴアイコン
- `docs/` にこの動作条件メモと追加ドキュメント

wallet-piccoro/
├─ src/main.rs
├─ Cargo.toml
├─ Cargo.lock
├─ config.json # 実際の設定（.gitignore対象）
├─ config.example.json # サンプル設定
├─ assets/ # ロゴやアイコン類
└─ docs/ # この README や追加ドキュメント

---

## 設定ファイル例 (`config.json`)
```json
{
  "wallet_id": 1,
  "rpc_url": "https://127.0.0.1:9256",
  "identity_p12_path": "wallet_identity.p12",
  "identity_p12_password": "",
  "check_interval_sec": 60,
  "notify_pending": true,
  "smtp": {
    "host": "smtp.gmail.com",
    "port": 587,
    "user": "you@gmail.com",
    "pass": "app_password_here",
    "to": "dest@gmail.com"
  },
  "webhook": {
    "url": "https://script.google.com/macros/s/XXXX/exec",
    "token": "your-long-secret-token",
    "to": "dest@gmail.com"
  }
}
```

## インストールと実行
```bash
cargo build --release
.\target\release\wallet-piccoro.exe
```

---

## 注意
- **秘密ファイルは絶対にリポジトリに含めないこと**
  - 証明書、鍵、ウォレットDB (`*.sqlite`) は外部に置く
- 本ツールは **入金通知専用**。送金機能や鍵管理は行わない。
- ノード停止中は通知できないが、自動復帰時に動作再開する。
- check_interval_sec は短すぎると RPC に蹴られることがあるので、30秒以上を推奨
- Windows 環境では OpenSSL の DLL を PATH に通す必要がある（合成キーの生成時）
- 自己署名証明書を使うため danger_accept_invalid_certs(true) を有効化している

---

[← プロジェクトトップへ戻る](../README.md)

---

# Operating Conditions Memo (Wallet Piccoro)

A tool that monitors the Chia Wallet RPC and sends an email notification when the balance is updated.  
This is an ultra-lightweight watcher with no sending or key management functionality.  

---

## Environment Requirements
- **OS**: Windows 10+, macOS 12+, Linux (Ubuntu 20.04+ recommended)
- **Runtime**
  - Rust (rustup recommended)
  - Node.js 18 or later (for Tauri GUI)
- **Build Environment**
  - Windows: Visual Studio Build Tools, Windows 10 SDK
  - macOS: Xcode Command Line Tools
  - Linux: `build-essential libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev`

---

## Features
- Periodically sends requests to Chia Wallet RPC (`https://127.0.0.1:9256`)
- Detects balance changes and sends email notifications
- Loads certificates in `.p12` (PKCS#12) format
- Manages settings with `config.json`
- Supports two notification methods:
  - **SMTP mail**
  - **Webhook** (Google Apps Script, Discord, LINE Notify, etc.)

---

## Chia Node Requirements
- **Chia Wallet RPC** must be running locally  
  - URL: `https://127.0.0.1:9256`
- Certificate file (not included in the repository)  
  - `.p12` file (e.g., `wallet_identity.p12`)
- Target wallet ID (usually 1)

---

### SMTP Mail
- SMTP-enabled email account (e.g., Gmail/Outlook)
- 2FA enabled + App Password (if supported)
- Required fields: `host`, `port`, `user`, `pass`, `to`

### Webhook (Recommended: GAS)
- Deploy a Google Apps Script (or other webhook service)
- Provide a Webhook `url`, `token`, and recipient `to` address
- Safer than storing SMTP credentials inside the app

---

## Recommended File Structure
- `config.json` (ignored by .gitignore)  
  - See `config.example.json` for sample settings
- `assets/` for logo icons
- `docs/` for this operating conditions memo and additional documents

wallet-piccoro/
├─ src/main.rs
├─ Cargo.toml
├─ Cargo.lock
├─ config.json # actual configuration (ignored by .gitignore)
├─ config.example.json # sample configuration
├─ assets/ # logos and icons
└─ docs/ # this README and additional documents

## Installation and Execution

```bash
cargo build --release
.\target\release\wallet-piccoro.exe
```
Notes

Never include secret files in the repository

- Certificates, keys, and wallet DB (*.sqlite) must be stored externally
- This tool is for deposit notifications only. It does not handle sending or key management.
- Notifications cannot be sent while the node is stopped, but will resume automatically once it restarts.
- If check_interval_sec is too short, RPC may reject the requests — 30 seconds or longer is recommended.
- On Windows, OpenSSL DLLs must be added to PATH (when generating composite keys).
- Since a self-signed certificate is used, danger_accept_invalid_certs(true) is enabled.

---

[← Back to Project Top](../README.md)
