# KCHNG

A Stellar blockchain token for kchng.org — a time- and labour-backed community currency with native on-chain demurrage and a verifier governance ecosystem.

## Overview

KCHNG is a community currency built on Stellar with Soroban smart contracts, implementing the **Wörgl demurrage model** to incentivize circulation.

**Core Economic Equation**: `30 minutes verified work = 1000 KCHNG = 1 community meal`

Time (verified work) is the fundamental unit — not speculation. The demurrage system ensures tokens circulate by decaying inactive balances, with revenue split between verifier compensation and the protocol.

### Merchant Ecosystem

KCHNG is designed for community-focused meal providers:
- Community cafés in cooperative spaces
- Worker cooperatives
- Social enterprises prioritizing community impact
- Restaurants in eco-villages or intentional communities

**External subsidies** bridge the gap between KCHNG and real-world costs (rent, utilities), allowing merchants to accept KCHNG at face value. See the [Merchant Onboarding Guide](docs/MERCHANT_ONBOARDING.md) for details.

### Key Features

- **Time-Standard Economics**: Tokens issued for verified community work (care, agriculture, teaching)
- **Native On-Chain Demurrage**: Percentage-based decay using Wörgl model (default 12% annual), with split distribution to verifiers
- **Verifier Ecosystem**: On-chain verifier elections, governor stakes, and compensation from demurrage revenue
- **Federated Communities**: Communities set their own demurrage rates (5–15% bounds)
- **Genesis Trust**: Auto-created default community for new members
- **Work Verification**: Multi-verifier approval system with stake requirements
- **Grace Periods**: Oracle-activated hardship protections
- **Governance**: On-chain proposal voting for community decisions
- **i18n**: Multi-language support (EN, ES, RU, ZH, DE, AR)

### Work Types & Multipliers

| Type | Multiplier | Examples |
|------|------------|----------|
| Basic Care/Agriculture | 1.0× | Elderly care, farming |
| Skilled Care/Heavy Labor | 1.3× | Nursing, construction |
| Training/Teaching | 1.5× | Skills transfer |
| Emergency Care | 2.0× | Crisis response |

---

## Project Structure

```
KCHNG/
├── packages/
│   ├── contracts/       # Soroban smart contracts (Rust)
│   │   ├── src/
│   │   │   ├── lib.rs   # Main token contract (~4700 lines)
│   │   │   └── test.rs  # Contract tests (~4300 lines)
│   │   └── Cargo.toml   # Soroban SDK v25.3, Rust edition 2024
│   ├── frontend/        # SvelteKit web app (PWA)
│   │   ├── src/routes/  # Page routes
│   │   ├── src/lib/     # Shared components, i18n, stores
│   │   └── package.json
│   └── shared/          # Shared types and utilities (TypeScript)
│       ├── src/
│       │   ├── types.ts       # Core type definitions
│       │   ├── demurrage.ts   # Demurrage calculation utilities
│       │   └── networks.ts    # Network configurations
│       └── package.json
├── docs/                # Documentation (PRD, guides, reports)
├── Makefile
├── pnpm-workspace.yaml
└── package.json
```

---

## Development

### Prerequisites

- Node.js >= 20
- pnpm >= 9
- Rust >= 1.85 (for contract development)
- Soroban CLI

### Installation

```bash
pnpm install
```

### Build

```bash
make build
# or
pnpm build
```

### Test

```bash
# Test all packages
make test
# or
pnpm test

# Test contracts only
cd packages/contracts && cargo test

# Test specific package
pnpm --filter contracts test
pnpm --filter frontend test
```

### Development

```bash
# Start frontend dev server (localhost:5173)
make dev
# or
pnpm --filter frontend dev
```

---

## Deployment Status

> **Source of Truth:** Contract IDs are maintained in [`packages/shared/src/networks.ts`](packages/shared/src/networks.ts)

### Current Deployments (v6 — Verifier Ecosystem)

| Network | Version | Contract ID | Status |
|---------|---------|-------------|--------|
| **Mainnet** | v6 | `CCGG5P7HU4TQNYOW6DK37JIDPZCAQ5ECBDENHEOULG72BZCD4BR7MKX6` | Active |
| **Testnet** | v6 | `CCIBWKAZYESALQMHAZOEH7FOMDMPPFLD74UGZMFEHIXUSFJWB2BDCGVQ` | Active |

See [`CHANGELOG.md`](CHANGELOG.md) for full deployment history and previous contract addresses.

### Contract Features by Phase

| Phase | Status | Description |
|-------|--------|-------------|
| Core Token | ✅ | Transfers, minting, demurrage |
| Trust/Community System | ✅ | Register/join/leave communities |
| Work Verification | ✅ | Multi-verifier claims |
| Reputation | ✅ | Role-based scoring (0–1000) |
| Grace Periods | ✅ | Emergency, illness, community |
| Cross-Community Exchange | ✅ | Rate-based swaps between communities |
| Governance | ✅ | Proposals, voting, quorum |
| Migration | ✅ | Contract upgrade support (v5+) |
| Verifier Ecosystem | ✅ | Split demurrage, elections, compensation (v6) |

---

## Contract Architecture

### Demurrage Split (v6)

Demurrage revenue is no longer purely burned. On each transfer, the split is calculated and distributed:

- **Verifier fund**: Compensation pool for the community's verifiers
- **Genesis pool**: Protocol-level pool from the genesis trust
- **Burned**: Remaining portion is permanently removed from supply

### Verifier Ecosystem

The v6 contract introduces a full verifier governance layer:

- **Verifier registration**: Verifiers register with a community and stake collateral
- **Elections**: Community members can propose and vote on verifier elections
- **Compensation**: Verifiers claim compensation from demurrage revenue via `claim_verifier_compensation()`
- **Governor stakes**: Community governors stake collateral when registering a community
- **Genesis trust**: An auto-created default community that new members join

### Key Public Functions

| Category | Functions |
|----------|-----------|
| **Token** | `transfer`, `mint`, `balance`, `total_supply` |
| **Communities** | `register_trust`, `join_trust`, `leave_trust`, `join_genesis_trust` |
| **Verifiers** | `register_verifier`, `unregister_verifier`, `get_verifier` |
| **Elections** | `propose_verifier_election`, `vote_verifier_election`, `finalize_verifier_election` |
| **Funds** | `get_verifier_fund`, `claim_verifier_compensation`, `get_genesis_pool` |
| **Work** | `submit_work_claim`, `approve_work_claim`, `reject_work_claim` |
| **Grace** | `register_oracle`, `activate_grace_period`, `extend_grace_period`, `report_grace_abuse` |
| **Exchange** | `cross_trust_swap`, `calculate_exchange_rate`, `simulate_cross_trust_swap` |
| **Governance** | `create_proposal`, `vote_on_proposal`, `process_proposal`, `implement_proposal` |
| **Reputation** | `get_reputation`, `update_role_score`, `set_probation`, `is_on_probation` |
| **Admin** | `migrate_data`, `designate_successor`, `step_down` |

### Contract Storage Keys

| Key | Data |
|-----|------|
| `U256(0)` | Admin address |
| `U256(1)` | Initialization flag |
| `U256(2)` | Accounts map (Address → AccountData) |
| `U256(3)` | Total supply |
| `U256(4)` | Registered communities map |
| `U256(9)` | Genesis trust ID |
| `U256(10)` | Genesis pool data |
| `U256(1000)` | Verifier funds map (per community) |
| `U256(1100)` | Verifier elections map |

---

## Frontend Pages

| Route | Description |
|-------|-------------|
| `/` | Homepage |
| `/about` | Protocol details and contract info |
| `/dashboard` | Role-based dashboard (verifier, governor, member) |
| `/communities` | Browse and join communities |
| `/work` | Submit and verify work claims |
| `/governance` | Proposals and voting |
| `/communicate` | Decentralized chat (Gun.js) |
| `/terms` | Terms of use |

---

## Frontend UI Coverage

The contract exposes ~65 public functions. The `KchngClient` library wraps most of them, but not all are wired into Svelte pages.

### Implemented in UI

| Function | Page |
|----------|------|
| `balance` / `get_account` | Dashboard, wallet store |
| `total_supply` | About page |
| `register_trust` | Communities |
| `join_trust` | Communities |
| `get_trust_info` / `get_all_trusts` | Communities, Dashboard |
| `register_verifier` | Work |
| `get_verifier` / `get_verifier_pending_claims` | Work, Dashboard |
| `submit_work_claim` / `approve_work_claim` / `reject_work_claim` | Work |
| `get_work_claim` | Work |
| `register_oracle` / `get_oracle` | Communities |
| `is_in_grace_period` / `get_grace_period` | Dashboard |
| `cross_trust_swap` / `calculate_exchange_rate` / `simulate_cross_trust_swap` | Dashboard |
| `create_proposal` / `vote_on_proposal` / `process_proposal` / `implement_proposal` | Governance |
| `get_proposal` / `get_all_proposals` | Governance |
| `get_reputation` / `is_on_probation` | Dashboard |
| `get_verifier_fund` | Dashboard |
| `designate_successor` / `step_down` | Communities |
| `get_community_info` / `get_account_data` | Communities, Dashboard |

### Not Implemented in UI (~28 functions)

| Function | Gap |
|----------|-----|
| `transfer` | No send/transfer UI |
| `mint` | Admin-only, no admin panel |
| `leave_trust` | No leave-community flow |
| `join_genesis_trust` / `get_genesis_trust_id` / `get_genesis_pool` | No genesis trust UI |
| `propose_verifier_election` / `vote_verifier_election` / `finalize_verifier_election` | Full election lifecycle missing |
| `get_election` / `get_trust_elections` | No election viewing |
| `claim_verifier_compensation` | No compensation claiming |
| `activate_grace_period` / `extend_grace_period` | Oracle-only, no UI |
| `report_grace_abuse` | No abuse reporting |
| `unregister_verifier` / `unregister_oracle` | No unregister flows |
| `get_account_demurrage_rate` | No per-account demurrage display |
| `update_role_score` / `set_probation` | Internal/contract-called only |
| `get_reputation_data` / `get_all_reputations` / `can_multi_trust_verify` | Extended reputation not surfaced |
| `migrate_data` / `get_migration_status` | No migration tool |
| `get_admin` / `get_protocol_version` / `get_total_supply_raw` | No admin dashboard |

---

## Documentation

| Document | Description |
|----------|-------------|
| [PRD](docs/PRD.md) | Product Requirements Document |
| [Merchant Onboarding](docs/MERCHANT_ONBOARDING.md) | Guide for cafés and meal providers |
| [Third-Party Integration Guide](docs/THIRD_PARTY_INTEGRATION.md) | Build apps on KCHNG |
| [Time-Standard Token Design](docs/time-standard-token-design.md) | Economic model deep-dive |
| [Audit Research](docs/AUDIT_RESEARCH.md) | Stellar/Soroban security review notes |

---

## License

MIT — see [LICENSE](LICENSE) for details, including blockchain-specific disclaimers.
