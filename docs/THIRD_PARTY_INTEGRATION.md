# KCHNG Third-Party Integration Guide

This guide explains how third-party applications can integrate with the KCHNG contract on Stellar.

---

## Architecture Overview

**Single-Contract Design**: KCHNG uses a **single contract deployment per network** (Stellar), with **multiple independent trusts** operating within it.

```
┌─────────────────────────────────────────────────────────────┐
│              KCHNG Contract (Testnet)                       │
│         CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX │
├─────────────────────────────────────────────────────────────┤
│  Trust A (Elder Care)    │  Trust B (Rural Health)          │
│  Trust C (Education)     │  Trust D (Emergency)             │
│  ...                    │  ...                             │
└─────────────────────────────────────────────────────────────┘
```

**Benefits of single-contract architecture**:
- ✅ Shared liquidity across all communities
- ✅ Built-in cross-trust exchange
- ✅ Unified governance possible
- ✅ Single codebase to maintain

Third-party apps interact with this **single contract** to access all trusts and their data.

---

## Overview

The KCHNG contract provides an **extensible platform** for community currency applications. Third-party apps can:

1. **Read on-chain data** (reputation, balances, claims, proposals)
2. **Build custom logic** on top of the base contract
3. **Implement additional features** (UI, analytics, incentives)
4. **Create specialized experiences** (mobile apps, dashboards, tools)

---

## Installation

The `@kchng/shared` package provides TypeScript types, network configurations, and utility functions for integrating with KCHNG.

### Install from Git (Current Method)

Since `@kchng/shared` is currently a private package, install directly from the repository:

```bash
# Using pnpm
pnpm add git+https://github.com/sammensuriet/KCHNG.git#main --filter=@kchng/shared

# Or from a specific tag/commit
pnpm add git+https://github.com/sammensuriet/KCHNG.git#v1.0.0

# Or using GitHub shorthand
pnpm add github:sammensuriet/KCHNG#main --filter=@kchng/shared
```

### Install Stellar SDK

```bash
pnpm add @stellar/stellar-sdk
```

### Future: npm Registry (When Published)

Once published to npm, installation will be simpler:

```bash
pnpm add @kchng/shared @stellar/stellar-sdk
```

---

## Integration Methods

### Method 1: TypeScript/JavaScript (Recommended)

**Example: Query Verifier Reputation**
```typescript
import { Contract } from "@stellar/stellar-sdk";
import { getNetworkConfig } from "@kchng/shared";

const network = getNetworkConfig("testnet");
const contractId = network.contractId;

const contract = new Contract({
  contractId,
  networkPassphrase: network.networkPassphrase,
  rpcUrl: network.rpcUrl,
});

// Query verifier reputation
const verifierAddress = "GD...XYZ";
const result = await contract.get_verifier({
  verifier: new Address(verifierAddress),
});

console.log(`Reputation: ${result.reputation_score}/1000`);
console.log(`Verified: ${result.verified_claims}`);
console.log(`Rejected: ${result.rejected_claims}`);
```

**Example: Calculate Account Balance with Demurrage**
```typescript
import { calculateBalanceWithDemurrage } from "@kchng/shared/demurrage";

// Get raw balance from contract
const accountData = await contract.balance({
  account: new Address(userAddress),
});

// Calculate current balance after demurrage
const currentBalance = calculateBalanceWithDemurrage({
  balance: accountData.balance,
  last_activity: accountData.last_activity,
  annual_rate_bps: 1200, // 12% annual
  current_timestamp: Math.floor(Date.now() / 1000),
});

console.log(`Current balance: ${currentBalance} KCHNG`);
```

### Method 2: Python / Other Languages

For non-JavaScript languages, use the Stellar SDK directly via RPC calls. No `@kchng/shared` equivalent is available - you'll interact with the contract directly.

**Installation**:
```bash
pip install stellar-sdk
```

**Example: Query Contract Data**
```python
from stellar_sdk import Server, Network
import json

# Connect to testnet
server = Server("https://horizon-testnet.stellar.org")
contract_id = "CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX"

# For Soroban contract calls, you'd use the RPC endpoint directly
# or use a wrapper library. Here's a basic example:

import requests

rpc_url = "https://soroban-testnet.stellar.org"

def get_verifier_reputation(verifier_address: str) -> dict:
    """Query verifier data from KCHNG contract"""
    payload = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "simulateTransaction",
        "params": [
            {
                "transaction": "...", # Build Soroban transaction
                "ledger": "latest"
            }
        ]
    }
    # Note: Full implementation requires proper transaction building
    # This is a simplified example
    response = requests.post(rpc_url, json=payload)
    return response.json()
```

