param(
  [Parameter(Mandatory=$true)] [string]$WalletSslDir,
  [Parameter(Mandatory=$true)] [string]$OutP12,
  [Parameter(Mandatory=$false)] [string]$P12Password = ""
)

$crt = Join-Path $WalletSslDir "private_wallet.crt"
$key = Join-Path $WalletSslDir "private_wallet.key"

if (-not (Get-Command openssl -ErrorAction SilentlyContinue)) {
  Write-Error "OpenSSL not found in PATH"; exit 1
}
if (-not (Test-Path $crt)) { Write-Error "Not found: $crt"; exit 1 }
if (-not (Test-Path $key)) { Write-Error "Not found: $key"; exit 1 }

if (Test-Path $OutP12) {
  $ans = Read-Host "[WARN] $OutP12 exists. Overwrite? [y/N]"
  if ($ans -ne "y" -and $ans -ne "Y") { Write-Host "Abort."; exit 1 }
}

# Create pkcs12
& openssl pkcs12 -export -in $crt -inkey $key -out $OutP12 -password "pass:$P12Password"
if ($LASTEXITCODE -ne 0) { Write-Error "OpenSSL failed"; exit 1 }

Write-Host "[OK] wrote $OutP12"
