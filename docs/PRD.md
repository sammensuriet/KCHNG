# KCHNG Product Requirements Document
**Version**: 1.5
**Last Updated**: 2026-03-05
**Status**: Active Development

---

## Executive Summary

KCHNG is a Stellar blockchain community currency implementing the **Wörgl demurrage model** with a time-based economic standard. The system enables communities to create and manage local currencies backed by verified work, with built-in circulation incentives through demurrage.

**Core Economic Equation**: `30 minutes verified work = 1000 KCHNG = 1 community meal`

**Economic Correlation**: `500 work hours = 3 months work = 1000 meals`

---

## Product Vision

Create a self-sustaining community currency that:
1. **Incentivizes circulation** through demurrage (negative interest on idle tokens)
2. **Values real work** through a time-based standard (not speculative)
3. **Enables community autonomy** through federated trust governance
4. **Provides safety nets** through grace periods for hardship cases

---

## Technical Architecture

### Stack
| Layer | Technology | Purpose |
|-------|-----------|---------|
| **Blockchain** | Stellar Network with Soroban | Smart contract platform |
| **Contract** | Rust (Soroban SDK v22) | Core business logic |
| **Frontend** | SvelteKit (static PWA) | User interface |
| **Build** | pnpm workspaces | Monorepo management |
| **Deployment** | Nix + Podman | Reproducible builds |

### Contract Storage Layout
```
Instance Storage (U256 keys):
  0: Admin address
  1: Initialization flag
  2: Total supply
  3: Next claim ID
  4: Next proposal ID

Persistent Storage:
  100: Map<Address, AccountData>
  200: Map<Address, TrustData>
  300: Map<Address, VerifierData>
  400: Map<u64, WorkClaim>
  500: Map<Address, GracePeriod>
  600: Map<u64, Proposal>
  700: Map<Address, OracleData>
```

---

## Core Functionality (Foundation)

| Requirement | Status | Notes |
|-------------|--------|-------|
| Token transfer with balance checking | ✅ Complete | `transfer()` implemented |
| Balance query with demurrage calculation | ✅ Complete | Real-time calculation |
| Admin-only mint function | ✅ Complete | `require_auth()` protected |
| Total supply tracking | ✅ Complete | Persistent storage |
| Native demurrage (percentage-based) | ✅ Complete | Wörgl model: `balance * (1 - rate)^periods` |

**Technical Specs:**
- Default rate: 1200 bps (12% annual)
- Per-period calculation: `period_rate = annual_rate * 10000 * period_days / 365 / 10000`
- Time-based activity tracking: `last_activity` timestamp per account

---

## Contract Phases

### Phase 2: Trust System ✅

| Requirement | Status | Notes |
|-------------|--------|-------|
| Register trust with custom demurrage | ✅ Complete | 5-15% bounds enforced |
| Join/leave trust functionality | ✅ Complete | Member tracking |
| Trust-specific rates & periods | ✅ Complete | Per-trust configuration |
| Governor management | ✅ Complete | `require_auth()` for changes |

**Technical Specs:**
- Rate bounds: MIN 500 bps (5%), MAX 1500 bps (15%)
- Governor: single address with control over membership
- Trust membership affects demurrage rate applied

---

### Phase 3: Enhanced Demurrage ✅

| Requirement | Status | Notes |
|-------------|--------|-------|
| Trust-specific demurrage rates | ✅ Complete | 5-15% bounds enforced |
| Percentage-based calculation | ✅ Complete | Per-trust configuration |
| Balance query with trust rates | ✅ Complete | `balance_with_trust_demurrage()` |

**Technical Specs:**
- Each trust can set its own demurrage rate within bounds
- Demurrage calculated using compound formula: `balance * (1 - rate)^periods`
- Accounts inherit demurrage rate from their trust membership
- Cross-trust exchange accounts for rate differences

---

### Phase 4: Work Verification System ✅

| Requirement | Status | Notes |
|-------------|--------|-------|
| Submit work claims with evidence | ✅ Complete | IPFS hash support |
| Multi-verifier assignment | ✅ Complete | Min 2 verifiers per claim |
| Approval/rejection voting | ✅ Complete | Simple majority |
| Token minting on approval | ✅ Complete | 30 min = 1000 KCHNG base |
| Work type multipliers | ✅ Complete | 1.0×, 1.3×, 1.5×, 2.0× |