### Method 3: Direct RPC (Any Language)

Query the Soroban RPC endpoint directly using HTTP requests.

**Endpoint**: `https://soroban-testnet.stellar.org`

**Available Read Methods**:
```
balance(account) -> AccountData
get_verifier(verifier) -> VerifierData
get_trust_info(trust_id) -> TrustData
get_work_claim(claim_id) -> WorkClaim
get_all_trusts() -> Vec<TrustData>
get_proposal(proposal_id) -> Proposal
get_account_demurrage_rate(account) -> (u32, u64)
```

---

## Use Cases & Examples

### Use Case 1: Reputation-Based Verifier Selection

Third-party apps can implement smart verifier selection based on reputation data.

```typescript
import type { VerifierData } from "@kchng/shared";

interface VerifierWithMeta extends VerifierData {
  address: string;
  trustScore: number;
}

// Fetch all verifiers in a trust
async function getVerifiersByTrust(trustId: string): Promise<VerifierWithMeta[]> {
  const verifiers: VerifierWithMeta[] = [];

  // Query trust members
  const trustData = await contract.get_trust_info({ trust_id: new Address(trustId) });

  // For each member, check if they're a verifier
  for (const memberId of trustData.members) {
    try {
      const verifierData = await contract.get_verifier({
        verifier: new Address(memberId),
      });

      verifiers.push({
        ...verifierData,
        address: memberId,
        trustScore: calculateCustomTrustScore(verifierData),
      });
    } catch {
      // Not a verifier
    }
  }

  return verifiers.sort((a, b) => b.trustScore - a.trustScore);
}

// Custom scoring algorithm
function calculateCustomTrustScore(data: VerifierData): number {
  let score = data.reputation_score;

  // Bonus for experience
  const totalClaims = data.verified_claims + data.rejected_claims;
  if (totalClaims > 100) score += 50;
  else if (totalClaims > 50) score += 25;

  // Bonus for balanced judgment
  if (totalClaims > 10) {
    const rejectionRate = data.rejected_claims / totalClaims;
    if (rejectionRate >= 0.05 && rejectionRate <= 0.15) {
      score += 30; // Good fraud detection without hostility
    }
  }

  return Math.min(1000, score);
}
```

### Use Case 2: Mobile App with Wallet Integration

```typescript
// React Native example
import { Contract, Address, xdr } from "@stellar/stellar-sdk";
import { getNetworkConfig } from "@kchng/shared";

class KCHNGClient {
  private contract: Contract;
  private network: NetworkConfig;

  constructor() {
    this.network = getNetworkConfig("testnet");
    this.contract = new Contract({
      contractId: this.network.contractId,
      networkPassphrase: this.network.networkPassphrase,
      rpcUrl: this.network.rpcUrl,
    });
  }

  // Get user's current balance (with demurrage calculated)
  async getBalance(userAddress: string): Promise<{
    raw: bigint;
    current: bigint;
    demurrage: bigint;
  }> {
    const accountData = await this.contract.balance({
      account: new Address(userAddress),
    });

    // Calculate demurrage on-chain
    const balance = await this.contract.balance_with_demurrage({
      account: new Address(userAddress),
    });

    return {
      raw: accountData.balance,
      current: balance,
      demurrage: accountData.balance - balance,
    };
  }

  // Get user's reputation (if verifier)
  async getReputation(userAddress: string): Promise<{
    score: number;
    tier: string;
    verified: number;
    rejected: number;
  } | null> {
    try {
      const verifierData = await this.contract.get_verifier({
        verifier: new Address(userAddress),
      });

      return {
        score: verifierData.reputation_score,
        tier: this.getReputationTier(verifierData.reputation_score),
        verified: verifierData.verified_claims,
        rejected: verifierData.rejected_claims,
      };
    } catch {
      return null; // Not a verifier
    }
  }

  // Submit work claim
  async submitWorkClaim(params: {
    worker: string;
    workType: number;
    minutes: number;
    evidenceHash: string;
  }): Promise<string> {
    // Build and sign transaction
    // Returns claim ID
    const tx = await this.contract.submit_work_claim({
      worker: new Address(params.worker),
      work_type: params.workType,
      minutes_worked: params.minutes,
      evidence_hash: params.evidenceHash,
      gps_lat: null,
      gps_lon: null,
    });

    // Submit to network
    // ... transaction signing and submission
    return "claim_id";
  }

  private getReputationTier(score: number): string {
    if (score >= 900) return "Legendary";
    if (score >= 750) return "Trusted";
    if (score >= 600) return "Established";
    if (score >= 500) return "Neutral";
    return "New";
  }
}
```

