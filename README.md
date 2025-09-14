# For Chia Super Light Wallet Piccoro (READ ONLY)

## Purpose
A **super-light Chia wallet monitor** designed for safety and simplicity.

- **Check balance only**  
- **No transfer function / no key handling**  
- **Deposit detection** → Notifications via **email** (preferred) and OS notifications  
- **Retrieve from authorized local wallets**  
- Avoid overloading with features → keep it **light, safe, and one-person maintainable**

---

## Planned Features
- [ ] Balance display in system tray
- [ ] Deposit detection with email notification
- [ ] OS notifications for new transactions
- [ ] Minimal settings GUI (SMTP, wallet_id, interval, etc.)
- [ ] Autostart on OS login

---

## Philosophy
This project is built with the idea that **less is safer**:  
Too many features lead to complexity, dependency, and risk.  
By keeping the tool small and focused, it stays reliable and sustainable for a single maintainer.

> *“Check balance, notify deposit — nothing more.”*

---

## Project Status
🟢 Planning phase (design + setup)  
⚪ Implementation (M1: Balance check + Email notification)  
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

