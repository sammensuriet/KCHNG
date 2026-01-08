# KCHNG Reputation System Demo

A demonstration of how third-party applications can use the KCHNG contract's reputation system to build custom features.

## Concept

The KCHNG contract tracks verifier reputation scores on-chain:
- **Range**: 0-1000 (starts at 500, neutral)
- **+5 points** for each claim approved
- **+10 points** for each claim rejected (higher incentive for fraud detection)

**Key Design**: The contract **tracks** reputation but doesn't **enforce** how it's used. Third-party apps can implement their own logic using this data.

## What This Demo Shows

### 1. Reputation Leaderboard
Rank all verifiers by their reputation score with visual tiers:
- 🏆 Legendary (900+)
- ⭐ Trusted (750+)
- ✓ Established (600+)
- ◯ Neutral (500+)
- △ New (400+)
- ▽ Unproven (<400)

### 2. Custom Trust Scoring
Demonstrates third-party logic that combines multiple factors:
- Base reputation score
- Verification activity bonus
- Balanced approval/rejection ratio
- Penalty for potential rubber-stamping

### 3. Verifier Recommendations
Shows how apps could recommend verifiers for work claims based on custom algorithms.

### 4. Extensibility Examples
Ideas for third-party features using reputation data:
- Verifier bounties (higher pay for top-tier)
- Priority queues
- Badge systems
- Discount programs
- Insurance pricing
- Governance weighting

## Installation

```bash
cd apps/reputation-demo
pnpm install
```

## Usage

```bash
# Build and run
pnpm dev

# Or separately:
pnpm build
pnpm start
```

## Architecture

```
apps/reputation-demo/
├── src/
│   └── index.ts        # Demo application
├── package.json
├── tsconfig.json
└── README.md
```

## Contract Integration

The demo queries the KCHNG contract's `get_verifier(address)` function:

```typescript
// Contract method
get_verifier(verifier: Address) -> VerifierData

// Returns:
{
  trust_id: Address | null,
  stake: Amount,
  reputation_score: number,    // 0-1000
  verified_claims: number,
  rejected_claims: number,
  fraud_reports: number,
}
```

## Sample Output

```
============================================================
📊 REPUTATION LEADERBOARD
============================================================

#1 GCW4XHQLIK3V...RVMS
   Reputation: 740/1000
   Tier: ✓ Established
   Trust Score: 770/1000
   Verified: 55 | Rejected: 6

#2 GAM6N54Y5SB...NDK2
   Reputation: 660/1000
   Tier: ✓ Established
   Trust Score: 685/1000
   Verified: 40 | Rejected: 4

============================================================
🎯 RECOMMENDED VERIFIERS FOR YOUR WORK CLAIM
============================================================

1. GCW4XHQLIK3V...RVMS
   ✓ Established (Reputation: 740, Trust Score: 770) - Active fraud detector

2. GAM6N54Y5SB...NDK2
   ✓ Established (Reputation: 660, Trust Score: 685) - Highly experienced
```

## Production Considerations

For a production app, you would:

1. **Add proper error handling** for RPC failures
2. **Cache verifier data** to reduce RPC calls
3. **Implement real transaction simulation** for contract queries
4. **Add authentication** for user-specific features
5. **Store historical data** for reputation tracking over time
6. **Implement reputation decay** for inactive verifiers (off-chain logic)

## Related Documentation

- [PRD](../../docs/PRD.md) - Product Requirements Document
- [Contract](../../packages/contracts/src/lib.rs) - Main contract implementation
- [Types](../../packages/shared/src/types.ts) - TypeScript type definitions

## License

MIT