### Use Case 3: Analytics Dashboard

```typescript
// Aggregate statistics across the network
async function getNetworkStats(): Promise<{
  totalSupply: bigint;
  totalTrusts: number;
  activeVerifiers: number;
  avgReputation: number;
}> {
  const totalSupply = await contract.total_supply();
  const allTrusts = await contract.get_all_trusts();

  let activeVerifiers = 0;
  let totalReputation = 0;

  for (const trust of allTrusts) {
    // Count active verifiers in each trust
    // Aggregate reputation data
  }

  return {
    totalSupply,
    totalTrusts: allTrusts.length,
    activeVerifiers,
    avgReputation: totalReputation / activeVerifiers,
  };
}
```

### Use Case 4: Custom Incentive Programs

Third-party apps can implement external rewards based on on-chain reputation:

```typescript
// Example: Premium service for high-reputation users
async function getUserDiscount(userAddress: string): Promise<number> {
  const reputation = await getUserReputation(userAddress);

  if (reputation >= 900) return 25; // 25% discount for Legendary
  if (reputation >= 750) return 15; // 15% discount for Trusted
  if (reputation >= 600) return 10; // 10% discount for Established
  return 0; // No discount
}

// Example: Priority queue for high-reputation workers
async function getPositionInQueue(userAddress: string): Promise<number> {
  const reputation = await getUserReputation(userAddress);

  if (reputation >= 750) return 1; // Front of queue
  if (reputation >= 600) return 5; // Near front
  return 999; // Back of queue
}
```

---

## Contract Reference

### Read Functions (No Gas Required)

| Function | Input | Returns | Description |
|----------|-------|---------|-------------|
| `balance(account)` | Address | `AccountData` | Get account balance and metadata |
| `get_verifier(verifier)` | Address | `VerifierData` | Get verifier reputation stats |
| `get_trust_info(trust_id)` | Address | `TrustData` | Get trust configuration |
| `get_all_trusts()` | - | `Vec<TrustData>` | List all trusts |
| `get_work_claim(claim_id)` | u64 | `WorkClaim` | Get claim details |
| `get_proposal(proposal_id)` | u64 | `Proposal` | Get proposal details |
| `total_supply()` | - | `U256` | Get total token supply |

### Write Functions (Require Gas + Signature)

| Function | Purpose | Auth Required |
|----------|---------|---------------|
| `transfer(from, to, amount)` | Transfer tokens | Sender |
| `submit_work_claim(...)` | Submit work for verification | Worker |
| `approve_work_claim(verifier, claim_id)` | Approve a claim | Verifier |
| `reject_work_claim(verifier, claim_id)` | Reject a claim | Verifier |
| `register_trust(governor, ...)` | Create a trust | Governor |
| `join_trust(member, trust_id)` | Join a trust | Member |
| `vote_on_proposal(voter, ...)` | Vote on proposal | Voter |

---

## Data Models

### AccountData
```typescript
{
  balance: bigint;           // Current balance (with demurrage)
  last_activity: number;     // Unix timestamp
  trust_id: string | null;   // Trust membership
  contribution_hours: number; // Total hours contributed
  grace_periods_used: number; // Grace periods this year
}
```

### VerifierData
```typescript
{
  trust_id: string | null;   // Trust membership
  stake: bigint;             // Verifier stake amount
  reputation_score: number;  // 0-1000, starts at 500
  verified_claims: number;   // Total approved
  rejected_claims: number;   // Total rejected
  fraud_reports: number;     // Fraud reports filed
}
```

### WorkClaim
```typescript
{
  claim_id: number;
  worker: string;
  work_type: number;         // 0=Basic, 1=Skilled, 2=Training, 3=Emergency
  minutes_worked: number;
  evidence_hash: string;     // IPFS CID or similar
  verifiers_assigned: string[];
  approvals_received: number;
  rejections_received: number;
  status: number;            // 0=Pending, 1=Approved, 2=Rejected
  multiplier: number;        // 100, 130, 150, or 200
}
```