**Work Types:**
| Type | Multiplier | Example |
|------|------------|---------|
| Basic Care/Agriculture | 1.0× | Elderly care, farming |
| Skilled Care/Heavy Labor | 1.3× | Nursing, construction |
| Training/Teaching | 1.5× | Skills transfer |
| Emergency Care | 2.0× | Crisis response |

**Technical Specs:**
- Min work: 15 minutes
- Verifier stake: 100,000 KCHNG (= 100 meals = 50 hours work)
- Approval threshold: `(total_verifiers / 2) + 1`
- Minting formula: `(minutes / 30) * 1000 * multiplier / 100`
- Economic model: 30 min = 1000 KCHNG = 1 meal (ensures demurrage precision)

---

### Phase 5: Grace Period System ⚠️

| Requirement | Status | Notes |
|-------------|--------|-------|
| Oracle registration | ✅ Complete | 5M KCHNG stake (anti-gaming) |
| Emergency grace (14-90 days) | ✅ Implemented | Untested |
| Illness grace (30+ days) | ✅ Implemented | Untested |
| Community-voted grace (30-180 days) | ✅ Implemented | Untested |
| Grace period extension | ✅ Implemented | Untested |
| Demurrage suspension during grace | ✅ Implemented | Untested |
| Anti-abuse (max 3/year, 100h contribution) | ✅ Implemented | Untested |
| Low-rep oracle limits | ✅ Complete | 1 grace/year for <200 rep |

**Technical Specs:**
- Oracle stake: 5,000,000 KCHNG (increased from 500K for anti-gaming)
- Grace types stored in enum: Emergency, Illness, Community
- Overlapping grace periods: longest duration wins
- Contribution hours tracked per account
- Low-rep oracles (<200): limited to 1 grace period per year

---

### Phase 6: Cross-Trust Exchange System ⚠️

| Requirement | Status | Notes |
|-------------|--------|-------|
| Rate calculation formula | ✅ Complete | `(1 - r_src) / (1 - r_dst)` |
| Swap function | ✅ Implemented | Untested in practice |
| Balance adjustments | ✅ Implemented | Both accounts updated |
| Demurrage-aware conversion | ✅ Complete | Accounts for rates |

**Technical Specs:**
- Example: Trust A (12%) → Trust B (8%) = 0.9565 multiplier
- No fees implemented
- No slippage protection
- Direct balance transfer

---

### Phase 7: Reputation System ✅

| Requirement | Status | Notes |
|-------------|--------|-------|
| Per-role reputation tracking | ✅ Complete | Governor, Verifier, Oracle, Worker, Member |
| TF2T pattern detection | ✅ Complete | Tit-for-2-Tats for consecutive negatives |
| Time-based decay/recovery | ✅ Complete | 30d high→500, 90d low→500 |
| Probation system | ✅ Complete | Score <200 triggers probation |
| Stake slashing on unregister | ✅ Complete | 10% verifiers, 25% oracles |

**Role Types:**
| Role | Reputation Events |
|------|-------------------|
| Governor | +2 join, -5 leave (severe if empty), +5 proposal pass |
| Verifier | +5 approve, +10 reject, -10 rejected claim |
| Oracle | +5 grace granted, stake slashed on abuse |
| Worker | +5 claim approved, -10 claim rejected |
| Member | +5 join, +2 vote participate |

**TF2T (Tit-for-2-Tats) Pattern:**
- Tracks consecutive negative reputation events
- 2+ consecutive negatives: -25 bonus penalty
- Prevents gaming through alternating good/bad behavior

**Technical Specs:**
- Range: 0-1000, starts at 500 (neutral)
- Decay: High reputation (>500) decays toward 500 after 30 days
- Recovery: Low reputation (<500) recovers toward 500 after 90 days
- Probation: Score <200 triggers probation status
- Queried via `get_reputation_data(address, role)`

---

## Governance (Cross-Cutting)

Governance functions are integrated across all phases and allow trust members to propose and vote on changes.

| Requirement | Status | Notes |
|-------------|--------|-------|
| Proposal creation | ✅ Complete | Multiple proposal types |
| Voting mechanism | ✅ Implemented | Untested at scale |
| Quorum requirements | ✅ Implemented | 40% participation, 60% approval |
| Timeline enforcement | ✅ Complete | 7d review, 3d vote, 30d notice |

**Proposal Types:**
| Type | Supermajority | Purpose |
|------|---------------|---------|
| Rate Change | No | Adjust trust demurrage rate |
| Trust Parameters | No | Modify trust rules |
| Protocol Upgrade | Yes | System-level changes |
| Emergency | Yes | Crisis measures (>15% rate) |

