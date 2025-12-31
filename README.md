# KCHNG

A Stellar blockchain token for kachi.ng - a community currency with native on-chain demurrage.

## Overview

KCHNG is a community currency built on Stellar with Soroban smart contracts. The core economic principle is **1 KCHNG = 1 local community meal**.

### Key Features

- **Native On-Chain Demurrage**: 2 KCHNG burned for every 7 days of account inactivity
- **Extensible**: Apps can implement additional demurrage logic around the base
- **Community Focused**: Designed for local community economies

## Project Structure

```
KCHNG/
├── packages/
│   ├── contracts/       # Soroban smart contracts (Rust)
│   ├── frontend/        # SvelteKit web app
│   └── shared/          # Shared types and utilities (TypeScript)
```

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
pnpm build
```

### Test

```bash
# Test all packages
pnpm test

# Test specific package
pnpm --filter contracts test
pnpm --filter frontend test
```

### Development

```bash
# Start frontend dev server
pnpm dev
```

## License

MIT
