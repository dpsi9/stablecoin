
# 🪙 Solana Programmable Money (Stablecoin Lending Protocol)

This project implements a **collateralized stablecoin protocol** on **Solana**, inspired by MakerDAO and other DeFi lending models.  
Users can deposit SOL as collateral and mint a **protocol-native stablecoin**, with on-chain **health factor** and **liquidation** logic.

---

## 📌 Features

- ✅ Deposit SOL as collateral  
- ✅ Mint stablecoins against collateral  
- ✅ Redeem collateral by burning stablecoins  
- ✅ Liquidate undercollateralized positions  
- ✅ On-chain health factor enforcement  
- ✅ Configurable risk parameters (liquidation threshold, bonus, etc.)  
- ✅ Mocked price feed using Pyth PriceUpdateV2  
- ✅ Full PDA-driven state management  
- ✅ Built with **Rust + Anchor Framework**

---

## 🏛️ Program Architecture

### 📂 Key Accounts:

| Account               | Purpose                                           |
|---------------------- |-------------------------------------------------- |
| **Config**            | Stores global risk parameters & mint PDA         |
| **Collateral**        | Per-user collateral state (SOL balance, debt)    |
| **SOL Account (PDA)** | PDA holding user’s deposited SOL (collateral)     |
| **Mint Account**      | PDA minting the protocol stablecoin              |
| **Token Account**     | User’s ATA for receiving stablecoins             |

---

## 🧱 Instructions Implemented:

| Instruction                            | Purpose                          |
|--------------------------------------  |-------------------------------- |
| `initialize_config`                   | Set up protocol config & mint   |
| `update_config`                       | Authority can update risk params|
| `deposit_collateral_and_mint_tokens`  | Deposit SOL + Mint stablecoins  |
| `redeem_collateral_and_burn_tokens`   | Burn stablecoins + Redeem SOL   |
| `liquidate`                           | Liquidate unhealthy positions   |

---

## 🧪 Testing (In Progress / Optional)

- ✅ Local testing with mocked Pyth oracles  
- ✅ Health factor & liquidation edge cases  
- ✅ Token balances & mint authority checks  

You can run:

```bash
anchor test --skip-build
````

(Make sure LiteSVM or local validator is set up)

---

## ⚙️ Tech Stack

* Rust
* Anchor
* Solana Program Library (SPL Token2022)
* Pyth Oracles (Mocked for testing)
* Anchor LiteSVM (for fast in-memory testing)

---

## 🚨 Disclaimer

> This is a learning-focused project built for educational purposes only.
> It is **NOT audited**, **NOT production-ready**, and has **incomplete safety checks**.
> **Do NOT deploy this on Solana Mainnet.**

---

## 📂 Project Structure



```
programs/
└── stablecoin/
    └── src/
        ├── instructions/
        │   ├── admin/
        │   │   ├── initialize_config.rs
        │   │   ├── update_config.rs
        │   │   └── mod.rs
        │   ├── deposit/
        │   │   ├── deposit_collateral_and_mint_tokens.rs
        │   │   ├── utils.rs
        │   │   └── mod.rs
        │   ├── withdraw/
        │   │   ├── redeem_collateral_and_burn_tokens.rs
        │   │   ├── liquidate.rs
        │   │   ├── utils.rs
        │   │   └── mod.rs
        │   └── mod.rs
        ├── state.rs
        ├── constants.rs
        ├── error.rs
        ├── lib.rs
        └── mod.rs
```

</details>

---

## ✅ Next Steps (Learning Roadmap)

* Improve interest rate & debt modeling
* Add real Pyth testnet feeds
* Build a frontend (Next.js / React) for UI
* Explore permissionless minting models

---

## 📝 License

MIT License (For educational use only)

