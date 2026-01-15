# Domain-Aspect Reputation System Design

**Date:** 2026-01-14
**Status:** Draft
**Author:** KCHNG Design Team

---

## Executive Summary

This document describes an extension to the KCHNG reputation system that supports **domain-aspect dependent reputation** while maintaining full backward compatibility with the existing single-score system.

### Core Design Principles

| Principle | Description |
|-----------|-------------|
| **Backward Compatible** | Existing `reputation_score` remains unchanged and independent |
| **Client-Side Governance** | Domain-aspect CRUD managed by community leadership, not contract-enforced |
| **Neutral Default** | New aspects initialize at 500 (neutral) |
| **Optional Extension** | Aspects are additive; accounts may have zero, one, or many aspect scores |

### Problem Statement

The current reputation system uses a single scalar score (0-1000) per account. This fails to capture real-world nuance where someone may be:
- A reliable dinner guest but unreliable host
- A careful driver but distracted passenger
- An excellent employee but poor employer

### Solution Overview

Extend `VerifierData` with an optional `aspect_scores` map that stores domain-specific reputation scores, while keeping the existing `reputation_score` as an independent general trustworthiness signal.

---

## Architecture

### System Boundaries

```
┌─────────────────────────────────────────────────────────────────┐
│                         Client Layer                            │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │
│  │   Domain    │  │   Aspect    │  │   Scoring   │              │
│  │ Management  │  │  Scoring    │  │   Display   │              │
│  │   (CRUD)    │  │  (Update)   │  │   (UI)      │              │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘              │
│         │                │                │                      │
│         └────────────────┴────────────────┘                      │
│                          │                                       │
└──────────────────────────┼───────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Smart Contract Layer                       │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  VerifierData {                                         │   │
│  │    reputation_score: u32,           // General (0-1000) │   │
│  │    aspect_scores: Map<Bytes, u32>,  // Aspects (0-1000) │   │
│  │    ...                                                     │   │
│  │  }                                                       │   │
│  └─────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

### Key Design Decisions

| Decision | Rationale |
|----------|-----------|
| **Aspect CRUD is client-side** | Reduces contract complexity, allows rapid iteration, keeps storage costs minimal |
| **reputation_score independent** | Preserves existing semantics, avoids derivation complexity |
| **Neutral default (500)** | Fair starting point, neither positive nor negative bias |
| **Community leadership governance** | Aligns with KCHNG's trust-based governance model |

---

## Data Structures

### Contract Extension (Rust/Soroban)

```rust
/// Verifier data for work verification
#[derive(Clone)]
#[contracttype]
pub struct VerifierData {
    pub trust_id: Option<Address>,
    pub stake: U256,
    pub reputation_score: u32,            // UNCHANGED: General trust (0-1000)
    pub verified_claims: u32,
    pub rejected_claims: u32,
    pub fraud_reports: u32,
    // NEW: Optional aspect-specific scores
    pub aspect_scores: Map<Bytes, u32>,   // Domain string → score (0-1000)
}
```

### TypeScript Types Extension

```typescript
// packages/shared/src/types.ts

/**
 * Domain-aspect identifier for reputation scoring
 *
 * Examples: "dinner_guest", "dinner_host", "car_driver", "car_passenger",
 *           "work_employee", "work_employer", "verifier", "oracle"
 */
export type AspectDomain = string;

/**
 * Aspect-specific reputation score (0-1000)
 * 500 = neutral (default)
 * 0 = lowest reputation
 * 1000 = highest reputation
 */
export type AspectScore = number;

/**
 * Extended verifier data with aspect scores
 */
export interface VerifierData {
  trust_id: AccountId | null;
  stake: Amount;
  reputation_score: number;           // UNCHANGED: General trust
  verified_claims: number;
  rejected_claims: number;
  fraud_reports: number;
  // NEW: Optional aspect-specific scores
  aspect_scores?: Map<AspectDomain, AspectScore>;
}

/**
 * Domain-aspect metadata (client-side managed)
 */
export interface AspectDomainMetadata {
  domain: AspectDomain;
  name: string;                  // Human-readable name
  description: string;
  category: AspectCategory;
  created_by: AccountId;         // Trust leadership that created it
  created_at: Timestamp;
  is_active: boolean;
}

/**
 * Category of aspect domain
 */
export enum AspectCategory {
  Hospitality = "hospitality",       // dinner_guest, dinner_host
  Transportation = "transportation", // driver, passenger
  Employment = "employment",         // employee, employer
  Verification = "verification",     // verifier, oracle
  Community = "community",           // voter, proposer, governor
}

