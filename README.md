# KCHNG

A Stellar blockchain token for kchng.org - a community currency with native on-chain demurrage and time-based economics.

## Overview

KCHNG is a community currency built on Stellar with Soroban smart contracts, implementing the **Wörgl demurrage model** to incentivize circulation.

**Core Economic Equation**: `30 minutes verified work = 1000 KCHNG = 1 community meal`

Time (verified work) is the fundamental unit - not speculation. The demurrage system ensures tokens circulate by decaying inactive balances.

### Merchant Ecosystem

KCHNG is designed for community-focused meal providers:
- Community cafés in cooperative spaces
- Worker cooperatives
- Social enterprises prioritizing community impact
- Restaurants in eco-villages or intentional communities

**External subsidies** bridge the gap between KCHNG and real-world costs (rent, utilities), allowing merchants to accept KCHNG at face value. See the [Merchant Onboarding Guide](docs/MERCHANT_ONBOARDING.md) for details.

### Key Features

- **Time-Standard Economics**: Tokens issued for verified community work (care, agriculture, teaching)
- **Native On-Chain Demurrage**: Percentage-based decay using Wörgl model (default 12% annual)
- **Federated Trusts**: Communities set their own demurrage rates (5-15% bounds)
- **Work Verification**: Multi-verifier approval system with stake requirements
- **Grace Periods**: Oracle-activated hardship protections
- **Governance**: On-chain proposal voting for trust decisions

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
│   │   │   ├── lib.rs   # Main token contract (1752 lines)
│   │   │   └── test.rs  # Contract tests (508 lines)
│   │   └── Cargo.toml
│   ├── frontend/        # SvelteKit web app
│   │   ├── src/routes/
│   │   ├── src/lib/
│   │   └── package.json
│   └── shared/          # Shared types and utilities (TypeScript)
│       ├── src/
│       │   ├── types.ts       # Core type definitions
│       │   ├── demurrage.ts   # Demurrage calculation utilities
│       │   └── networks.ts    # Network configurations
│       └── package.json
├── Makefile
├── pnpm-workspace.yaml
└── package.json
```

---

## Development

### Prerequisites

- Node.js >= 20
- pnpm >= 9
- Rust (for contract development)
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

# Test specific package
pnpm --filter contracts test
pnpm --filter frontend test
```

### Development

```bash
# Start frontend dev server (localhost:5173)
make dev
# or
pnpm dev
```

---

## Deployment Status

### Testnet ✅ Deployed

| Property | Value |
|----------|-------|
| **Contract ID** | `CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX` |
| **Explorer** | [stellar.expert](https://stellar.expert/explorer/testnet/contract/CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX) |
| **WASM Size** | 55,956 bytes (optimized) |
| **Public Methods** | 39 |
| **Unit Tests** | 15/15 passing |

### Mainnet ✅ Deployed

| Property | Value |
|----------|-------|
| **Contract ID** | `CDMKVYIU6KAATZXLLFT6KTJCKXCWP3HPYNMA3HMEUUWYVCM5DJVZ5AQS` |
| **Explorer** | [stellar.expert](https://stellar.expert/explorer/public/contract/CDMKVYIU6KAATZXLLFT6KTJCKXCWP3HPYNMA3HMEUUWYVCM5DJVZ5AQS) |
| **Deployed** | 2026-02-11 |

Pending items:
- Frontend completion
- Testing of advanced features (grace periods, governance, swaps)

---

## Contract Features

### Phase 1: Core Token ✅
- Token transfers with balance checking
- Admin-only mint function
- Percentage-based demurrage (Wörgl model)
- Time-based activity tracking

### Phase 2: Trust System ✅
- Register trust with custom rates (5-15%)
- Join/leave trust functionality
- Governor-managed membership

### Phase 3: Work Verification ✅
- Submit work claims with evidence (IPFS)
- Multi-verifier assignment (min 2)
- Approval/rejection voting
- Token minting: 30 min = 1 KCHNG (base)

### Phase 4: Reputation System ✅
- Score tracking (0-1000), starts at 500
- Updates on approval (+5) / rejection (+10)
- **Extensible design** - data available for third-party apps
- Queried via `get_verifier(address).reputation_score`

### Phase 5: Grace Periods ⚠️
- Oracle registration (500K stake)
- Emergency/Illness/Community types
- Untested in practice

### Phase 6: Cross-Trust Exchange ⚠️
- Rate calculation: `(1 - r_src) / (1 - r_dst)`
- Swap function complete, untested

### Phase 7: Governance ⚠️
- Proposal creation (4 types)
- Voting with quorum requirements
- Untested at scale

---

## Documentation

| Document | Description |
|----------|-------------|
| [PRD](docs/PRD.md) | Product Requirements Document |
| [Merchant Onboarding](docs/MERCHANT_ONBOARDING.md) | Guide for cafés and meal providers |
| [Third-Party Integration Guide](docs/THIRD_PARTY_INTEGRATION.md) | Build apps on KCHNG |
| [DEPLOYMENT_REPORT](docs/DEPLOYMENT_REPORT.md) | Testnet deployment details |
| [time-standard-token-design](docs/time-standard-token-design.md) | Economic model deep-dive |
| [contract-improvement-needed](docs/2026-01-02_contract-improvement-needed.md) | Known issues analysis |

---

## License

MIT
