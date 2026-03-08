# Demurrage Bug - Verification Status

**Date:** 2026-01-02
**Question:** How can we be SURE the deployed contract has this bug?

---

## The Honest Answer

**We cannot be 100% certain without further testing.**

### What We Know

✅ **Confirmed:**
1. Source code at `packages/contracts/src/lib.rs:709` has the integer division bug
2. Python simulation of that logic produces 0 demurrage
3. Deployment report says contract was deployed from this codebase

❓ **Unconfirmed:**
1. Does deployed contract match this source?
2. Was bug fixed before deployment without committing?
3. Does actual testnet contract behave this way?

---

## Testing Limitations

### Python Time Acceleration Test

```
What we did:
  • Read Rust/Soroban source code
  • Wrote Python equivalent
  • Simulated 12 months in seconds
  • Result: 0 demurrage

Limitation:
  • Tests our INTERPRETATION of the code
  • NOT the actual deployed contract
  • Could have misinterpreted the logic
```

### On-Chain Verification (What We Need)

```
What we need:
  • Test ACTUAL deployed contract
  • With REAL time passage
  • Cannot accelerate blockchain time

Problem:
  • Must wait 30+ days for real time
  • Or find old inactive account
```

---

## Verification Options

### Option 1: Find Old Inactive Account ⭐ (BEST)

Search for account with:
- Balance > 0
- last_activity > 30 days ago
- Check if balance decreased

**If balance unchanged:** Bug confirmed ✓
**If balance decreased:** Bug doesn't exist (deployed contract fixed) ✓

### Option 2: Wait 30 Days

Create test account:
1. Give account 1000 KCHNG
2. Record balance and timestamp
3. Wait 30 days
4. Check balance again
5. If still 1000 → bug confirmed

**Status:** Would take until Feb 2, 2026

### Option 3: Trust Deployment Process

Assume:
- Deployed contract = source code
- Source has bug → deployed has bug

**Likelihood:** Very high (standard practice)
**Certainty:** Not 100%

---

## Current Recommendation

### Before Fixing Code

1. **Search for old accounts** on testnet
   ```bash
   # Try various test accounts
   # Check last_activity timestamps
   # Look for accounts inactive 30+ days
   ```

2. **Check contract bytecode** (if possible)
   ```bash
   # Download contract WASM
   # Decompile/disassemble
   # Compare to expected bytecode
   ```

3. **Ask deployment team**
   ```bash
   # "Was contract deployed from this source?"
   # "Were any fixes applied before deployment?"
   # "Can you verify the bytecode?"
   ```

### Most Likely Scenario

```
Source Code (has bug)
         ↓
    Compiled to WASM
         ↓
  Deployed to testnet
         ↓
Deployed contract (has bug)

Probability: 95%
Reason: Standard deployment process
```

### Alternative Scenario

```
Source Code (has bug)
         ↓
 Someone noticed bug
         ↓
  Manually fixed WASM
         ↓
 Deployed to testnet
         ↓
Deployed contract (bug fixed)

Probability: 5%
Reason: Would require manual intervention
          Not documented in git
          Unusual practice
```

---

## Action Items

### Immediate

1. **Search for verification evidence**
   - Check if anyone noticed this before
   - Look for testnet issues
   - Search for demurrage discussions

2. **Document uncertainty clearly**
   - Bug exists in SOURCE CODE ✓
   - Deployed contract: UNVERIFIED ⚠️
   - Need on-chain confirmation

### Before Mainnet

3. **Definitively verify**
   - Find old account OR
   - Wait 30 days with test account OR
   - Confirm with deployment team

4. **Fix regardless**
   - Bug exists in source (confirmed)
   - Source should be fixed either way
   - Deploy fixed version to testnet
   - Verify fix works

---

## Conclusion

**Question:** "How can we be sure this affects the deployed contract?"

**Answer:** We CANNOT be 100% sure without on-chain verification.

**What we KNOW:**
- Source code has the bug ✓
- Python simulation confirms the logic fails ✓
- Deployed contract PROBABLY matches source ✓

**What we NEED:**
- On-chain verification with real time passage ⚠️
- Or confirmation from deployment process ⚠️

**RECOMMENDATION:**
1. Assume bug exists (95% probability)
2. Fix source code
3. Deploy fixed contract to testnet
4. Create test account to verify fix works
5. Wait 30 days to confirm
6. Then mainnet

---

**Status:** Source code bug confirmed, deployed contract unverified
**Confidence:** High for source, medium for deployment
**Next Step:** Fix source, verify on testnet with real time