/**
 * Aspect score update request
 */
export interface AspectScoreUpdate {
  subject: AccountId;            // Account being scored
  domain: AspectDomain;
  delta: number;                 // Change to apply (positive or negative)
  reason?: string;               // Optional justification
  scored_by: AccountId;          // Account submitting the score
  timestamp: Timestamp;
}
```

---

## API Design

### Contract Functions

#### Read Operations (No Changes Required)

Existing read functions already return the full `VerifierData` struct, which will include `aspect_scores` after extension.

```rust
// Existing function - no signature change
pub fn get_verifier(env: &Env, verifier: Address) -> VerifierData {
    // Now returns aspect_scores if present
}
```

#### Write Operations (New)

```rust
/// Update an aspect score for a verifier
///
/// # Arguments
/// * `verifier` - The verifier whose aspect score is being updated
/// * `domain` - The aspect domain (e.g., "dinner_guest")
/// * `delta` - The change to apply (can be positive or negative)
/// * `scorer` - The account submitting this score update
///
/// # Behavior
/// - If aspect doesn't exist, initializes to 500 then applies delta
/// - Caps final score at [0, 1000]
/// - Requires auth from scorer
pub fn update_aspect_score(
    env: &Env,
    verifier: Address,
    domain: Bytes,
    delta: i32,
    scorer: Address,
) {
    scorer.require_auth();

    let mut verifier_data = get_verifier(env, verifier.clone());

    // Initialize to neutral if not present
    let current_score = verifier_data.aspect_scores
        .get(domain.clone())
        .unwrap_or(500);

    // Apply delta with bounds checking
    let new_score = (current_score as i32 + delta).clamp(0, 1000) as u32;

    verifier_data.aspect_scores.set(domain, new_score);
    env.storage().persistent().set(&verifier, &verifier_data);
}
```

### Client-Side Domain Management

Since domain CRUD is client-side, these operations are performed through the frontend/app layer:

#### Create Domain

```typescript
interface CreateDomainRequest {
  domain: AspectDomain;
  name: string;
  description: string;
  category: AspectCategory;
  created_by: AccountId; // Must be trust leadership
}

async function createAspectDomain(
  request: CreateDomainRequest
): Promise<AspectDomainMetadata> {
  // 1. Verify creator has trust leadership role
  // 2. Validate domain name uniqueness
  // 3. Store domain metadata (client-side storage or IPFS)
  // 4. Emit event for community awareness
}
```

#### Read Domains

```typescript
async function listAspectDomains(
  category?: AspectCategory
): Promise<AspectDomainMetadata[]> {
  // Retrieve from client-side storage
}
```

#### Update Domain

```typescript
async function updateAspectDomain(
  domain: AspectDomain,
  updates: Partial<Pick<AspectDomainMetadata, 'name' | 'description' | 'is_active'>>
): Promise<AspectDomainMetadata> {
  // Only domain creator or trust leadership can update
}
```

#### Delete Domain

```typescript
async function deleteAspectDomain(
  domain: AspectDomain
): Promise<void> {
  // Soft delete: mark is_active = false
  // Historical scores preserved
}
```

---

## Aspect Scoring Logic

### Score Initialization

When an aspect is first set for an account:

```typescript
function initializeAspectScore(
  currentScores: Map<AspectDomain, AspectScore>,
  domain: AspectDomain
): AspectScore {
  if (currentScores.has(domain)) {
    return currentScores.get(domain)!;
  }
  return 500; // Neutral default
}
```

### Score Update with Bounds

```typescript
function updateAspectScore(
  current: AspectScore,
  delta: number
): AspectScore {
  const newScore = current + delta;
  return Math.max(0, Math.min(1000, newScore));
}
```

### Example Score Progression

```
Initial state:     aspect_scores = {}

After first update: aspect_scores = { "dinner_guest": 500 }
                    delta = +30 (hosted great dinner)
                    result: aspect_scores = { "dinner_guest": 530 }

Second update:     aspect_scores = { "dinner_guest": 530 }
                    delta = -50 (guest arrived late, rude)
                    result: aspect_scores = { "dinner_guest": 480 }

