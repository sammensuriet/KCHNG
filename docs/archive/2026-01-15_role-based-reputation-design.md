# Role-Based Reputation System Design

**Date:** 2026-01-15
**Status:** Draft
**Author:** KCHNG Design Team

---

## Executive Summary

This document describes an extension to the KCHNG reputation system that supports **role-based reputation** through a three-level hierarchy: **Domain → Aspect → Role**. This system maintains full backward compatibility with the existing single-score system.

### Core Design Principles

| Principle | Description |
|-----------|-------------|
| **Backward Compatible** | Existing `reputation_score` remains unchanged and independent |
| **Client-Side Governance** | Aspect/role definitions managed by community leadership, not contract-enforced |
| **Neutral Default** | New roles initialize at 500 (neutral) |
| **Optional Extension** | Role scores are additive; accounts may have zero, one, or many role scores |

### Problem Statement

The current reputation system uses a single scalar score (0-1000) per account. This fails to capture real-world nuance where someone may be:
- A reliable dinner guest but unreliable host
- A careful driver but distracted passenger
- An excellent employee but poor employer

### Solution Overview

Extend `VerifierData` with an optional `aspect_scores` map that stores **role-based** reputation scores using compound keys (`"aspect:role"`), while keeping the existing `reputation_score` as an independent general trustworthiness signal.

### Hierarchy: Domain → Aspect → Role

```
Domain (Transportation)
  └── Aspect (Ride-sharing)
      ├── Role (Driver) → Score: 920
      └── Role (Passenger) → Score: 450

Domain (Hospitality)
  └── Aspect (Dining)
      ├── Role (Guest) → Score: 850
      └── Role (Host) → Score: 400

Domain (Employment)
  ├── Aspect (Work)
  │   ├── Role (Employee) → Score: 950
  │   └── Role (Employer) → Score: 550
  └── Aspect (Management)
      ├── Role (Manager) → Score: 720
      └── Role (Report) → Score: 680
```

---

## Architecture

### System Boundaries

```
┌─────────────────────────────────────────────────────────────────┐
│                         Client Layer                            │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │
│  │  Aspect &   │  │    Role     │  │   Scoring   │              │
│  │  Role       │  │  Scoring    │  │   Display   │              │
│  │ Management  │  │  (Update)   │  │   (UI)      │              │
│  │   (CRUD)    │  │             │  │             │              │
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
│  │    aspect_scores: Map<Bytes, u32>,  // "aspect:role"    │   │
│  │    ...                                                     │   │
│  │  }                                                       │   │
│  └─────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

### Key Design Decisions

| Decision | Rationale |
|----------|-----------|
| **3-level hierarchy** | Domain → Aspect → Role provides maximum clarity and flexibility |
| **Compound key format** | `"aspect:role"` allows simple storage while maintaining semantic clarity |
| **Aspect/role CRUD is client-side** | Reduces contract complexity, allows rapid iteration |
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
    pub reputation_score: u32,     // UNCHANGED: General trust (0-1000)
    pub verified_claims: u32,
    pub rejected_claims: u32,
    pub fraud_reports: u32,
    /// Role-based scores (compound key "aspect:role" → score 0-1000)
    /// Default for new roles is 500 (neutral)
    /// Examples: "dining:guest" → 850, "ride_sharing:driver" → 920
    pub aspect_scores: Map<Bytes, u32>,
}
```

### TypeScript Types Extension

