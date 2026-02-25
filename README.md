## Status: Frozen

The author does not currently use this tool in practice.
Development is halted indefinitely.

This repository remains as an architectural reference.

It may be resumed if a clear practical need arises.

---

# For Chia Super Light Wallet Piccoro (Watcher)

## Purpose
A **super-light Chia wallet monitor** designed for safety and simplicity.

- **Check balance only**  
- **No transfer function / no key handling**  
- **Deposit detection** → Notifications via **Webhook (recommended: GAS)** and OS notifications  
- **Retrieve from authorized local wallets**  
- Avoid overloading with features → keep it **light, safe, and one-person maintainable**

---

## Status: Frozen

The author does not currently use this tool in practice.
Development is halted indefinitely.

This repository remains as an architectural reference.

## Planned Features
- [x] Balance check with Webhook notification
- [ ] Balance display in system tray
- [ ] OS notifications for new transactions
- [ ] Minimal settings GUI (webhook, wallet_id, interval, etc.)
- [ ] Autostart on OS login

---

## Philosophy
This project is built with the idea that **less is safer**:  
Too many features lead to complexity, dependency, and risk.  
By keeping the tool small and focused, it stays reliable and sustainable for a single maintainer.

> *“Check balance, notify deposit — nothing more.”*

---

## Project Status
🟢 M1 complete (Balance check + Webhook notification)  
⚪ Next: Tray + OS notification  
⚪ Future: Minimal settings UI

---

## Security Notice
This project is designed to run on the same machine as a Chia full node or wallet.  
**Do not commit or push any sensitive Chia files** such as private keys, SSL certificates, or database files.  

All secrets (certificates, wallet DBs, keys) must stay outside of this repository.  
This repository is for the **wallet monitor code only**.

---

## License
Apache-2.0

![Wallet Piccoro Logo](./assets/OriginPiccoro_icon.png
)

## Documentation
詳細な動作条件やセットアップ手順は [docs/README.md](./docs/README.md) を参照。