New aspect added:  aspect_scores = { "dinner_guest": 480, "car_driver": 500 }
```

---

## Governance and Authorization

### Who Can Manage Domains?

| Role | Can Create Domain | Can Update Domain | Can Delete Domain |
|------|-------------------|-------------------|-------------------|
| **Trust Governor** | ✓ (within trust) | ✓ (within trust) | ✓ (within trust) |
| **Protocol Admin** | ✓ (global) | ✓ (global) | ✓ (global) |
| **Regular Member** | ✗ | ✗ | ✗ |

### Who Can Submit Scores?

| Role | Can Score Aspect |
|------|------------------|
| **Trust Governor** | ✓ (any aspect) |
| **Verifier** | ✓ (verification-related aspects) |
| **Oracle** | ✓ (grace period aspects) |
| **Regular Member** | Case-by-case (community-defined) |

### Dispute Resolution

Client-side governance layer should implement:

1. **Score appeal mechanism** - Accounts can contest unfair scores
2. **Review by trust leadership** - Governors review appeals
3. **Score adjustment** - Leadership can correct erroneous scores
4. **Audit trail** - All changes logged with reason and author

---

## Security Considerations

### Contract-Level Security

| Threat | Mitigation |
|--------|------------|
| **Score spam** | Rate limiting per scorer-subject pair |
| **Collusion** | Require minimum number of independent scores for visibility |
| **Self-scoring** | Block scorers from scoring themselves |
| **Extreme deltas** | Cap single-update delta (e.g., ±100) |

### Client-Side Security

| Threat | Mitigation |
|--------|------------|
| **Unauthorized domain creation** | Verify trust leadership role before creating |
| **Domain name collision** | Enforce uniqueness constraint |
| **Inactive domain abuse** | Soft delete preserves history, prevents new scores |

### Privacy Considerations

- **Aspect scores are public** (on-chain data)
- **Scorer identity is public** (required for auth)
- Communities can opt to create "private" aspects with restricted access

---

## Implementation Phases

### Phase 1: Contract Extension (Week 1)

- [ ] Extend `VerifierData` struct with `aspect_scores: Map<Bytes, u32>`
- [ ] Implement `update_aspect_score()` function
- [ ] Add bounds checking (0-1000)
- [ ] Add neutral initialization (500)
- [ ] Write unit tests
- [ ] Update snapshot tests

### Phase 2: Type Definitions (Week 1)

- [ ] Extend `VerifierData` in `packages/shared/src/types.ts`
- [ ] Add `AspectDomain`, `AspectScore` types
- [ ] Add `AspectCategory` enum
- [ ] Add `AspectDomainMetadata` interface
- [ ] Add `AspectScoreUpdate` interface

### Phase 3: Client-Side Domain Management (Week 2)

- [ ] Implement domain CRUD operations
- [ ] Add trust leadership authorization checks
- [ ] Create domain management UI components
- [ ] Add domain listing and search
- [ ] Implement soft delete

### Phase 4: Scoring Interface (Week 2)

- [ ] Implement score submission UI
- [ ] Add score history tracking
- [ ] Create dispute resolution interface
- [ ] Add score visualization (charts, trends)

### Phase 5: Integration and Testing (Week 3)

- [ ] End-to-end testing
- [ ] Security audit
- [ ] Performance testing (large aspect maps)
- [ ] User acceptance testing
- [ ] Documentation

---

## Storage Cost Analysis

### Per-Verifier Storage

| Field | Type | Approximate Size |
|-------|------|------------------|
| `reputation_score` | `u32` | 4 bytes |
| `aspect_scores` | `Map<Bytes, u32>` | Base + entries |
| Per aspect entry | key + value | ~32 + 4 bytes |

**Example**: An account with 10 aspects
- Base map overhead: ~100 bytes
- 10 aspects × 36 bytes: ~360 bytes
- Total: ~460 bytes additional storage

### Network Fee Impact

- Writing to `aspect_scores` map requires XDR serialization
- Estimated cost per aspect update: ~5,000-10,000 operations
- At ~100 ops/XDR: 50-100 XDR per update
- Acceptable for occasional scoring events

---

## User Experience

### Viewing Reputation

```
┌─────────────────────────────────────────────────────┐
│  Alice's Reputation                                  │
├─────────────────────────────────────────────────────┤
│  General: ████████░░ 820/1000                       │
│                                                      │
│  Aspect Scores:                                      │
│  ┌─────────────────────┬──────────────┬───────────┐ │
│  │ Domain              │ Score        │ Trend     │ │
│  ├─────────────────────┼──────────────┼───────────┤ │
│  │ Dinner Guest        │ ████████░ 850│ ↗ +20    │ │
│  │ Dinner Host         │ ████░░░░░ 420│ ↘ -30    │ │
│  │ Car Driver          │ █████████ 920│ → 0      │ │
│  │ Car Passenger       │ ██████░░░ 610│ ↗ +5     │ │
│  │ Work Verifier       │ ███████░░ 750│ ↗ +15    │ │
│  └─────────────────────┴──────────────┴───────────┘ │
│                                                      │
│  [View History] [Submit Score] [Dispute]            │
└─────────────────────────────────────────────────────┘
```

### Submitting a Score

```
┌─────────────────────────────────────────────────────┐
│  Submit Aspect Score for Bob                         │
├─────────────────────────────────────────────────────┤
│                                                      │
│  Domain: [Dinner Guest ▼]                            │
│                                                      │
│  Score Change:                                       │
│    ○ +100 (Excellent)                                │
│    ○ +50  (Very Good)                                │
│    ● +30  (Good)                       ← Selected    │
│    ○ +10  (Slightly Positive)                         │
│    ○ 0    (Neutral)                                  │
│    ○ -10  (Slightly Negative)                        │
│    ○ -30  (Poor)                                     │
│    ○ -50  (Very Poor)                                │
│    ○ -100 (Terrible)                                 │
│                                                      │
│  Reason (optional):                                   │
│  ┌─────────────────────────────────────────────┐    │
│  │ Arrived on time, brought dessert, great     │    │
│  │ conversation. Would host again!             │    │
│  └─────────────────────────────────────────────┘    │
│                                                      │
│  Estimated Impact: 500 → 530                         │
│                                                      │
│               [Cancel]  [Submit Score]               │
└─────────────────────────────────────────────────────┘
```

---

## Migration Strategy

### Existing Data

- Existing `VerifierData` records have no `aspect_scores` field
- Contract migration: Read existing data, write back with empty `aspect_scores` map
- Frontend: Handle missing `aspect_scores` gracefully (display as empty)

### Migration Steps

1. Deploy new contract with extended `VerifierData`
2. Run migration script to add empty `aspect_scores` to existing verifiers
3. Update frontend to use new types
4. Enable domain creation and scoring features

---

## Future Enhancements

### Potential Extensions

| Feature | Description | Complexity |
|---------|-------------|------------|
| **Aspect expiration** | Scores decay over time if not updated | Medium |
| **Cross-aspect bonuses** | High score in one aspect boosts related aspects | High |
| **Reputation badges** | Earn badges for aspect achievements | Low |
| **Aspect transfer** | Export/import reputation between contexts | Medium |
| **Privacy tiers** | Private aspects with restricted visibility | High |

### Research Questions

1. What is the optimal number of aspects for usability?
2. Should there be a maximum number of aspects per account?
3. How do aspect scores correlate with general reputation over time?
4. What prevents gaming through multiple aspect creation?

---

## Appendix: Example Scenarios

### Scenario 1: The Dinner Paradox

Alice is an excellent dinner guest but frequently cancels when hosting.

```
Initial:
  reputation_score: 700 (general)
  aspect_scores: {}