```typescript
// packages/shared/src/types.ts

/**
 * High-level domain for aspect grouping
 */
export enum AspectDomain {
  Hospitality = "hospitality",       // dining, hosting, events
  Transportation = "transportation", // ride-sharing, car rental, delivery
  Employment = "employment",         // work, management, freelance
  Verification = "verification",     // work verification, oracle services
  Community = "community",           // voting, governance, proposals
}

/**
 * Specific role within an aspect
 * Examples: "driver", "passenger", "guest", "host"
 */
export type AspectRole = string;

/**
 * Aspect identifier within a domain
 * Examples: "ride_sharing", "dining", "freelance_work"
 */
export type Aspect = string;

/**
 * Role-based reputation score (0-1000)
 * 500 = neutral (default for new roles)
 */
export type RoleScore = number;

/**
 * Compound key type for role-based scoring
 * Format: "aspect:role" → score
 */
export type RoleScoreKey = `${Aspect}:${AspectRole}`;

/**
 * Verifier data for work verification
 */
export interface VerifierData {
  trust_id: AccountId | null;
  stake: Amount;
  reputation_score: number;        // 0-1000 (general trust, independent of roles)
  verified_claims: number;
  rejected_claims: number;
  fraud_reports: number;
  /**
   * Optional role-based scores (aspect:role → score)
   * Examples:
   *   "dining:guest" → 850
   *   "dining:host" → 400
   *   "ride_sharing:driver" → 920
   */
  role_scores?: Record<RoleScoreKey, RoleScore>;
}

/**
 * Aspect metadata (client-side managed)
 *
 * Defines an aspect (activity) within a domain, including which roles
 * are available for scoring.
 */
export interface AspectMetadata {
  aspect: Aspect;                 // "ride_sharing"
  name: string;                  // "Ride-sharing"
  description: string;           // "Shared transportation services"
  domain: AspectDomain;          // AspectDomain.Transportation
  roles: AspectRole[];           // ["driver", "passenger"]
  created_by: AccountId;         // Trust leadership that created this aspect
  created_at: Timestamp;
  is_active: boolean;
}

/**
 * Role score update request
 */
export interface RoleScoreUpdate {
  subject: AccountId;            // Account being scored
  aspect: Aspect;                // "dining"
  role: AspectRole;              // "guest"
  delta: number;                 // Change to apply (positive or negative)
  reason?: string;               // Optional justification
  scored_by: AccountId;          // Account submitting this score
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
pub fn get_verifier(env: Env, verifier: Address) -> VerifierData {
    // Now returns aspect_scores if present
}
```

#### Write Operations (New)

```rust
/// Update a role-based score for a verifier
///
/// Role-based reputation allows for context-specific scoring.
/// Hierarchy: Domain → Aspect → Role
///
/// # Arguments
/// * `verifier` - The verifier whose role score is being updated
/// * `role_key` - Compound key "aspect:role" (e.g., "dining:guest", "ride_sharing:driver")
/// * `delta` - The change to apply (positive or negative, e.g., +30, -50)
/// * `scorer` - The account submitting this score update (must authenticate)
///
/// # Behavior
/// - If role doesn't exist, initializes to 500 (neutral) then applies delta
/// - Caps final score at [0, 1000]
/// - Requires auth from scorer
pub fn update_role_score(
    env: Env,
    verifier: Address,
    role_key: Bytes,
    delta: i32,
    scorer: Address,
) {
    scorer.require_auth();

    // Prevent self-scoring
    if scorer == verifier {
        panic!("Cannot score yourself");
    }

    // Get existing verifier data - map may not exist yet
    let verifiers: Map<Address, VerifierData> =
        env.storage().persistent().get(&KEY_VERIFIERS).unwrap_or(Map::new(&env));

    if !verifiers.contains_key(verifier.clone()) {
        panic!("Verifier not found");
    }

    let mut verifier_data = verifiers.get(verifier.clone()).unwrap();

    // Get current score, defaulting to neutral (500) if not present
    let current_score = verifier_data.aspect_scores
        .get(role_key.clone())
        .unwrap_or(500);

    // Apply delta with bounds checking [0, 1000]
    let new_score = (current_score as i32 + delta).clamp(0, 1000) as u32;

    // Update the role score
    verifier_data.aspect_scores.set(role_key, new_score);

    // Get mutable map and update
    let mut verifiers: Map<Address, VerifierData> =
        env.storage().persistent().get(&KEY_VERIFIERS).unwrap_or(Map::new(&env));
    verifiers.set(verifier, verifier_data);
    env.storage().persistent().set(&KEY_VERIFIERS, &verifiers);
}
```

### Client-Side Aspect Management

Since aspect and role definitions are client-side, these operations are performed through the frontend/app layer:

#### Create Aspect

```typescript
interface CreateAspectRequest {
  aspect: Aspect;                // "ride_sharing"
  name: string;                  // "Ride-sharing"
  description: string;           // "Shared transportation services"
  domain: AspectDomain;          // AspectDomain.Transportation
  roles: AspectRole[];           // ["driver", "passenger"]
  created_by: AccountId;         // Must be trust leadership
}

async function createAspect(
  request: CreateAspectRequest
): Promise<AspectMetadata> {
  // 1. Verify creator has trust leadership role
  // 2. Validate aspect name uniqueness within domain
  // 3. Validate roles are provided
  // 4. Store aspect metadata (client-side storage or IPFS)
  // 5. Emit event for community awareness
}
```

#### Read Aspects

```typescript
async function listAspects(
  domain?: AspectDomain
): Promise<AspectMetadata[]> {
  // Retrieve from client-side storage
}
```

