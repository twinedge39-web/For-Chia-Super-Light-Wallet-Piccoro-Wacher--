#!/usr/bin/env bash
set -euo pipefail

usage() {
  echo "Usage: $0 <wallet_ssl_dir> <out_p12> [p12_password]"
  echo "  <wallet_ssl_dir>: path that contains private_wallet.crt and private_wallet.key"
  echo "  <out_p12>:       output .p12 file path"
  echo "  [p12_password]:  optional; default is empty password"
}

if [[ $# -lt 2 || $# -gt 3 ]]; then
  usage; exit 1
fi

SSL_DIR="$1"
OUT_P12="$2"
P12_PASS="${3:-}"

CRT_PATH="${SSL_DIR%/}/private_wallet.crt"
KEY_PATH="${SSL_DIR%/}/private_wallet.key"

# Checks
command -v openssl >/dev/null 2>&1 || { echo "[ERR] openssl not found"; exit 1; }
[[ -f "$CRT_PATH" ]] || { echo "[ERR] not found: $CRT_PATH"; exit 1; }
[[ -f "$KEY_PATH" ]] || { echo "[ERR] not found: $KEY_PATH"; exit 1; }
if [[ -e "$OUT_P12" ]]; then
  read -r -p "[WARN] $OUT_P12 exists. Overwrite? [y/N] " ans
  [[ "$ans" == "y" || "$ans" == "Y" ]] || { echo "Abort."; exit 1; }
fi

# Build pkcs12
openssl pkcs12 -export \
  -in "$CRT_PATH" \
  -inkey "$KEY_PATH" \
  -out "$OUT_P12" \
  -password pass:"$P12_PASS"

echo "[OK] wrote $OUT_P12"