After 5 successful guest visits:
  reputation_score: 700 (unchanged)
  aspect_scores: { "dinner_guest": 850 }

After 3 cancelled hosting attempts:
  reputation_score: 700 (unchanged)
  aspect_scores: {
    "dinner_guest": 850,
    "dinner_host": 400
  }

Community decision: Still invite Alice to dinners, but don't ask her to host.
```

### Scenario 2: The Careful Driver

Bob is an exceptionally careful driver but gets car sick as a passenger.

```
Initial:
  reputation_score: 650
  aspect_scores: {}

After providing rides for 6 months:
  reputation_score: 650
  aspect_scores: { "car_driver": 920 }

After several uncomfortable passenger experiences:
  reputation_score: 650
  aspect_scores: {
    "car_driver": 920,
    "car_passenger": 520  // Slightly below neutral
  }

Community decision: Bob is the go-to driver, but he drives himself.
```

### Scenario 3: The Asymmetric Employer

Carol is an excellent employee but struggles as an employer.

```
Initial:
  reputation_score: 800
  aspect_scores: {}

As employee at community farm:
  reputation_score: 800
  aspect_scores: { "work_employee": 950 }

After starting her own business:
  reputation_score: 800
  aspect_scores: {
    "work_employee": 950,
    "work_employer": 450
  }

Community decision: Carol is great to hire, but not ready to employ others.
```

---

## References

- Original Reputation Gap Analysis: `/docs/2026-01-02_reputation-system-gap-analysis.md`
- Shared Types: `/packages/shared/src/types.ts`
- Contract Source: `/packages/contracts/src/lib.rs`

---

**Document Version:** 1.0
**Last Updated:** 2026-01-14
**Status:** Draft - Pending Community Review
