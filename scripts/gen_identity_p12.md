## gen_identity_p12 の使い方

Wallet Piccoro が Chia Wallet RPC にアクセスするための  
PKCS#12 (`.p12`) 証明書ファイルを生成するスクリプトです。

対応スクリプト:
- `gen_identity_p12.sh` (Linux / macOS / WSL 用)
- `gen_identity_p12.ps1` (Windows PowerShell 用)

---

## 実行例

### Linux / macOS
```bash
chmod +x scripts/gen_identity_p12.sh
./scripts/gen_identity_p12.sh ~/.chia/mainnet/config/ssl/wallet wallet_identity.p12
```

```bash
.\gen_identity_p12.ps1 -WalletSslDir "$env:USERPROFILE\.chia\mainnet\config\ssl\wallet" -OutP12 ".\wallet_identity.p12"
```

## 注意点

- openssl がインストールされている必要があります
  - macOS: brew install openssl
  - Ubuntu: sudo apt install openssl
  - Windows: Git Bash / WSL / OpenSSL for Windows を導入
- 出力される wallet_identity.p12 は 秘密情報です
  - 公開リポジトリやクラウドストレージにアップロードしないでください
- パスワードは既定で空です
  - パスワードを付けたい場合はスクリプト引数で指定し、config.json の identity_p12_password も合わせて設定してください
- Chia の RPC 証明書 (private_wallet.crt / private_wallet.key) を正しいディレクトリから指定してください