#### Update Aspect

```typescript
async function updateAspect(
  aspect: Aspect,
  updates: Partial<Pick<AspectMetadata, 'name' | 'description' | 'roles' | 'is_active'>>
): Promise<AspectMetadata> {
  // Only aspect creator or trust leadership can update
}
```

#### Delete Aspect

```typescript
async function deleteAspect(
  aspect: Aspect
): Promise<void> {
  // Soft delete: mark is_active = false
  // Historical role scores preserved
}
```

---

## Role Scoring Logic

### Score Initialization

When a role is first set for an account:

```typescript
function initializeRoleScore(
  currentScores: Record<RoleScoreKey, RoleScore>,
  roleKey: RoleScoreKey
): RoleScore {
  if (currentScores[roleKey] !== undefined) {
    return currentScores[roleKey];
  }
  return 500; // Neutral default
}
```

### Score Update with Bounds

```typescript
function updateRoleScore(
  current: RoleScore,
  delta: number
): RoleScore {
  const newScore = current + delta;
  return Math.max(0, Math.min(1000, newScore));
}
```

### Example Score Progression

```
Initial state:     role_scores = {}

After first update: role_scores = { "dining:guest": 500 }
                    delta = +30 (hosted great dinner)
                    result: role_scores = { "dining:guest": 530 }

Second update:     role_scores = { "dining:guest": 530 }
                    delta = -50 (arrived late, rude)
                    result: role_scores = { "dining:guest": 480 }

New role added:    role_scores = {
                      "dining:guest": 480,
                      "ride_sharing:driver": 500
                    }
```

---

## Governance and Authorization

### Who Can Manage Aspects?

| Role | Can Create Aspect | Can Update Aspect | Can Delete Aspect |
|------|-------------------|-------------------|-------------------|
| **Trust Governor** | ✓ (within trust) | ✓ (within trust) | ✓ (within trust) |
| **Protocol Admin** | ✓ (global) | ✓ (global) | ✓ (global) |
| **Regular Member** | ✗ | ✗ | ✗ |

### Who Can Submit Scores?

| Role | Can Score Role |
|------|----------------|
| **Trust Governor** | ✓ (any role) |
| **Verifier** | ✓ (verification-related roles) |
| **Oracle** | ✓ (grace period roles) |
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
| **Unauthorized aspect creation** | Verify trust leadership role before creating |
| **Aspect name collision** | Enforce uniqueness constraint (domain:aspect) |
| **Inactive aspect abuse** | Soft delete preserves history, prevents new scores |

### Privacy Considerations

- **Role scores are public** (on-chain data)
- **Scorer identity is public** (required for auth)
- Communities can opt to create "private" aspects with restricted access

---

## Implementation Phases

### Phase 1: Contract Extension (Week 1)

- [x] Extend `VerifierData` struct with `aspect_scores: Map<Bytes, u32>`
- [x] Implement `update_role_score()` function
- [x] Add bounds checking (0-1000)
- [x] Add neutral initialization (500)
- [x] Write unit tests
- [x] Update snapshot tests

### Phase 2: Type Definitions (Week 1)

- [x] Extend `VerifierData` in `packages/shared/src/types.ts`
- [x] Add `AspectDomain`, `Aspect`, `AspectRole` types
- [x] Add `AspectMetadata` interface
- [x] Add `RoleScoreUpdate` interface
- [x] Add `RoleScoreKey` type

### Phase 3: Client-Side Aspect Management (Week 2)

- [ ] Implement aspect CRUD operations
- [ ] Add trust leadership authorization checks
- [ ] Create aspect management UI components
- [ ] Add aspect listing and search
- [ ] Implement soft delete

### Phase 4: Scoring Interface (Week 2)

- [ ] Implement score submission UI
- [ ] Add score history tracking
- [ ] Create dispute resolution interface
- [ ] Add score visualization (charts, trends)

### Phase 5: Integration and Testing (Week 3)

- [ ] End-to-end testing
- [ ] Security audit
- [ ] Performance testing (large role maps)
- [ ] User acceptance testing
- [ ] Documentation

---

## Storage Cost Analysis

### Per-Verifier Storage

| Field | Type | Approximate Size |
|-------|------|------------------|
| `reputation_score` | `u32` | 4 bytes |
| `aspect_scores` | `Map<Bytes, u32>` | Base + entries |
| Per role entry | key + value | ~32 + 4 bytes |

**Example**: An account with 10 roles
- Base map overhead: ~100 bytes
- 10 roles × 36 bytes: ~360 bytes
- Total: ~460 bytes additional storage

