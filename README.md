# Barangay Sports Fund

A transparent, community-driven sports fund where barangay donations and spending are tracked on-chain via Stellar Soroban.

## Problem
Barangay sports programs in the Philippines have no transparent funding mechanism. Residents donate money but have no way to verify how funds are spent. Coaches and athletes suffer from lack of equipment and support due to mismanaged or underfunded programs. This app brings full on-chain transparency to barangay-level sports funding.

## How It Works
1. Community members connect their Freighter wallet
2. Anyone can donate XLM to the shared sports fund
3. Admins propose spending (e.g. "Buy basketball uniforms — 1,200 XLM")
4. 2-of-3 admins must approve before any funds are released
5. All donations and spending are recorded on-chain — visible to everyone

## How It Uses Stellar
- **Soroban Smart Contract** — handles donations, spending proposals, and 2-of-3 multi-sig approvals
- **Stellar Testnet** — low-cost, fast transactions ideal for community-scale fund management
- **Freighter Wallet** — donor and admin authentication
- Why Stellar: sub-cent transaction fees make micro-donations viable; Soroban enables trustless multi-sig without a bank

## Track
Track 5 — Social Impact

## Tech Stack
- Framework: Next.js 16
- Stellar SDK: @stellar/stellar-sdk
- Smart Contract: Rust / Soroban
- Network: Testnet
- Wallet: Freighter

## Setup & Run

```bash
git clone https://github.com/mavstarr/Barangay-Sports-Fund.git
cd Barangay-Sports-Fund
cd web
npm install
npm run dev
```

Then open http://localhost:3000 in your browser.

## Network Details
- Network: Testnet
- RPC URL: https://soroban-testnet.stellar.org
- Contract IDs: TBD after deployment
- Asset issuers: N/A

## Team
- Mavs — @mavstarr

## License
MIT