---

## Current Deployment Status

### Testnet ✅
| Property | Value |
|----------|-------|
| **Contract ID** | `CDMSMELWB6ERPXOSD7L3DXXJIG5A6PMBT6R6VFV6FOENKYYN7QNQPBFH` |
| **Deployed** | 2026-03-01 (v3) |
| **Explorer** | [stellar.expert](https://stellar.expert/explorer/testnet/contract/CDMSMELWB6ERPXOSD7L3DXXJIG5A6PMBT6R6VFV6FOENKYYN7QNQPBFH) |
| **Version** | v3 (succession, TF2T, cross-trust fix) |

### Mainnet ✅
| Property | Value |
|----------|-------|
| **Contract ID** | `CAKJ4KXWR57TPAPBKM3KTK4RCAJNBT3TVOBXTXDZVVBNLBAL7UQKHLRO` |
| **Deployed** | 2026-03-08 (v4) |
| **Explorer** | [stellar.expert](https://stellar.expert/explorer/public/contract/CAKJ4KXWR57TPAPBKM3KTK4RCAJNBT3TVOBXTXDZVVBNLBAL7UQKHLRO) |
| **Version** | v4 (grace abuse penalty, SDK 25.3) |

**Previous versions:**
- v3: `CDMSMELWB6ERPXOSD7L3DXXJIG5A6PMBT6R6VFV6FOENKYYN7QNQPBFH` (testnet)
- v2: `CCPZSMXRKN3FM7WDIA3NZMJMZ6E577YDXFBUKACFQKTLBP7HZH63A5OK` (mainnet, deprecated)

---

## Known Issues

### Critical Blocking Issues
None identified. All contract phases are implemented as designed.

### Untested Features

| Feature | Risk Level | Status |
|---------|------------|--------|
| Demurrage over time (long-term) | High | Time capsule test running |
| Grace periods | Medium | Functions complete, untested |
| Governance voting | Medium | Functions complete, untested |
| Cross-trust swaps | Low | Formula verified, swaps untested |

### Minor Issues

| Issue | Impact | Status |
|-------|--------|--------|
| Generic error messages | UX | Improvement needed |
| Access control clarity | Security | Documentation needed |
| Unused `key_to_role_type` function | Code smell | Cleanup needed |

---

## External Features (Non-Contract)

These features are part of the KCHNG ecosystem but not implemented in the smart contract.

### Community Chat ✅

| Requirement | Status | Notes |
|-------------|--------|-------|
| Real-time messaging | ✅ Complete | Gun.js peer-to-peer |
| Wallet-based identity | ✅ Complete | Stellar address as username |
| Decentralized storage | ✅ Complete | No central database |
| Self-hosted relay | ✅ Complete | gun.kchng.org with SSL |
| Responsive UI | ✅ Complete | Desktop and mobile |

**Technical Specs:**
- **Technology**: Gun.js (decentralized graph database)
- **Endpoint**: `/communicate`
- **Channel**: `kchng/chat` (public, single channel)
- **Identity**: Connected wallet address (truncated display)
- **Message limit**: 200 messages retained locally
- **Relay server**: `https://gun.kchng.org/gun` (WSS)

**Architecture:**
```
Browser ←→ Gun.js Client ←→ WSS ←→ Nginx ←→ Gun Relay (VPS)
                                                       ↓
                                              Gun P2P Network
```

**Implementation Report**: `docs/2026-02-28_gun-chat-implementation.md`

---

## Frontend Status

### Completed ✅
- Contract client (all 39 methods)
- TypeScript type definitions
- Network configurations (testnet/mainnet/standalone)
- Landing page with demurrage explanation
- Basic routing structure
- Wallet connection UI
- Dashboard with balance display
- **Community chat (Gun.js)**

### In Progress ⚠️
- Trust management interface
- Work verification UI
- Governance voting interface

### Planned ❌
- IPFS evidence upload
- Grace period application
- Cross-trust exchange UI
- Mobile responsiveness
- Accessibility audit

---

## Testing Coverage

### Unit Tests ✅
```
packages/contracts/src/test.rs: 15/15 tests passing
- Core operations (transfer, mint, balance)
- Trust system (register, join)
- Work claims (submit, approve, reject)
- Governance (create, vote)
- Demurrage calculation
```

### Integration Tests ✅
```
tests/regression/
- test_contract_functionality.sh: Basic operations
- behavioral_tests.sh: User workflows
- full_behavioral_tests.sh: End-to-end scenarios
- time_capsule_demurrage_test.sh: Long-term verification
```

### Missing Tests ❌
- Reputation score changes (in practice)
- Grace period activation and expiry
- Governance proposal lifecycle
- Cross-trust swap execution
- Stress testing (high volume)
- Security fuzzing

---

## Security Considerations

### Implemented ✅
- `require_auth()` on all state-changing functions
- Admin-only mint function
- Protocol-level rate bounds (5-15%)
- Stake requirements (verifiers, oracles)
- Supermajority for emergency proposals

### Pending ⚠️
- Multi-sig admin control
- Timelock on proposal implementation
- Circuit breaker for emergencies
- Audit by third-party firm
- Verifier slashing for fraud

---

## Anti-Requirements (Explicitly Out of Scope)

These features are **deliberately not implemented** due to security, complexity, or gaming concerns.

### Liquid Democracy / Vote Delegation ❌

**Decision**: Vote delegation will NOT be implemented.

**Rationale**: Liquid democracy creates a severe Sybil attack vector:
- Bad actor creates 1000 dummy accounts
- Each dummy account delegates voting power to the attacker
- Attacker gains 1000× voting power with minimal cost

**Current Design**: One member = one vote (no delegation)

**Alternative Considered**: Quadratic voting with identity verification
- Rejected due to complexity and reliance on external identity providers

### Other Anti-Requirements

| Feature | Reason for Exclusion |
|---------|---------------------|
| Anonymous voting | Transparency required for community trust |
| Token-weighted voting | Would concentrate power with wealthy accounts |
| Automatic slashing | Requires due process through governance proposals |
| Fast-track governance | 7-day review period protects against rash decisions |

---

## Roadmap

### Immediate (Before Mainnet)
1. **Security Audit** - Third-party review of contract
2. **Complete Frontend** - Trust management, work verification, governance UI
3. **Test Advanced Features** - Grace periods, governance, swaps in practice

### Short Term (Post-Mainnet)
1. **Community Onboarding** - Documentation, tutorials
2. **Mobile App** - Progressive Web App enhancements
3. **Analytics** - Dashboard for trust administrators
4. **IPFS Integration** - Decentralized evidence storage
5. **Chat Enhancements** - Message signing, private channels, moderation

### Long Term
1. **Multi-Chain** - Consider other chains
2. **Fiat Bridge** - Connect to traditional currency
3. **Reputation Marketplace** - Cross-trust verifier reputation
4. **Advanced Analytics** - Community health metrics, forecasting

---

## Appendix: Key Constants

```rust
// Economic Constants
MINUTES_PER_KCHNG: 30          // Time standard (30 min = 1000 KCHNG = 1 meal)
KCHNG_PER_MEAL: 1000           // 1000 KCHNG = 1 community meal
MIN_WORK_MINUTES: 15           // Minimum claim
DEFAULT_ANNUAL_RATE_BPS: 1200  // 12%

// Economic Correlation
500_WORK_HOURS = 1000_MEALS    // 3 months full-time work
VERIFIER_STAKE = 100_MEALS     // 100,000 KCHNG = 50 hours work

// Protocol Bounds
MIN_ANNUAL_RATE_BPS: 500       // 5% minimum
MAX_ANNUAL_RATE_BPS: 1500      // 15% maximum

// Stake Requirements
VERIFIER_STAKE: 100,000 KCHNG  // 100 meals = 50 hours work
ORACLE_STAKE: 5,000,000 KCHNG  // 5000 meals = 2500 hours work (anti-gaming: increased from 500K)

// Governance
MIN_VERIFIERS: 2
REVIEW_PERIOD_DAYS: 7
VOTING_PERIOD_DAYS: 3
IMPLEMENTATION_DELAY_DAYS: 30
QUORUM_PARTICIPATION_BPS: 4000  // 40%
QUORUM_APPROVAL_BPS: 6000        // 60%

// Reputation
REPUTATION_MIN: 0
REPUTATION_MAX: 1000
REPUTATION_INITIAL: 500
```

---

## References

- **Design Doc**: `docs/time-standard-token-design.md`
- **Deployment Report**: `docs/DEPLOYMENT_REPORT.md`
- **Chat Implementation**: `docs/2026-02-28_gun-chat-implementation.md`
- **Contract**: `packages/contracts/src/lib.rs`
- **Tests**: `packages/contracts/src/test.rs`, `tests/regression/`

---

**Document Status**: ✅ Aligned with contract (v1.5) - phase numbering matches lib.rs, deployment status updated
