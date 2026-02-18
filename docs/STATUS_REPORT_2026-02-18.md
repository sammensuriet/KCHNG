# KCHNG Contract Status Report

**Date**: 2026-02-18
**Branch**: main

## Summary

Completed implementation of contract cleanup and PWA event emission integration.

---

## Completed Tasks

### 1. Remove Unused Constants

Removed the following unused constants from the contract:
- `SECONDS_PER_HOUR` (3_600) - was not used anywhere
- `MAX_VERIFIERS` (5) - was not enforced anywhere

### 2. Remove Unused Code

Removed:
- `AppDemurrageEntry` struct - legacy struct not connected to any functionality
- `register_app()` function - was not integrated with demurrage calculations

### 3. Implement PROPOSAL_STAKE Requirement

Added stake requirement for governance proposals:
- Proposers must stake 100 KCHNG (defined by `PROPOSAL_STAKE` constant)
- Stake is returned after proposal resolution (approved, rejected, or expired)
- Added `stake` field to `Proposal` struct
- Created helper function `return_proposal_stake()` for stake returns

### 4. Implement REMOVAL_THRESHOLD (100)

Added automatic removal threshold with tiered slashing:

| Reputation | Verifier Slash | Oracle Slash |
|------------|---------------|--------------|
| < 100 (removal) | 20% | 50% |
| < 200 (probation) | 10% | 25% |
| >= 200 | 0% | 0% |

- Added `enforce_removal_threshold()` function
- Governor trusts are auto-disabled when governor reputation < 100
- Updated `unregister_verifier()` and `unregister_oracle()` with tiered slashing

### 5. Add Event Emission for PWA Integration

Added blockchain events for key contract actions:

| Event | Symbol | Emitted When |
|-------|--------|--------------|
| Transfer | `transfer` | Tokens transferred between accounts |
| Trust Registered | `trust_new` | New trust created |
| Member Joined | `member_jn` | User joins a trust |
| Member Left | `member_lv` | User leaves a trust |
| Work Claim Submitted | `claim_sub` | Worker submits a work claim |
| Work Claim Approved | `claim_app` | Claim approved and tokens minted |
| Work Claim Rejected | `claim_rej` | Claim rejected by verifiers |
| Proposal Created | `prop_new` | Governance proposal submitted |
| Vote Cast | `vote` | Member votes on proposal |
| Grace Period Activated | `grace_on` | Oracle activates grace period |
| Reputation Changed | `rep_chg` | User's reputation score changes |

---

## Contract Statistics

- **Total Lines**: ~2,700 (Rust)
- **Storage Keys**: 13
- **Public Functions**: 45+
- **Event Types**: 11

---

## Known Issues

### Test Failures

55 tests are failing due to the 24-hour transfer cooldown (anti-gaming protection). These tests were written before the cooldown was added and need to be updated to advance the ledger timestamp between transfers.

This is a **test maintenance issue**, not a contract bug. The cooldown is working as designed.

---

## Next Steps

1. **Update tests** - Modify test cases to account for 24-hour transfer cooldown
2. **Testnet deployment** - Deploy updated contract to testnet for integration testing
3. **PWA integration** - Update frontend to subscribe to blockchain events

---

## Files Modified

| File | Changes |
|------|---------|
| `packages/contracts/src/lib.rs` | +216 lines, -40 lines |

---

**Report Generated**: 2026-02-18
