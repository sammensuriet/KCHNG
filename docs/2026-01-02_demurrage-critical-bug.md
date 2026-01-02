# CRITICAL BUG: Demurrage Not Actually Applying

**Severity:** 🔴 **CRITICAL**
**Date:** 2026-01-02
**Status:** **BLOCKING MAINNET DEPLOYMENT**
**Location:** `packages/contracts/src/lib.rs:709`

---

## Executive Summary

**Demurrage is configured but NOT actually applying due to integer division bug.**

The contract stores demurrage rates (12% annual), configures periods (30 days), but the calculation that should reduce balances over time produces **ZERO** due to loss of precision in integer arithmetic.

**Impact:** Tokens will NOT decay over time, breaking the core economic model.

---

## The Bug

### Location
File: `packages/contracts/src/lib.rs`
Line: 709
Function: `calculate_balance_with_demurrage()`

### Buggy Code
```rust
// Line 709
let period_rate_bps = (annual_rate_bps as u64) * period_days / 36500;
```

### What Happens

For standard configuration (12% annual, 30-day periods):

```
annual_rate_bps = 1200 (12%)
period_days = 30

period_rate_bps = (1200 * 30) / 36500
                = 36000 / 36500
                = 0  ← INTEGER DIVISION TRUNCATES TO ZERO!
```

### Result

```rust
// Line 716-720
let burn_amount = {
    let rate_factor = U256::from_u128(env, period_rate_bps as u128);
    let tmp = balance.mul(&rate_factor);  // balance * 0 = 0
    tmp.div(&U256::from_u128(env, 10000)) // 0 / 10000 = 0
};
```

**`burn_amount = 0` for all balances!**

---

## Test Evidence

### Time Acceleration Test Results

```
Initial Balance: 1000 KCHNG
Demurrage: 12% annual, 30-day periods

Month  1: 1000.00 KCHNG (0.00 lost, 0.00%)  ← SHOULD BE ~9.86 KCHNG
Month  2: 1000.00 KCHNG (0.00 lost, 0.00%)  ← SHOULD BE ~19.43 KCHNG
...
Month 12: 1000.00 KCHNG (0.00 lost, 0.00%)  ← SHOULD BE ~113.00 KCHNG
```

### Live Testnet Verification

```bash
# Worker balance after earning and waiting
Balance: 100,002 KCHNG

# After "time passes" (different days)
Balance: 100,002 KCHNG ← NO CHANGE!

Expected: Should see gradual decrease
Actual: No demurrage applied
```

---

## Root Cause Analysis

### The Formula Issue

The comment on line 708 says:
```rust
// Example: 1200 bps annual (12%), 30 day period
// period_rate = 1200 * 30 / 36500 ≈ 0.986% per period (roughly 1%)
```

**Problem:** The comment shows floating-point math (≈ 0.986), but the code uses **integer arithmetic**.

```
Expected: 36000 / 36500 = 0.986... (decimal)
Actual:   36000 / 36500 = 0       (integer)
```

### Why This Happened

1. **Integer Division Truncates**
   - Rust/Soroban integer division: `a / b` truncates toward zero
   - 36,000 / 36,500 = 0.986 → **truncates to 0**

2. **Order of Operations**
   - Multiply first: 1200 × 30 = 36,000
   - Then divide: 36,000 / 36,500 = 0
   - **Precision lost before division**

3. **Denominator Too Large**
   - 36,500 makes the ratio < 1 for most reasonable rates
   - Any result < 1 becomes 0 in integer math

---

## Impact Assessment

### Economic Model Broken

The entire Wörgl demurrage model depends on:
1. ✅ Tokens losing value over time
2. ❌ **Current: Tokens don't lose value**
3. ❌ **Result: No incentive to spend**

