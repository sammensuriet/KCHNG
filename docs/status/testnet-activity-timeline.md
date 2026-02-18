# KCHNG Testnet Activity Timeline

**Generated**: 2026-02-18
**Contract**: `CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX` (Main Testnet)

---

## Timeline of Events

```
2026-01-01 22:51:05  │  Urban Elder Care Trust created by Admin
2026-01-01 23:12:46  │  Work Claim #1 submitted (30 min basic care)
2026-01-01 23:38:08  │  Rural Health Trust created by Governor 1
2026-01-02 01:52:37  │  TestCommunity Trust created by Governor 2
2026-01-02 07:57:02  │  Time Capsule Test #1 setup (30-day period)
2026-01-02 08:21:57  │  Time Capsule Test #1 transfer completed
2026-02-10 06:48:04  │  Time Capsule Test #2 setup (7-day period contract)
2026-02-18 09:38:05  │  Time Capsule Test #2 verified via transfer (9977 KCHNG)
```

---

## Accounts & Roles

| Short ID | Full Address | Roles |
|----------|--------------|-------|
| **ADMIN** | `GCW4X...B7RVMS` | Admin, Urban Elder Care Governor, Verifier |
| **GOV1** | `GAM6N...NNDK2` | Rural Health Governor, Verifier, Worker |
| **GOV2** | `GB4KL...OK54A62` | TestCommunity Governor, Worker |
| **WORKER1** | `GBOAA...R4LUUSV` | TestCommunity Member, Worker |
| **TC7DAY** | `GBO4A...IX2ZUQP` | Time Capsule Test Account (7-day contract) |

---

## Trusts

| Trust Name | Governor | Rate | Period | Members |
|------------|----------|------|--------|---------|
| Urban Elder Care Trust | ADMIN | 12% | 30 days | 2 |
| Rural Health Trust | GOV1 | 8% | 30 days | 1 |
| TestCommunity | GOV2 | 5% | 365 days | 2 |

---

## Work Claims

### Claim #1 (Approved)
- **Worker**: GOV1
- **Work Type**: Basic Care (1.0×)
- **Minutes**: 30
- **Submitted**: 2026-01-01 23:12:46
- **Verifiers**: GOV1, ADMIN
- **Status**: Approved (2/2 approvals)
- **Tokens Minted**: 1,000 KCHNG

---

## Transactions Summary

### Time Capsule Test #1 (30-day period)
- **Setup**: 2026-01-02 07:57:02
- **Contract**: Main Testnet
- **Initial Balance**: 1,000 KCHNG
- **Purpose**: Verify demurrage applies after 30+ days
- **Status**: Awaiting verification (47+ days elapsed)

### Time Capsule Test #2 (7-day period)
- **Setup**: 2026-02-10 06:48:04
- **Contract**: `CAZNVOPPOMRYC5SUN2O4U4T4B4YWTGBMCP7PN2R4XPZQYWAQ6NRNBX6Z`
- **Initial Balance**: 10,000 KCHNG
- **Verification**: 2026-02-18 09:38:05
- **Final Balance**: 9,977 KCHNG
- **Demurrage Applied**: 23 KCHNG (0.23%)
- **Status**: ✅ Verified - Demurrage working correctly

### Transfer Verification Test (2026-02-18)
| Attempt | Amount | Result |
|---------|--------|--------|
| Test 1 | 10,000 KCHNG | ❌ Rejected (exceeds balance) |
| Test 2 | 9,978 KCHNG | ❌ Rejected (1 over balance) |
| Test 3 | 9,977 KCHNG | ✅ Success (exact balance) |

---

## Current Balances (Main Testnet Contract)

| Account | Stored Balance | Trust | Notes |
|---------|---------------|-------|-------|
| ADMIN | 698,999 | Urban Elder Care | Last active 2026-01-02 |
| GOV1 | (in trust) | Rural Health | Governor + Verifier |
| GOV2 | (in trust) | TestCommunity | Governor |
| WORKER1 | 1,000 | None | Time capsule test account |
| TC7DAY | 0 | N/A | Transferred to ADMIN (7-day contract) |

---

## Untested Features

- **Grace Periods**: No oracles registered
- **Governance**: No proposals created
- **Cross-Trust Swaps**: Not tested
- **Reputation Scoring**: Verifiers registered but no role-based scoring updates

---

## Key Learnings

1. **Demurrage works correctly**: 7-day test verified 0.23% weekly burn (~12% annual)
2. **Transfer protections work**: 24h cooldown, minimum amount, self-transfer ban
3. **Trust system functional**: 3 trusts with different rates created
4. **Work verification works**: Claim submitted, verified, and tokens minted
