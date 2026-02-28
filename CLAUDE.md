# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

KCHNG is a Stellar blockchain token project for kchng.org - a community currency with native on-chain demurrage. The core economic principle: **1 KCHNG = 1 local community meal**.

## Technology Stack

- **Blockchain**: Stellar Network with Soroban smart contracts
- **Smart Contract Language**: Rust (Soroban SDK v22)
- **Frontend**: SvelteKit (static adapter for PWA)
- **Package Manager**: pnpm workspaces

## Project Structure

```
KCHNG/
├── packages/
│   ├── contracts/       # Soroban smart contracts (Rust)
│   │   ├── src/
│   │   │   ├── lib.rs   # Main token contract
│   │   │   └── test.rs  # Contract tests
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

## Development Commands

### All Packages
```bash
make install       # Install all dependencies (pnpm install)
make build         # Build all packages
make test          # Run all tests
make lint          # Run linter
make clean         # Clean build artifacts
```

### Frontend (SvelteKit)
```bash
make dev           # Start dev server (localhost:5173)
pnpm --filter frontend dev
pnpm --filter frontend build
pnpm --filter frontend preview
```

### Contracts (Soroban/Rust)
```bash
make contract-build    # Build contract to WASM
make contract-test     # Run contract tests
cd packages/contracts && cargo build --release --target wasm32-unknown-unknown
cd packages/contracts && cargo test
cd packages/contracts && cargo clippy
```

### Shared Package
```bash
pnpm --filter shared build    # Build TypeScript
pnpm --filter shared dev      # Watch mode
```

## Key Concepts

### Demurrage Implementation
Demurrage is enforced **natively on-chain** through the Soroban smart contract:

1. **Base Demurrage**: 2 KCHNG burned for every 7 days of account inactivity
2. **Calculation**: Triggered on any transaction involving the account
3. **Extension**: Apps can register for additional demurrage via `register_app()`

The contract tracks `last_activity` timestamp per account and calculates inactive periods (7-day intervals).

### Contract Storage Keys
- `U256(0)` - Admin address
- `U256(1)` - Initialization flag
- `U256(2)` - Accounts map (Address → AccountData)
- `U256(3)` - Total supply
- `U256(4)` - Registered apps map (Address → AppDemurrageEntry)

### Shared Utilities
The `@kchng/shared` package contains TypeScript utilities that mirror the contract logic:
- `calculateBalanceWithDemurrage()` - Client-side demurrage calculation
- `calculateInactivePeriods()` - Period calculation from timestamps
- `getNetworkConfig()` - Network configurations (mainnet/testnet/standalone)

## Development Workflow

1. **Smart Contract Changes**: Modify `packages/contracts/src/lib.rs`, run `make contract-test`
2. **Shared Types**: Update `packages/shared/src/types.ts` first when changing contract interfaces
3. **Frontend**: Uses workspace protocol - changes to shared package auto-update

## Important Notes

- The contract uses `U256` for token amounts (big integer support)
- Timestamps are Unix seconds from `env.ledger().timestamp()`
- All account operations require authentication via `require_auth()`
- Demurrage is applied automatically before transfers in `transfer()` function
