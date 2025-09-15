# 動作条件メモ (Wallet Piccoro)

このツールは **Chiaウォレットの残高チェックと入金通知** のみに特化した
超軽量ウォッチャーです。送金機能や鍵操作は一切含まれていません。

Chia Wallet RPC を監視して、残高が更新されたらメールで通知するツール。  
Rust + Tokio + Reqwest + Lettre で実装。

---

## 必要環境
- Rust (1.80+ 推奨)
- OpenSSL (Win32/Win64 OpenSSL Light インストール済み)
- Chia ノード (wallet RPC が有効化されていること)
- **ビルド環境**
  - Windows: Visual Studio Build Tools, Windows 10 SDK
  - macOS: Xcode Command Line Tools
  - Linux: `build-essential libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev`

---

## 機能
- Chia Wallet RPC (`https://127.0.0.1:9256`) へ定期的にリクエスト
- バランスの変化を検出してメール通知
- 証明書は `.p12` (PKCS#12) で読み込み
- 設定は `config.json`

---
## Chiaノード要件
- ローカルで **ChiaウォレットRPC** が動作していること  
  - URL: `https://127.0.0.1:9256`
- 証明書ファイル（リポジトリには含めない）
  - `private_wallet.crt`
  - `private_wallet.key`
- 対象ウォレットID（通常は 1）

---

## メール通知要件
- SMTP対応メールアカウント（Gmail/Outlook等）
- 2FA有効化 + **アプリパスワード**発行済み
- 必要情報: `host`, `port`, `user`, `pass`, `to`

---

## 推奨ファイル構成
- `config.json` （.gitignore対象）
  - 設定例は `config.example.json` を参照
- `assets/` にロゴアイコン
- `docs/` にこの動作条件メモと追加ドキュメント

---

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

---

[← プロジェクトトップへ戻る](../README.md)

---

# System Requirements (Wallet Piccoro)

This tool is a **super-lightweight watcher** specialized only for  
**checking Chia wallet balance and deposit notifications**.  
It does not include transfer functions or key operations.

---

## Environment Requirements
- **OS**: Windows 10+, macOS 12+, Linux (Ubuntu 20.04+ recommended)  
- **Runtime**:  
  - Rust (rustup recommended)  
  - Node.js 18+ (for Tauri GUI)  
- **Build Environment**:  
  - Windows: Visual Studio Build Tools, Windows 10 SDK  
  - macOS: Xcode Command Line Tools  
  - Linux: `build-essential libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev`

---

## Chia Node Requirements
- Must have **Chia wallet RPC** running locally  
  - URL: `https://127.0.0.1:9256`  
- Certificate files (never include in repository):  
  - `private_wallet.crt`  
  - `private_wallet.key`  
- Target wallet ID (usually 1)

---

## Mail Notification Requirements
- SMTP-capable mail account (e.g. Gmail/Outlook)  
- 2FA enabled + **App Password** issued  
- Required info: `host`, `port`, `user`, `pass`, `to`

---

## Recommended File Structure
- `config.json` (excluded by `.gitignore`)  
  - See `config.example.json` for example configuration  
- `assets/` for logo icons  
- `docs/` for requirements and additional documentation

---

## Notes
- **Never include secret files in the repository**  
  - Certificates, keys, wallet DBs (`*.sqlite`) must remain external  
- This tool is **for deposit notifications only**.  
  No transfer or key management is performed.  
- Notifications will fail if the node is stopped,  
  but resume automatically when restarted.

---

[← Back to Project Top](../README.md)