### What Still Works
- ✅ Token transfers
- ✅ Worker payments
- ✅ Approval/rejection
- ✅ Grace period logic (but meaningless if demurrage doesn't work)

### What Doesn't Work
- ❌ **Demurrage (primary feature of the currency)**
- ❌ Incentive to circulate tokens
- ❌ Protection against hoarding
- ❌ Economic velocity stimulation

---

## Mathematical Proof

### For 12% Annual, 30-Day Periods

**Expected Formula:**
```
monthly_rate = 0.12 × (30 / 365) ≈ 0.00986 = 0.986%
```

**Buggy Implementation:**
```
period_rate_bps = 1200 × 30 / 36500 = 0
monthly_burn = balance × 0 / 10000 = 0
```

**Correct Implementation (should be):**
```
period_rate_bps = 1200 × 30 × 100 / 36500 = 98
monthly_burn = balance × 98 / 10000 = balance × 0.0098
```

---

## Affected Configurations

### Working (Large Values)
- 100% annual, 365-day period: 100000 × 365 / 36500 = 1000 ✓
- Rare edge cases only

### Broken (All Realistic Values)
- 5% annual, 30 days: `500 × 30 / 36500 = 0` ❌
- 8% annual, 30 days: `800 × 30 / 36500 = 0` ❌
- **12% annual, 30 days: `1200 × 30 / 36500 = 0` ❌**
- 15% annual, 30 days: `1500 × 30 / 36500 = 1` ✅ (but wrong amount)

### Severity
**ALL standard configurations (5-15% annual, 30-day periods) are broken.**

---

## Fix Required

### Option A: Reorder Formula (RECOMMENDED)

Multiply by balance BEFORE dividing:

```rust
// BEFORE (broken):
let period_rate_bps = (annual_rate_bps as u64) * period_days / 36500;
let burn_amount = balance * period_rate_bps / 10000;

// AFTER (fixed):
// burn = balance * annual_rate * period_days / (365 * 10000)
let burn_amount = {
    let rate = balance.clone().mul(&U256::from_u128(env, annual_rate_bps as u128));
    let rate = rate.mul(&U256::from_u128(env, period_days as u128));
    let divisor = U256::from_u128(env, 36500u128);
    rate.div(&divisor)
};
```

### Option B: Use Fixed-Point Arithmetic

Implement proper fixed-point math with higher precision:

```rust
// Use 18 decimals like Ethereum
let balance_scaled = balance * 10^18;
let period_rate = annual_rate_bps * period_days * 10^18 / 36500;
let burn = balance_scaled * period_rate / (10000 * 10^18);
```

### Option C: Different Formula

Simplify to avoid precision loss:

```rust
// Annual demurrage directly
let annual_burn = balance * annual_rate_bps / 10000;
let period_burn = annual_burn * period_days / 365;
```

---

## Testing the Fix

### Before Deploying Fix

1. **Run time acceleration tests**
   ```bash
   ./tests/simulation/test_demurrage_time_acceleration.py
   ```

2. **Verify expected outputs**
   - 12% annual = ~12% loss over year
   - 30-day periods = ~1% loss per month
   - No unexpected zeros

3. **Test edge cases**
   - Small balances (should stop at 0)
   - Large balances (no overflow)
   - Partial periods (no demurrage)

---

## Verification Steps

### How We Found This

1. Created time acceleration test
2. Ran simulation for 12 months
3. Expected: ~12% loss
4. Actual: 0% loss
5. Investigated contract code
6. Found integer division bug at line 709

### How to Verify Fix

1. Apply code fix
2. Run time acceleration tests
3. Confirm: 1000 KCHNG → ~887 KCHNG after 1 year (12% loss)
4. Test on testnet with real time passage

---

## Deployment Impact

### Current State
- ❌ **NOT READY FOR MAINNET**
- Demurrage is broken
- Economic model doesn't work
- Tokens won't circulate as designed

### Required Before Mainnet
1. ✅ Fix integer division bug
2. ✅ Verify fix with time acceleration tests
3. ✅ Deploy fixed contract to testnet
4. ✅ Observe actual demurrage over 30+ days
5. ✅ Confirm balances decrease correctly
6. ✅ Then consider mainnet

---

## Comparison: Expected vs Actual

### Over 1 Year (1000 KCHNG, 12% annual)

| Time | Expected Balance | Actual Balance (Bug) | Loss |
|------|------------------|----------------------|------|
| Start | 1000.00 | 1000.00 | 0 |
| Month 1 | 990.14 | 1000.00 | 0 (bug) |
| Month 6 | 942.84 | 1000.00 | 0 (bug) |
| Month 12 | 886.99 | 1000.00 | 0 (bug) |

**Demurrage never applies.**

---

## Lessons Learned

1. **Test Critical Formulas**
   - Don't trust comments
   - Verify with actual values
   - Use time acceleration tests

2. **Integer Arithmetic Traps**
   - Division before multiplication loses precision
   - Order matters
   - Test with realistic values

3. **Integration Testing**
   - Unit tests didn't catch this
   - Needed time acceleration to see bug
   - Always test economic logic end-to-end

---

## Recommendations

### Immediate Actions

1. 🔴 **DO NOT DEPLOY TO MAINNET** with current code
2. 🔴 Fix line 709 integer division bug
3. ✅ Run time acceleration tests to verify fix
4. ✅ Deploy fixed contract to testnet
5. ✅ Observe real demurrage over 30+ days

### Code Review Needed

1. Check for similar integer division issues elsewhere
2. Review all financial calculations
3. Add unit tests for edge cases
4. Add integration tests for time-based logic

### Process Improvements

1. Require time acceleration tests for all time-based features
2. Add economic logic to regression test suite
3. Review all basis-point calculations
4. Test with realistic values (not just round numbers)

---

## Related Issues

This bug was discovered while investigating:
- Question: "Are there other aspects than reputation logic we need to fix?"
- Answer: **YES - Demurrage is completely broken**

**Status:**
- 🔴 Reputation: Non-functional (documented)
- 🔴 **Demurrage: Broken (NEW DISCOVERY)**
- ⚠️ Grace periods: Can't test without working demurrage
- ⚠️ Other features: Untested

---

**Report Completed:** 2026-01-02
**Severity:** CRITICAL - BLOCKING MAINNET
**Status:** AWAITING FIX
**Discovery Method:** Time acceleration testing

---

## Appendix: Test Code

Time acceleration test that discovered this bug:
`tests/simulation/test_demurrage_time_acceleration.py`

Run with:
```bash
python3 tests/simulation/test_demurrage_time_acceleration.py
```

Expected output after fix:
```
Month  1: 990.14 KCHNG (9.86 lost, 0.99%)
Month  2: 980.43 KCHNG (19.57 lost, 1.96%)
...
Month 12: 886.99 KCHNG (113.01 lost, 11.30%)
```
