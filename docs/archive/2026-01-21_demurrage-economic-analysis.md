# KCHNG Demurrage Economic Analysis

**Date:** 2026-01-21  
**Purpose:** Analyze demurrage impact on realistic work amounts

---

## Executive Summary

Current demurrage parameters (12% annual) create **significant economic burden** on realistic work balances. Someone working 2.5 weeks to earn 100 KCHNG loses 51% of their savings after 6 months of inactivity.

---

## Simulation Parameters

| Parameter | Value |
|-----------|-------|
| Annual rate | 12% (1200 basis points) |
| Period | 30 days |
| Rate per period | ~0.986% (986 bps) |
| Work rate | 30 min = 1 KCHNG = 1 meal |

---

## Results: 100 KCHNG (Realistic Balance)

**Context:** 100 KCHNG = 50 hours work = 100 meals ≈ 2.5 weeks full-time work

| Time | Balance | Lost | % Lost | Economic Impact |
|------|---------|------|--------|-----------------|
| 1 month | 91 KCHNG | 9 | 9.0% | 4.5 hours = 9 meals |
| 2 months | 83 KCHNG | 17 | 17.0% | 8.5 hours = 17 meals |
| 3 months | 75 KCHNG | 25 | 25.0% | 12.5 hours = 25 meals |
| 6 months | 56 KCHNG | 44 | **44.0%** | **22 hours = 51 meals** |
| 1 year | 32 KCHNG | 68 | **68.0%** | **34 hours = 68 meals** |
| 2 years | 13 KCHNG | 87 | **87.0%** | **43.5 hours = 87 meals** |

**Period-by-period breakdown:**
```
Period 1: 100 → 91  (lost 9)
Period 2:  91 → 83  (lost 8)
Period 3:  83 → 75  (lost 8)
Period 4:  75 → 68  (lost 7)
Period 5:  68 → 62  (lost 6)
Period 6:  62 → 56  (lost 6)
```

---

## Results: 1000 KCHNG (Large Hoard)

**Context:** 1000 KCHNG = 500 hours work = 1000 meals ≈ 3 months full-time work

| Time | Balance | Lost | % Lost | Economic Impact |
|------|---------|------|--------|-----------------|
| 1 month | 902 KCHNG | 98 | 9.8% | 49 hours = 98 meals |
| 3 months | 734 KCHNG | 266 | 26.6% | 133 hours = 266 meals |
| 6 months | 539 KCHNG | 461 | **46.1%** | **230.5 hours = 461 meals** |
| 1 year | 291 KCHNG | 709 | **70.9%** | **354.5 hours = 709 meals** |
| 2 years | 87 KCHNG | 913 | **91.3%** | **456.5 hours = 913 meals** |

---

## Key Findings

### 1. Demurrage is Proportionally Similar

Both balances lose approximately the same percentage over time:
- ~10% after 1 month
- ~45% after 6 months  
- ~70% after 1 year
- ~90% after 2 years

### 2. Economic Impact is More Severe for Small Balances

For someone with 100 KCHNG (2.5 weeks work):
- Losing 44% after 6 months means **losing more than a week of work**
- Losing 68% after 1 year means **losing nearly a month of work**

This creates a **barrier to participation** for casual workers.

### 3. Grace Periods are Essential

The contract's grace period system (3 per year, up to 180 days) becomes critical for real-world usage:
- Seasonal workers can preserve earnings during off-seasons
- Illness or emergency doesn't wipe out savings
- Community support through voted grace periods

---

## Comparison with Historical Wörgl

The Wörgl experiment used **1% monthly demurrage** (similar to KCHNG's 0.986%).

Key difference: Wörgl was a **local scrip** meant for rapid circulation, not a **long-term savings vehicle**. KCHNG serves both purposes:
1. Medium of exchange (encourages circulation via demurrage)
2. Store of labor value (problematic with high demurrage)

---

## Recommendations

### Option 1: Reduce Demurrage Rate

**Current:** 12% annual (~1% monthly)  
**Proposed:** 5-6% annual (~0.5% monthly)

| Rate | 6-month loss (100 KCHNG) | 1-year loss (100 KCHNG) |
|------|--------------------------|-------------------------|
| 12% (current) | 44 KCHNG | 68 KCHNG |
| 6% (proposed) | 22 KCHNG | 39 KCHNG |
| 3% (minimal) | 11 KCHNG | 21 KCHNG |

### Option 2: Introduce Demurrage-Free Threshold

**Proposal:** First 50 KCHNG per account is demurrage-exempt

This ensures:
- Basic savings (25 hours work) are protected
- Larger hoards still face circulation incentive
- Encourages participation without fear of total loss

### Option 3: Trust-Level Configuration

Allow trusts to set lower rates:
- Urban trusts (high circulation): 8-12%
- Rural trusts (seasonal work): 3-6%
- Emergency/slow economies: 1-3%

### Option 4: Status Quo with Strong Grace Periods

Keep 12% rate but:
- Increase grace periods from 3/year to 6/year
- Allow longer grace periods (up to 365 days)
- Community voting easier to activate

---

## Data Files

- Simulation data: `docs/demurrage_simulation_100_vs_1000.json`
- Simulation script: See analysis above

---

**Status:** Analysis complete, awaiting decision on parameter adjustments.

*Generated: 2026-01-21*