---

## Network Configuration

```typescript
import { getNetworkConfig } from "@kchng/shared";

// Testnet (current deployment)
const testnet = getNetworkConfig("testnet");
// {
//   networkUrl: "https://horizon-testnet.stellar.org",
//   rpcUrl: "https://soroban-testnet.stellar.org",
//   networkPassphrase: "Test SDF Network ; September 2015",
//   contractId: "CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX"
// }

// Mainnet (when deployed)
const mainnet = getNetworkConfig("mainnet");
// {
//   networkUrl: "https://horizon.stellar.org",
//   rpcUrl: "https://mainnet.soroban.rpc.stellar.org",
//   networkPassphrase: "Public Global Stellar Network ; September 2015",
//   contractId: "..." // To be filled after deployment
// }
```

---

## Best Practices

### 1. Use Type Definitions

```typescript
import type { VerifierData, WorkClaim, TrustData } from "@kchng/shared";

// Type-safe contract interactions
function processVerifier(data: VerifierData) {
  // TypeScript will ensure correct property access
  console.log(data.reputation_score);
}
```

### 2. Handle Errors Gracefully

```typescript
async function safeGetVerifier(address: string): Promise<VerifierData | null> {
  try {
    return await contract.get_verifier({
      verifier: new Address(address),
    });
  } catch (error) {
    if (error.message.includes("not found")) {
      return null; // Not a verifier
    }
    throw error; // Re-throw unexpected errors
  }
}
```

### 3. Cache Read Operations

```typescript
// Implement caching to reduce RPC calls
const cache = new Map<string, { data: VerifierData; expiry: number }>();

async function getCachedVerifier(address: string): Promise<VerifierData> {
  const cached = cache.get(address);
  if (cached && cached.expiry > Date.now()) {
    return cached.data;
  }

  const data = await contract.get_verifier({ verifier: new Address(address) });
  cache.set(address, { data, expiry: Date.now() + 60000 }); // 1 min cache
  return data;
}
```

### 4. Use Demurrage Utilities

```typescript
import { calculateInactivePeriods, calculateBalanceWithDemurrage } from "@kchng/shared/demurrage";

// Client-side demurrage calculation
const inactivePeriods = calculateInactivePeriods({
  lastActivity: accountData.last_activity,
  currentTimestamp: Date.now(),
  periodDays: 30,
});

const currentBalance = calculateBalanceWithDemurrage({
  balance: rawBalance,
  lastActivity: accountData.last_activity,
  annualRateBps: 1200,
  currentTimestamp: Date.now(),
});
```

---

## Security Considerations

### For Third-Party Apps:

1. **Never store private keys** - Use wallet SDKs for signing
2. **Verify contract address** - Ensure you're connected to the correct network
3. **Validate user input** - Don't trust addresses from URL parameters
4. **Use HTTPS** - All RPC communication should be encrypted
5. **Implement rate limiting** - Protect against abuse of your integration

### For Smart Contract Interaction:

1. **Transaction simulation** - Always simulate before signing
2. **Fee estimation** - Show users estimated gas costs
3. **Timeout handling** - RPC calls can hang, implement timeouts
4. **Retry logic** - Network failures happen, retry appropriately

---

## Getting Started Checklist

- [ ] Install dependencies: `@kchng/shared`, `@stellar/stellar-sdk`
- [ ] Configure network (testnet or mainnet)
- [ ] Implement contract client
- [ ] Add wallet connection (Freighter, Rabet, etc.)
- [ ] Implement read operations first (balance, reputation)
- [ ] Add write operations (submit claim, vote)
- [ ] Test on testnet before mainnet
- [ ] Implement error handling and caching
- [ ] Deploy and monitor

---

## Resources

- **PRD**: [docs/PRD.md](docs/PRD.md)
- **Contract**: [packages/contracts/src/lib.rs](packages/contracts/src/lib.rs)
- **Types**: [packages/shared/src/types.ts](packages/shared/src/types.ts)
- **Reputation Demo**: [apps/reputation-demo/](apps/reputation-demo/)
- **Testnet Explorer**: https://stellar.expert/explorer/testnet/contract/CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX

---

**Last Updated**: 2026-01-08
**Contract Version**: Testnet (CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX)
