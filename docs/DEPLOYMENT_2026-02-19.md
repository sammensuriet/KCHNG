# KCHNG Contract Deployment Report

**Date**: 2026-02-19
**Network**: Stellar Mainnet
**Deployer**: kchng_admin

---

## Deployment Summary

| Item | Value |
|------|-------|
| **Contract ID** | `CCPZSMXRKN3FM7WDIA3NZMJMZ6E577YDXFBUKACFQKTLBP7HZH63A5OK` |
| **WASM Hash** | `d70fde58a69dffb90551bdbf9a1c372153e851e3f8770b3624d59b1db43d7bd7` |
| **Initial Supply** | 10,000,000 KCHNG |
| **Explorer Link** | https://stellar.expert/explorer/public/contract/CCPZSMXRKN3FM7WDIA3NZMJMZ6E577YDXFBUKACFQKTLBP7HZH63A5OK |

---

## Previous Deployment (Feb 11, 2026)

| Item | Value |
|------|-------|
| **Contract ID** | `CDMKVYIU6KAATZXLLFT6KTJCKXCWP3HPYNMA3HMEUUWYVCM5DJVZ5AQS` |
| **WASM Hash** | `dc90e75de093eb27d53249025c3595d0dd093618c2c8a0f69023d4bfaf532b97` |

---

## New Features Since Last Deployment

### 1. Anti-Gaming Protections

| Feature | Value | Purpose |
|---------|-------|---------|
| Transfer cooldown | 24 hours | Prevent rapid transfers |
| Minimum transfer | 100 KCHNG | Prevent dust spam |
| Supply cap | 1 quintillion | Prevent inflation attacks |
| Contribution requirement | 100 hours | Grace period qualification (was 30) |
| Grace cooldown | 90 days | Prevent grace period abuse |
| Governor trust limit | 1 trust | Prevent trust fragmentation |

### 2. Token Minting Formula Fix

- **Previous**: `minutes * 1000 / 15` (incorrect, used MIN_WORK_MINUTES)
- **Current**: `minutes * 1000 / 30` (correct per PRD: 30 min = 1000 KCHNG)

### 3. Two-Way Reputation System

- `RoleType` enum: Governor, Verifier, Oracle, Worker, Member
- `ReputationData` struct with per-role tracking
- TF2T (Tit-for-2-Tats) pattern detection
- Time-based decay (30 days) and recovery (90 days)
- Stake slashing on unregister (10% verifiers, 25% oracles)

### 4. Type-Safe Event Publishing

Migrated from deprecated `env.events().publish()` to `#[contractevent]` macro:

| Event | Struct | Emitted When |
|-------|--------|--------------|
| Transfer | `Transfer` | Tokens transferred |
| TrustNew | `TrustNew` | New trust registered |
| MemberJoin | `MemberJoin` | Member joins trust |
| MemberLeave | `MemberLeave` | Member leaves trust |
| ClaimSubmitted | `ClaimSubmitted` | Work claim submitted |
| ClaimApproved | `ClaimApproved` | Claim approved |
| ClaimRejected | `ClaimRejected` | Claim rejected |
| GraceActivated | `GraceActivated` | Grace period activated |
| ProposalCreated | `ProposalCreated` | Governance proposal created |
| VoteCast | `VoteCast` | Vote cast on proposal |
| ReputationChanged | `ReputationChanged` | Reputation score changes |

### 5. Other Changes

- Fixed floating-point usage in grace period multiplier (now uses basis points)
- Added comments to all `REP_EVENT_*` constants
- Removed unused `symbol_short` import
- Demurrage period default: 28 days

---

## Contract Statistics

| Metric | Value |
|--------|-------|
| Total Lines | ~3,000 (Rust) |
| Storage Keys | 13 |
| Public Functions | 50+ |
| Event Types | 11 |
| Test Coverage | 74 tests (100% pass) |

---

## Breaking Changes

⚠️ **Important**: This deployment introduces breaking changes:

1. **24-hour transfer cooldown** - Users can only transfer once per 24 hours
2. **100 KCHNG minimum transfer** - Small transfers will be rejected
3. **100-hour contribution requirement** - Higher threshold for grace periods
4. **90-day grace cooldown** - Longer wait between grace periods
5. **Token minting formula** - Workers receive 50% fewer tokens per claim

---

## Verification Commands

```bash
# Check contract info
stellar contract invoke --id CCPZSMXRKN3FM7WDIA3NZMJMZ6E577YDXFBUKACFQKTLBP7HZH63A5OK --network mainnet -- total-supply

# Check account balance
stellar contract invoke --id CCPZSMXRKN3FM7WDIA3NZMJMZ6E577YDXFBUKACFQKTLBP7HZH63A5OK --network mainnet -- balance --address <ADDRESS>
```

---

## Commits Included

| Commit | Description |
|--------|-------------|
| `38dc979` | Migrate to #[contractevent] macro |
| `21240a3` | Fix final test for grace period demurrage pause |
| `6e653f0` | Fix grace period cooldown tests |
| `f09c71f` | Continue fixing remaining tests |
| `dab6f04` | Fix test_mint_capped_at_max_supply |
| `1497a89` | Continue fixing tests for transfer cooldown |
| `d51d72a` | Fix contract bugs and update tests |
| `ce0c370` | Set default demurrage period to 28 days |
| `59246cd` | Add event emission for PWA integration |
| `3870ec3` | Implement two-way reputation system |
| `44620f5` | Implement comprehensive anti-gaming protections |

---

**Report Generated**: 2026-02-19
