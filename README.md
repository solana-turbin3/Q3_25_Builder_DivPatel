#  Q3 Turbin3 Builder Program — Solana + Anchor Track

This repository contains all projects completed during the **Q3 Turbin3 Builder Program**, focused on building real on-chain programs on Solana using **Rust + Anchor**.  
Each subproject targets a core area of Solana development — PDAs, token management, escrow logic, AMMs, staking, and vault security.

---

##  Learning Outcomes (High-Level)

Throughout these projects, the focus was on mastering:

- **Anchor framework**: accounts, instruction handlers, CPI, macros
- **Program Derived Addresses (PDAs)**: deterministic authorities without private keys
- **Token Program / Token-2022**: minting, transferring, vaulting SPL tokens
- **Vault mechanics**: securing user balances by delegating authority to PDAs
- **Escrow architecture**: locking funds until conditions are met
- **AMM design**: liquidity pools, swap pricing, invariant maintenance
- **Account introspection**: reading on-chain account state during execution
- **End-to-end dApp building**: on-chain program + client + testing workflow

---

##  Repository Structure

| Folder / Program | What it Demonstrates |
|------------------|----------------------|
| `dice-game/` | Minimal Anchor program; randomness + PDA authority + state storage. |
| `NFT_MARKET_PLACE/` | NFT listing & buying; tests include royalties & marketplace fees. |
| `anchor-amm/` | Automated Market Maker; liquidity adding/removing + token swapping. |
| `anchor-escrow/` | Escrow contract; lock tokens until both parties sign the trade. |
| `anchor-vault/` | Token vault system; PDA holds custody of user funds securely. |
| `nft-staking/` | Stake NFTs & earn rewards over time; time-based emissions logic. |
| `prerequisite/` | Test  |
| `solana-starter/` | Boilerplate: project layout, scripts, workflows. |
| `Credetia/` | Final Capstone Project |

---

##  Key Concepts Learned (Simple Mental Models)

| Concept | Explanation (Short & Practical) |
|--------|----------------------------------|
| **PDA (Program Derived Address)** | A signer controlled by the program, not a private key. Used for vaults, escrow, AMMs. |
| **Vault Security** | Instead of sending tokens to a wallet, tokens are owned by the PDA → prevents rugging or private-key compromise. |
| **Escrow** | Program enforces both sides of a trade before releasing tokens. |
| **AMM / Swap** | Users deposit both tokens → pool manages reserves → invariant formula determines price. |
| **Account introspection** | Program inspects account data during runtime to validate ownership, token balance etc. |

---

##  Testing Philosophy

Each project contains complete tests:

- Local validator launch (`anchor test`)
- PDAs + auth checks using `assert!`
- Simulated real flows: mint → deposit → trade → withdraw

Testing was used to *prove correctness*, not just compile code.

---

## Capstone — Credentia (P2P NFT Lending Protocol)

The final stage was integrating everything learned across all projects — PDAs, escrow, token vaults, account introspection, AMM logic, and secure account authority — into a **complete, production-grade on-chain program**.

**Credentia** is a peer-to-peer NFT lending protocol built on Solana.  
It enables NFT holders to unlock liquidity *without selling their NFTs.* Borrowers lock their NFT as collateral into a **PDA-secured vault**, and lenders provide a loan in SOL (or USDC in V2). If the borrower repays before the due date, they receive their NFT back; if not, ownership transfers to the lender.

**Key Capstone Outcomes**
- Full P2P lending flow using Anchor
- PDA-powered vault custody for NFT collateral
- Escrow-based loan settlement (repay → return NFT, default → lender receives NFT)
- Loan parameters are fully customizable (amount, duration, interest)
- Supports long-tail NFT collections — no oracle dependency


---

##  Tech Stack

- **Rust**
- **Anchor Framework**
- **SPL Token / Token-2022**
- **TypeScript** (tests & client)

---

