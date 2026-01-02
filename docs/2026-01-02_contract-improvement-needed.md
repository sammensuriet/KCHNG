# Contract Improvement Analysis

**Date:** 2026-01-02
**Contract:** `CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX`
**Status:** Ready for Review

---

## Summary

Review of contract features tested and areas needing improvement.

---

## ✅ Working Features (No Action Needed)

### Core Functionality
- ✅ Work claim submission with evidence
- ✅ Verifier assignment (2-5 verifiers)
- ✅ Approval/rejection workflow
- ✅ Worker payment (tokens minted correctly)
- ✅ Token transfers (consumption)
- ✅ Balance queries with demurrage calculation
- ✅ Total supply tracking
- ✅ Cross-trust rate calculation
- ✅ Work type multipliers (1.0×, 1.3×, 1.5×, 2.0×)

### Economic Model
- ✅ Time-standard: 30 minutes = 1 KCHNG
- ✅ Demurrage configuration: 12% annual, 30-day periods
- ✅ Per-trust rate customization (5-15% constraints)
- ✅ Formula: `(base_kchng * multiplier) / 100` (line 972)

---

## 🔴 Critical Issues (Must Fix)

### 1. Reputation System - NON-FUNCTIONAL

**Status:** Structurally defined but never implemented

**Impact:** High - Core game theory incentives missing

**Details:**
- `VerifierData.reputation_score` exists but never changes from 500
- `VerifierData.fraud_reports` exists but never written to
- No reputation updates in `approve_work_claim()`
- No reputation updates in `reject_work_claim()`
- No reputation-based verifier selection

**Reference:** `docs/2026-01-02_reputation-system-gap-analysis.md`

**Decision Required:**
- Option A: Implement full reputation logic
- Option B: Remove dead code fields
- Option C: Document as "future feature"

---

## ⚠️ Medium Priority Issues (Should Fix)

### 2. Unverified Features (Need Testing)

**Features configured but not observed working:**

#### Demurrage Application Over Time
- **Status:** Configured correctly, but not observed reducing balances
- **Code location:** `calculate_balance_with_demurrage()` (line 651)
- **Risk:** Formula could be wrong; no real-world validation
- **Action needed:**
  - Wait 30+ days OR create time acceleration test
  - Verify balance actually decreases
  - Test edge cases (leap years, partial periods)

#### Grace Periods
- **Status:** Functions exist, not tested
- **Functions:** `activate_grace_period()`, `extend_grace_period()`, `is_in_grace_period()`
- **Risk:** May not pause demurrage correctly
- **Action needed:**
  - Register test oracle
  - Activate grace period
  - Verify demurrage pause works
  - Test all three grace types (Emergency, Illness, Community)

#### Governance System
- **Status:** Complete but untested
- **Functions:** Proposals, voting, implementation
- **Risk:** Democratic processes may not work
- **Action needed:**
  - Create proposal
  - Test voting workflow
  - Verify implementation after approval
  - Test rate change constraints

#### Cross-Trust Swaps
- **Status:** Rate calculation works, actual swap untested
- **Risk:** Swap execution may fail
- **Action needed:**
  - Execute actual cross-trust swap
  - Verify tokens move between trusts
  - Test rate-adjusted amounts

---

## 🔍 Minor Issues (Nice to Have)

### 3. Dead Code Cleanup

**Fields that exist but are never used:**

```rust
// VerifierData - line 159
pub fraud_reports: u32,        // Never written to

// AccountData - potentially unused:
pub contribution_hours: u64,   // Updated but never read
pub grace_periods_used: u32,   // Updated but may not be enforced
```

**Impact:** Low - wastes storage, adds complexity

**Recommendation:** Either use them or remove them

---

### 4. Access Control Gaps

**Observations from testing:**

#### Work Claim Submission
- **Issue:** Only certain accounts can submit claims
- **Error:** `UnreachableCodeReached` for test_member1
- **Question:** What are the exact requirements?
- **Action needed:** Document access control rules clearly

#### Verifier Authorization
- **Issue:** Can't verify who is authorized to verify
- **Question:** Is there a verifier registry?
- **Action needed:** Test verifier registration workflow

---

### 5. Error Messages

**Current state:** Generic contract errors

**Examples:**
- `UnreachableCodeReached` - Doesn't explain WHY
- `HostError: Error(WasmVm, InvalidAction)` - Not user-friendly

