# 動作条件メモ (Wallet Piccoro)

このツールは **Chiaウォレットの残高チェックと入金通知** のみに特化した
超軽量ウォッチャーです。送金機能や鍵操作は一切含まれていません。

---

## 環境要件
- **OS**: Windows 10+, macOS 12+, Linux (Ubuntu 20.04+ 推奨)
- **ランタイム**
  - Rust (rustup 推奨)
  - Node.js 18 以上（Tauri GUI用）
- **ビルド環境**
  - Windows: Visual Studio Build Tools, Windows 10 SDK
  - macOS: Xcode Command Line Tools
  - Linux: `build-essential libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev`

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

## 注意
- **秘密ファイルは絶対にリポジトリに含めないこと**
  - 証明書、鍵、ウォレットDB (`*.sqlite`) は外部に置く
- 本ツールは **入金通知専用**。送金機能や鍵管理は行わない。
- ノード停止中は通知できないが、自動復帰時に動作再開する。

---
