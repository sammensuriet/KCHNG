# KCHNG Contract: Current vs Mainnet Deployment Differences

## Context

This report summarizes the differences between the current contract code and the version deployed on Stellar mainnet.

**Mainnet Deployment:**
- **Date**: 2026-02-11
- **Commit**: `e709623`
- **Contract ID**: `CDMKVYIU6KAATZXLLFT6KTJCKXCWP3HPYNMA3HMEUUWYVCM5DJVZ5AQS`
- **WASM Hash**: `dc90e75de093eb27d53249025c3595d0dd093618c2c8a0f69023d4bfaf532b97`

---

## Summary of Changes Since Mainnet Deployment

### 4 commits modified the contract after mainnet deployment:

| Commit | Date | Description |
|--------|------|-------------|
| `44620f5` | 2026-02-12 | Comprehensive anti-gaming protections |
| `cfc8f8e` | 2026-02-13 | Token minting formula fix (KCHNG_PER_30MINUTES) |
| `e38024b` | 2026-02-16 | Token minting formula fix (30-minute divisor) |
| Current | 2026-02-18 | Two-way reputation system (just implemented) |

---

## Detailed Differences

### 1. Anti-Gaming Protections (NOT on mainnet)

Added in commit `44620f5` - these features are **NOT** in the mainnet deployment:

| Feature | Constant | Value | Purpose |
|---------|----------|-------|---------|
| Transfer cooldown | `TRANSFER_COOLDOWN_SECONDS` | 24 hours | Prevent rapid transfers |
| Minimum transfer | `MIN_TRANSFER_AMOUNT` | 100 KCHNG | Prevent dust spam |
| Supply cap | `MAX_SUPPLY` | 1 quintillion | Prevent inflation attacks |
| Contribution requirement | `MIN_CONTRIBUTION_HOURS` | 100 hours | Grace period qualification |
| Grace cooldown | `GRACE_COOLDOWN_DAYS` | 90 days | Prevent grace period abuse |
| Governor trust limit | `KEY_GOVERNOR_TRUSTS` | 1 trust per governor | Prevent trust fragmentation |

**Code changes in `transfer()` function:**
- Self-transfer prevention
- Minimum amount check (100 KCHNG)
- 24-hour cooldown between transfers

**Code changes in `mint()` function:**
- Supply cap check (1 quintillion max)

**Code changes in `register_oracle()` function:**
- Increased stake requirement to 5M KCHNG (from 500K)

### 2. Token Minting Formula Fix (NOT on mainnet)

The mainnet contract has a **bug** in the token minting formula:

**Mainnet (buggy):**
```rust
// Uses MIN_WORK_MINUTES (15) as divisor
let base_kchng = claim.minutes_worked * KCHNG_PER_30MINUTES / MIN_WORK_MINUTES;
```

**Current (fixed):**
```rust
// Uses 30 as divisor (correct per PRD: 30 min = 1000 KCHNG = 1 meal)
let base_kchng = claim.minutes_worked * KCHNG_PER_30MINUTES / 30;
```

**Impact**: Workers on mainnet receive **2x more tokens** than intended (15 vs 30 divisor).

### 3. Two-Way Reputation System (NOT on mainnet)

Just implemented - not deployed anywhere yet:

| Feature | Description |
|---------|-------------|
| `RoleType` enum | Governor, Verifier, Oracle, Worker, Member |
| `ReputationData` struct | Per-role reputation tracking |
| `update_reputation()` | Reputation changes with TF2T pattern tracking |
| `apply_reputation_decay()` | Time-based decay (30 days high, 90 days low) |
| `is_on_probation()` | Check probation status |
| `unregister_verifier()` | Unregister with 10% stake slashing |
| `unregister_oracle()` | Unregister with 25% stake slashing |
| New `ProposalType`s | RemoveVerifier, RemoveGovernor, RemoveOracle, RoleProbation |

**Reputation changes integrated into:**
- `approve_work_claim()` - Verifier +5, Worker +5
- `reject_work_claim()` - Verifier +10, Worker -10
- `join_trust()` - Governor +2, Member +5
- `leave_trust()` - Governor -5 (severe if empty)
- `activate_grace_period()` - Oracle +5
- `register_trust()` - Initialize governor reputation

### 4. Features Present on Mainnet (Unchanged)

These features exist on mainnet and haven't changed:

- ✅ Native demurrage (2 KCHNG per 28 days inactivity)
- ✅ Trust system with custom rates (5-15%)
- ✅ Work verification with multi-verifier system
- ✅ Grace periods (Emergency, Illness, Community)
- ✅ Governance voting (RateChange, TrustParameters, etc.)
- ✅ Cross-trust exchange
- ✅ Basic reputation score (0-1000, stored in VerifierData/OracleData)

---

## Storage Differences

| Storage Key | Mainnet | Current |
|-------------|---------|---------|
| `KEY_REPUTATIONS` (900) | ❌ Not present | ✅ Map<Address, Map<RoleType, ReputationData>> |
| `KEY_GOVERNOR_TRUSTS` (201) | ❌ Not present | ✅ Governor-to-trust mapping |
| `KEY_LAST_GRACE_TIMES` (501) | ❌ Not present | ✅ Grace period cooldown tracking |

---

## Upgrade Path Considerations

If upgrading mainnet to current version:

1. **Storage Migration Required**:
   - Initialize `KEY_REPUTATIONS` for all existing verifiers/oracles/governors
   - Initialize `KEY_GOVERNOR_TRUSTS` for all governors
   - Initialize `KEY_LAST_GRACE_TIMES` (empty is fine)

2. **Breaking Changes**:
   - 24-hour transfer cooldown will affect existing users
   - 100 KCHNG minimum transfer may break small transactions
   - Token minting formula change will reduce worker payouts by 50%

3. **Governance Required**:
   - Contract upgrade needs `ProtocolUpgrade` proposal
   - Requires 40% quorum, 60% approval (or 80% for emergency)

---

## Files to Modify for Upgrade

| File | Changes Needed |
|------|----------------|
| `packages/contracts/src/lib.rs` | Add migration functions |
| `packages/shared/src/types.ts` | Add new types (RoleType, ReputationData) |

---

## Verification Commands

```bash
# Check current mainnet contract info
stellar contract invoke --id CDMKVYIU6KAATZXLLFT6KTJCKXCWP3HPYNMA3HMEUUWYVCM5DJVZ5AQS -- network mainnet -- balance --address <address>

# Compare WASM hashes
# Mainnet: dc90e75de093eb27d53249025c3595d0dd093618c2c8a0f69023d4bfaf532b97
# Current: (run `stellar contract upload --wasm ...` to get new hash)
```

---

**Report Generated**: 2026-02-18
