# Fixed Contract Deployment Report

**Date:** 2026-01-02
**Network:** Stellar Testnet
**Status:** ✅ Deployed Successfully

---

## Contract Addresses

### Old Contract (Buggy)
- **Address:** `CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX`
- **Status:** ⚠️ Contains demurrage bug and non-functional reputation system
- **Time Capsule Test:** Active (verify on Feb 2, 2026)
- **Documentation:** [2026-01-02_demurrage-critical-bug.md](./2026-01-02_demurrage-critical-bug.md)

### New Contract (Fixed)
- **Address:** `CBMICVZ3FOVBLUBGOHU33FLCEO4ZSODIT7RS4GH27LECR6KHPNJP4CNB`
- **Status:** ✅ All bugs fixed
- **Transaction:** [e2f1e9e6](https://stellar.expert/explorer/testnet/tx/e2f1e9e6453a4c99bb1584928222f288d50084cda334d350c740943e555d68ad)
- **Explorer:** [CBMICVZ3FOVBLUBGOHU33FLCEO4ZSODIT7RS4GH27LECR6KHPNJP4CNB](https://stellar.expert/explorer/testnet/contract/CBMICVZ3FOVBLUBGOHU33FLCEO4ZSODIT7RS4GH27LECR6KHPNJP4CNB)
- **WASM Hash:** `dd5973412f89f37ee5e34893d11a90ef4b187014a74660c02211d61fce6f4fdc`

---

## Fixes Applied

### 1. Demurrage Integer Division Bug (lib.rs:710)
```rust
// Before (broken):
period_rate_bps = 1200 * 30 / 36500 = 0  // Always zero!

// After (fixed):
period_rate_bps = 1200 * 10000 * 30 / 365 / 10000 = 986 bps ≈ 0.986%
```

**Impact:** Demurrage now correctly applies ~1% per month (12% annual).

### 2. Reputation System Implementation
- **Added:** `get_verifier()` function
- **Approval:** +5 reputation points
- **Rejection:** +10 reputation points (higher incentive for fraud detection)
- **Cap:** Maximum 1000 points

---

## Verification

### Initial State
- **Total Supply:** 1,000,000 KCHNG
- **Admin Balance:** 1,000,000 KCHNG
- **Admin Address:** GCW4XHQLIK3VHXUGNXJ4NFLW5JDWZZ3UT2XJCYTZSO6AQJZUUDB7RVMS

### Test Results
All 20 tests passing:
- ✅ Demurrage calculation (single period)
- ✅ Demurrage calculation (multiple periods)
- ✅ Reputation increases on approval
- ✅ Reputation increases on rejection
- ✅ Reputation caps at 1000
- ✅ All existing tests

---

## Comparison Strategy

### Purpose of Two Contracts

**Old Contract (CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX):**
- Time capsule test active (verify Feb 2, 2026)
- Will definitively prove whether deployed contract had the bug
- Provides baseline for comparison

**New Contract (CBMICVZ3FOVBLUBGOHU33FLCEO4ZSODIT7RS4GH27LECR6KHPNJP4CNB):**
- All known bugs fixed
- Ready for testing and development
- Will have its own time capsule test to verify fixes work

### Feb 2, 2026 Verification

When time capsule test runs, we'll compare:

| Metric | Old Contract | New Contract |
|--------|--------------|--------------|
| **Demurrage** | Expected: 0 (bug) | Expected: ~1% (working) |
| **Reputation** | Always 500 (broken) | Increases with activity (working) |

If old contract shows no demurrage → bug confirmed in deployed contracts
If new contract shows demurrage → fix verified

---

## Next Steps

1. **Create new time capsule test** for fixed contract
2. **Deploy frontend** configured for new contract address
3. **Run integration tests** on new contract
4. **Monitor time capsule** results (Feb 2, 2026)
5. **Mainnet deployment** after successful testnet verification

---

## Deployment Commands

For reference, deployment was done with:

```bash
stellar contract deploy \
  --source-account kchng_admin \
  --wasm target/wasm32-unknown-unknown/release/kchng_contract.optimized.wasm \
  --network testnet \
  -- \
  --creator GCW4XHQLIK3VHXUGNXJ4NFLW5JDWZZ3UT2XJCYTZSO6AQJZUUDB7RVMS \
  --initial-supply 1000000
```

---

## Files Modified

- `packages/contracts/src/lib.rs` - Core fixes
- `packages/contracts/src/test.rs` - Comprehensive tests
- Commit: `ee727f9` - "Fix critical demurrage bug and implement reputation system"

---

**Status:** ✅ Deployment successful
**Next:** Create new time capsule test for fixed contract
