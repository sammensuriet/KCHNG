# KCHNG Product Requirements Document
**Version**: 1.0
**Last Updated**: 2026-01-08
**Status**: Active Development

---

## Executive Summary

KCHNG is a Stellar blockchain community currency implementing the **Wörgl demurrage model** with a time-based economic standard. The system enables communities to create and manage local currencies backed by verified work, with built-in circulation incentives through demurrage.

**Core Economic Equation**: `30 minutes verified work = 1 KCHNG = 1 community meal`

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

## Requirements by Phase

### Phase 1: Core Token ✅

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

### Phase 3: Work Verification ✅

| Requirement | Status | Notes |
|-------------|--------|-------|
| Submit work claims with evidence | ✅ Complete | IPFS hash support |
| Multi-verifier assignment | ✅ Complete | Min 2 verifiers per claim |
| Approval/rejection voting | ✅ Complete | Simple majority |
| Token minting on approval | ✅ Complete | 30 min = 1 KCHNG base |
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
- Verifier stake: 100,000 KCHNG
- Approval threshold: `(total_verifiers / 2) + 1`
- Minting formula: `(minutes / 30) * multiplier / 100`

---

### Phase 4: Reputation System ✅

| Requirement | Status | Notes |
|-------------|--------|-------|
| Reputation score tracking (0-1000) | ✅ Complete | Initialized at 500 |
| Score increases on approval (+5) | ✅ Complete | Per-approver increment |
| Score increases on rejection (+10) | ✅ Complete | Higher reward for fraud detection |
| Queryable via `get_verifier()` | ✅ Complete | Public read access |

**Design Philosophy:**
Reputation is tracked on-chain but **not enforced** by the contract. This follows the same pattern as `register_app()` - the contract provides extensible data that third-party applications can use for their own logic:
- Off-chain verifier selection algorithms
- Reputation-weighted UI/UX
- External incentive/bonus systems
- Community-driven reputation markets

**Technical Specs:**
- Range: 0-1000, starts at 500 (neutral)
- Approval: +5 points
- Rejection: +10 points (higher incentive to catch fraud)
- No cap resets (permanent reputation building)
- Queried via `get_verifier(address).reputation_score`

---

### Phase 5: Grace Periods ⚠️

| Requirement | Status | Notes |
|-------------|--------|-------|
| Oracle registration | ✅ Complete | 500K KCHNG stake |
| Emergency grace (14-90 days) | ✅ Implemented | Untested |
| Illness grace (30+ days) | ✅ Implemented | Untested |
| Community-voted grace (30-180 days) | ✅ Implemented | Untested |
| Grace period extension | ✅ Implemented | Untested |
| Demurrage suspension during grace | ✅ Implemented | Untested |
| Anti-abuse (max 3/year, 30h contribution) | ✅ Implemented | Untested |

**Technical Specs:**
- Oracle stake: 500,000 KCHNG
- Grace types stored in enum: Emergency, Illness, Community
- Overlapping grace periods: longest duration wins
- Contribution hours tracked per account

---

### Phase 6: Cross-Trust Exchange ⚠️

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

### Phase 7: Governance ⚠️

| Requirement | Status | Notes |
|-------------|--------|-------|
| Proposal creation | ✅ Complete | 4 proposal types |
| Voting mechanism | ✅ Implemented | Untested at scale |
| Quorum requirements | ✅ Implemented | 40% participation, 60% approval |
| Timeline enforcement | ✅ Complete | 7d review, 3d vote, 30d notice |
| Proposal implementation | ✅ Implemented | Untested |

**Proposal Types:**
| Type | Supermajority | Purpose |
|------|---------------|---------|
| Rate Change | No | Adjust trust demurrage rate |
| Trust Parameters | No | Modify trust rules |
| Protocol Upgrade | Yes | System-level changes |
| Emergency | Yes | Crisis measures (>15% rate) |

**Technical Specs:**
- Review period: 7 days (editable)
- Voting period: 3 days (editable)
- Implementation delay: 30 days (editable)
- Quorum: 40% of trust members must vote
- Approval: 60% of votes must support

---

## Current Deployment Status

### Testnet ✅
| Property | Value |
|----------|-------|
| **Contract ID** | `CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX` |
| **Explorer** | [stellar.expert](https://stellar.expert/explorer/testnet/contract/CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX) |
| **WASM Size** | 55,956 bytes (optimized) |
| **Initial Supply** | 1,000,000 KCHNG |
| **Public Methods** | 39 |

### Mainnet ❌
Not deployed. Pending:
- Security audit
- Frontend completion
- Testing of advanced features (grace periods, governance, swaps)

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
| Dead code (`fraud_reports` field) | Code smell | Cleanup needed |
| Generic error messages | UX | Improvement needed |
| Access control clarity | Security | Documentation needed |

---

## Frontend Status

### Completed ✅
- Contract client (all 39 methods)
- TypeScript type definitions
- Network configurations (testnet/mainnet/standalone)
- Landing page with demurrage explanation
- Basic routing structure

### In Progress ⚠️
- Wallet connection UI
- Dashboard with balance display
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

## Roadmap

### Immediate (Before Mainnet)
1. **Fix Reputation System** - Either implement reputation-weighted decisions OR remove if not essential
2. **Security Audit** - Third-party review of contract
3. **Complete Frontend** - Wallet UI, dashboard, key workflows
4. **Test Advanced Features** - Grace periods, governance, swaps in practice

### Short Term (Post-Mainnet)
1. **Community Onboarding** - Documentation, tutorials
2. **Mobile App** - Progressive Web App enhancements
3. **Analytics** - Dashboard for trust administrators
4. **IPFS Integration** - Decentralized evidence storage

### Long Term
1. **Multi-Chain** - Consider other chains
2. **Fiat Bridge** - Connect to traditional currency
3. **Advanced Governance** - Quadratic voting, delegation
4. **Reputation Marketplace** - Cross-trust verifier reputation

---

## Appendix: Key Constants

```rust
// Economic Constants
MINUTES_PER_KCHNG: 30          // Time standard
MIN_WORK_MINUTES: 15           // Minimum claim
DEFAULT_ANNUAL_RATE_BPS: 1200  // 12%

// Protocol Bounds
MIN_ANNUAL_RATE_BPS: 500       // 5% minimum
MAX_ANNUAL_RATE_BPS: 1500      // 15% maximum

// Stake Requirements
VERIFIER_STAKE: 100,000 KCHNG
ORACLE_STAKE: 500,000 KCHNG

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
- **Contract**: `packages/contracts/src/lib.rs`
- **Tests**: `packages/contracts/src/test.rs`, `tests/regression/`

---

**Document Status**: ✅ Complete | ⚠️ Needs Update for Reputation Decision