### Network Fee Impact

- Writing to `aspect_scores` map requires XDR serialization
- Estimated cost per role update: ~5,000-10,000 operations
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
│  Role Scores:                                        │
│  ┌─────────────────────┬──────────────┬───────────┐ │
│  │ Aspect:Role         │ Score        │ Trend     │ │
│  ├─────────────────────┼──────────────┼───────────┤ │
│  │ Dining:Guest        │ ████████░ 850│ ↗ +20    │ │
│  │ Dining:Host         │ ████░░░░░ 400│ ↘ -30    │ │
│  │ Ride-sharing:Driver │ █████████ 920│ → 0      │ │
│  │ Ride-sharing:Pass.  │ ██████░░░ 610│ ↗ +5     │ │
│  │ Work:Employee       │ ███████░░ 750│ ↗ +15    │ │
│  └─────────────────────┴──────────────┴───────────┘ │
│                                                      │
│  [View History] [Submit Score] [Dispute]            │
└─────────────────────────────────────────────────────┘
```

### Submitting a Score

```
┌─────────────────────────────────────────────────────┐
│  Submit Role Score for Bob                            │
├─────────────────────────────────────────────────────┤
│                                                      │
│  Aspect: [Dining ▼]                                  │
│  Role:   [Guest ▼]                                   │
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
- Frontend: Handle missing `role_scores` gracefully (display as empty)

### Migration Steps

1. Deploy new contract with extended `VerifierData`
2. Run migration script to add empty `aspect_scores` to existing verifiers
3. Update frontend to use new types
4. Enable aspect creation and scoring features

---

## Future Enhancements

### Potential Extensions

| Feature | Description | Complexity |
|---------|-------------|------------|
| **Role expiration** | Scores decay over time if not updated | Medium |
| **Cross-role bonuses** | High score in one role boosts related roles | High |
| **Reputation badges** | Earn badges for role achievements | Low |
| **Role transfer** | Export/import reputation between contexts | Medium |
| **Privacy tiers** | Private roles with restricted visibility | High |

### Research Questions

1. What is the optimal number of roles per account for usability?
2. Should there be a maximum number of roles per account?
3. How do role scores correlate with general reputation over time?
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
  aspect_scores: {
    "dining:guest": 850
  }

After 3 cancelled hosting attempts:
  reputation_score: 700 (unchanged)
  aspect_scores: {
    "dining:guest": 850,
    "dining:host": 400
  }

Community decision: Still invite Alice to dinners, but don't ask her to host.
```

### Scenario 2: The Careful Driver

Bob is an exceptionally careful driver but gets carsick as a passenger.

```
Initial:
  reputation_score: 650
  aspect_scores: {}

After providing rides for 6 months:
  reputation_score: 650
  aspect_scores: {
    "ride_sharing:driver": 920
  }

After several uncomfortable passenger experiences:
  reputation_score: 650
  aspect_scores: {
    "ride_sharing:driver": 920,
    "ride_sharing:passenger": 520  // Slightly below neutral
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
  aspect_scores: {
    "employment:employee": 950
  }

After starting her own business:
  reputation_score: 800
  aspect_scores: {
    "employment:employee": 950,
    "employment:employer": 450
  }

Community decision: Carol is great to hire, but not ready to employ others.
```

---

## Type Naming Summary

| Old Name | New Name | Description |
|----------|----------|-------------|
| `AspectCategory` | `AspectDomain` | High-level domain (e.g., Transportation) |
| `AspectDomain` (type) | `Aspect` | Specific activity within domain (e.g., Ride-sharing) |
| (new) | `AspectRole` | Role within an aspect (e.g., Driver, Passenger) |
| `AspectScore` | `RoleScore` | Score value for a specific role |
| `AspectDomainMetadata` | `AspectMetadata` | Metadata about an aspect |
| `AspectScoreUpdate` | `RoleScoreUpdate` | Update request for a role score |
| (new) | `RoleScoreKey` | Compound key type: `"${Aspect}:${AspectRole}"` |

---

## References

- Original Design: `/docs/2026-01-14_domain-aspect-reputation-design.md` (superseded)
- Shared Types: `/packages/shared/src/types.ts`
- Contract Source: `/packages/contracts/src/lib.rs`

---

**Document Version:** 2.0
**Last Updated:** 2026-01-15
**Status:** Draft - Pending Community Review
**Changes from v1.0:** Complete refactoring to 3-level hierarchy (Domain → Aspect → Role)
