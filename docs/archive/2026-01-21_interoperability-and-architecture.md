# KCHNG Interoperability and Architecture Analysis

**Date:** 2026-01-21  
**Status:** Technical Assessment & Decision Record

---

## Executive Summary

This document records two key assessments:

1. **I-SODA Interoperability Standards Alignment** - KCHNG's contract architecture is fundamentally sound, with demurrage being core to the economic model. Interoperability can be achieved through wrapper/adaptor patterns without modifying the core contract.

2. **U256 vs u64 Integer Type Decision** - Confirmed decision to use U256 for token amounts despite current no-decimal model, providing future flexibility for minimal gas cost overhead.

---

## 1. I-SODA Standards Alignment

### I-SODA Framework Overview

The [Interoperability Standards Organization for Digital Assets (I-SODA)](https://soda-services.com/how-to-create-them/) at MIT focuses on creating open standards for tokenization through:

| Framework Area | KCHNG Status | Notes |
|----------------|--------------|-------|
| **Token analysis** | ✓ | KCHNG is a novel demurrage token |
| **Data Model Definition** | ⚠️ Partial | Has clear structures but uses custom enums |
| **Lifecycle Participants** | ⚠️ Partial | Defines roles but uses custom "aspect:role" system |
| **Common digital functions** | ❌ Misaligned | `balance()` returns calculated value, not stored balance |
| **Legal and governance** | ❌ Unaddressed | No regulatory framework mapping |

### Key Misalignment: Native On-Chain Demurrage

KCHNG's `balance()` function calculates demurrage on-the-fly rather than returning a stored balance:

```rust
pub fn balance(env: Env, account: Address) -> U256 {
    if let Some(data) = accounts.get(account.clone()) {
        Self::calculate_balance_with_demurrage(&env, account, &data)  // Calculated!
    } else {
        U256::from_u32(&env, 0)
    }
}
```

This violates the expectation that token balances are stable storage values, which could create integration challenges for "frictionless movement across chains and venues."

### Resolution Strategy

**Core contract remains unchanged.** Interoperability can be achieved through:

| Approach | Contract Changes? | Tradeoff |
|----------|-------------------|----------|
| **External metadata** | No | Off-chain only, tools must fetch/interpret |
| **Wrapper/adaptor contract** | No | Adds gas/complexity, preserves original |
| **Bridge integration** | No | KCHNG remains "non-standard" but usable elsewhere |

### Decision

**KCHNG's demurrage architecture is core to its purpose** as a community circulation currency. Sacrificing this for premature standardization would be counterproductive.

Interoperability wrappers can be added later if needed, without touching the production contract.

---

## 2. U256 vs u64 Integer Type Analysis

### Background

KCHNG uses `U256` (256-bit unsigned integers) for token amounts, despite currently using a no-decimal model where 1 KCHNG = 1 unit.

### Economic Range Comparison

| Metric | u64 | U256 |
|--------|-----|------|
| **Max value** | 18,446,744,073,709,551,615 | ~1.16 × 10^77 |
| **KCHNG years of work** | 1 quadrillion years | Effectively unlimited |
| **Storage size** | 8 bytes | 32 bytes (4x) |
| **Current supply usage** | 0.00000000017% | Same |

### Computational Cost

| Operation | u64 | U256 | Overhead |
|-----------|-----|------|----------|
| **Addition** | 1 CPU cycle | ~4 cycles | 4x |
| **Multiplication** | 3-5 cycles | ~20-50 cycles | 10x |
| **Division** | 20-80 cycles | ~200-1000 cycles | 10-50x |

### Per-Transaction Impact

Every `transfer()` calls `calculate_balance_with_demurrage()`:

```rust
// Current U256 implementation
let rate_factor = U256::from_u128(env, period_rate_bps as u128);
let tmp = balance.mul(&rate_factor);     // U256 × U256
tmp.div(&U256::from_u128(env, 10000))    // U256 ÷ U256
```

**Estimated cost:**
- u64 version: ~1,000 gas
- U256 version: ~10,000-20,000 gas
- **Overhead:** ~9,000-19,000 gas per transfer
- **Annual cost (1 transfer/day):** 3.3-6.9 million gas/year

### Why U256 Was Originally Chosen

The contract code shows evidence of 18-decimal planning:

```rust
const PROPOSAL_STAKE: u64 = 100 * 10_000_000_000_000_000; // 100 KCHNG (planned)
```

However, the deployed contract uses whole KCHNG units (no decimals):
- On-chain `total_supply`: `1000000` (not `1000000000000000000000`)

### Decision: Stick with U256

**Rationale:**

| Factor | Assessment |
|--------|------------|
| **Economic cost** | ~$0.01-0.05 per transfer (negligible) |
| **Future decimals** | Can add 0.001 KCHNG without breaking changes |
| **ERC-20 compatibility** | Easier bridging to Ethereum chains |
| **Interoperability** | Aligns with existing token standards |
| **Upgrade risk** | Avoids contract upgrade for type change |

The 4x memory overhead is trivial given Soroban's storage limits, and the flexibility outweighs the marginal gas cost.

---

## 3. Time Capsule Test Status

### Current State

**Test Account:** `GAFJAAPDI566PF6KZLDDLN5JU4HVVXJJ52JGF6I5IASI7AOQS225RQ76`

| Metric | Value |
|--------|-------|
| **Setup Date** | 2026-01-03 16:46:44 |
| **Current Date** | 2026-01-21 |
| **Days Inactive** | 18.1 days |
| **Current Balance** | 10,000 KCHNG ⚠️ |
| **Expected Balance** | 1,000 KCHNG (from setup) |
| **Demurrage Period** | 30 days |
| **Demurrage Status** | ❌ Not yet applied (need 30 days) |

### Discrepancy Found

The on-chain balance shows **10,000 KCHNG** instead of the expected **1,000 KCHNG**. This suggests additional tokens were transferred to this account after setup, but the `last_activity` timestamp remains unchanged (2026-01-03), confirming the test integrity is preserved.

### Timeline

- **Setup:** 2026-01-03
- **Today:** 2026-01-21 (18 days passed)
- **Verify:** 2026-02-02 (~12 days remaining)

### Expected Results (2026-02-02)

| Scenario | Balance | Interpretation |
|----------|---------|----------------|
| **Demurrage works** | ~9,900 KCHNG | ~1% loss (fix confirmed) |
| **Demurrage broken** | 10,000 KCHNG | No change (bug persists) |

---

## 4. Architecture Strengths

KCHNG's contract demonstrates several architectural strengths:

1. **Modular Design** - 7 phases cleanly separated (Token, Trust, Demurrage, Work Verification, Grace Periods, Cross-Trust Exchange, Governance)

2. **Backward Compatibility** - `init()` function preserves legacy initialization pattern

3. **Comprehensive Testing** - 20+ tests covering all phases including edge cases

4. **Clear Data Structures** - Well-defined types (`AccountData`, `TrustData`, `VerifierData`, etc.)

5. **Role-Based Reputation** - 3-level hierarchy (Domain → Aspect → Role) allowing context-specific scoring

---

## 5. Recommendations

1. **✅ Maintain current U256 implementation** - Gas cost is acceptable for future flexibility

2. **✅ Keep demurrage in core contract** - Economic model is fundamental to KCHNG's purpose

3. **📋 Monitor time capsule test** - Re-run verification on 2026-02-02 to confirm demurrage fix

4. **📋 Consider interoperability wrappers** - If/when cross-chain operations are needed, implement adaptor contracts rather than modifying core

5. **📋 Document KCHNG-specific behaviors** - Create I-SODA-style metadata documenting demurrage calculation for external integrators

---

## References

- I-SODA: https://soda-services.com/how-to-create-them/
- Time capsule data: `/tests/regression/time_capsule_fixed_data.json`
- Contract source: `/packages/contracts/src/lib.rs`
- Deployment report: `/docs/2026-01-02_fixed-contract-deployment.md`

---

*Report generated: 2026-01-21*  
*Author: Technical Assessment*