**Improvement needed:**
- "Account not registered in a trust"
- "Not authorized to submit work claims"
- "Verifier not assigned to this claim"

---

## 📊 Feature Status Matrix

| Feature | Status | Tested | Working | Notes |
|---------|--------|--------|---------|-------|
| Work Submission | ✅ Complete | ✅ | ✅ | |
| Verifier Assignment | ✅ Complete | ✅ | ✅ | 2-5 verifiers |
| Approval Workflow | ✅ Complete | ✅ | ✅ | Majority rules |
| Rejection Workflow | ✅ Complete | ✅ | ✅ | |
| Worker Payment | ✅ Complete | ✅ | ✅ | 30min = 1 KCHNG |
| Token Transfers | ✅ Complete | ✅ | ✅ | Consumption |
| Demurrage Config | ✅ Complete | ✅ | ✅ | 12% annual |
| **Demurrage Application** | ⚠️ Needs Testing | ❌ | ❓ | Not observed |
| Work Multipliers | ✅ Complete | ⚠️ | ✅ | Formula verified |
| Grace Periods | ⚠️ Partial | ❌ | ❓ | Untested |
| Governance | ⚠️ Complete | ❌ | ❓ | Untested |
| Cross-Trust Calc | ✅ Complete | ✅ | ✅ | |
| Cross-Trust Swap | ⚠️ Partial | ❌ | ❓ | Untested |
| **Reputation System** | 🔴 **Incomplete** | ✅ | ❌ | **Dead code** |

---

## 🎯 Priority Recommendations

### Before Mainnet Deployment

#### Must Do (Critical Path):
1. ✅ **Decide on reputation system**
   - Implement it properly OR
   - Remove dead code OR
   - Document as future feature

2. ⚠️ **Test demurrage application**
   - Verify formula works over time
   - Test with real time passage

3. ⚠️ **Test grace periods**
   - Verify demurrage pause
   - Test oracle activation

#### Should Do (High Priority):
4. **Test governance system**
   - Verify voting works
   - Test proposal implementation

5. **Test cross-trust swaps**
   - Verify actual swap execution
   - Test rate adjustments

6. **Document access control**
   - Who can submit claims?
   - Who can verify?
   - Clear error messages

#### Nice to Have:
7. **Improve error messages**
   - User-friendly explanations
   - Actionable guidance

8. **Clean up dead code**
   - Remove or use unused fields
   - Simplify data structures

---

## 🧪 Testing Checklist

### Before Production:

- [ ] Reputation decision made (implement/remove/document)
- [ ] Demurrage observed reducing balances over 30+ days
- [ ] Grace period activated and verified
- [ ] Governance proposal created and voted on
- [ ] Cross-trust swap executed
- [ ] Access control documented
- [ ] Error messages improved
- [ ] All regression tests pass

---

## 💡 Design Philosophy Questions

### For Team Discussion:

1. **Reputation System**
   - Is it essential for MVP?
   - Can we launch without it?
   - Timeline for implementation?

2. **Demurrage Validation**
   - How to test without waiting 30 days?
   - Can we create time acceleration tests?
   - What's the rollback plan if formula is wrong?

3. **Access Control**
   - Who should be able to submit claims?
   - Trust members only? Anyone?
   - Should this be configurable?

4. **Grace Periods**
   - Are oracles realistic for small communities?
   - What prevents abuse?
   - Is 3 per year the right limit?

5. **Governance**
   - Will proposals actually be used?
   - Is the voting system too complex?
   - Can decisions be made off-chain instead?

---

## Conclusion

**Current State:**
- Core economic loop works ✅
- Worker payment verified ✅
- Token consumption works ✅

**Gaps:**
- Reputation system non-functional 🔴
- Advanced features untested ⚠️
- Some access control unclear ❓

**Recommendation:**
1. **Decide on reputation** (critical path item)
2. **Test unverified features** before mainnet
3. **Document access control** clearly
4. **Consider launching MVP** with core features only

**Risk Assessment:**
- **Low risk:** Work/earn/spend cycle
- **Medium risk:** Untested features (grace periods, governance)
- **High risk:** Reputation gap (if deemed essential)

---

**Next Steps:**
1. Team discussion on reputation system
2. Priority decision on testing gaps
3. Create testing plan for unverified features
4. Update documentation with access control rules

---

**Report Completed:** 2026-01-02
**Status:** Awaiting Team Decision on Priorities
**Recommendation:** Fix reputation gap, test remaining features, then mainnet
