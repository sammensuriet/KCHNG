# Stellar Mutual Credit System: Dynamics & Game Theory Analysis

**Date:** 2026-01-02
**Contract ID:** `CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX`

---

## Part 1: Trust, Reputation & Token Dynamics

### Reputation System Architecture

#### Verifier Reputation
```rust
struct VerifierData {
    stake: U256,              // Skin in the game
    reputation_score: u32,     // 0-100 scale
    verified_claims: u32,       // Good deeds
    rejected_claims: u32,      // May be appropriate
    fraud_reports: u32,        // Bad deeds
    trust_id: Option<Address>, // Which trust they belong to
}
```

**How Reputation is Earned:**
| Action | Reputation Change | Mechanism |
|--------|-------------------|------------|
| Approve valid work claim | +1 to +5 | Based on work quality/verification accuracy |
| Reject invalid claim | +1 to +5 | Protecting system from fraud |
| Approve fraudulent claim | -10 to -50 | Allowing bad work |
| Reject valid claim | -5 to -20 | Blocking good work |
| Fraud report confirmed | -50 to -100 | Malicious behavior |
| Consistency bonus | +0.1 per claim | Long-term reliability |

#### Oracle Reputation
```rust
struct OracleData {
    stake: U256,
    reputation_score: u32,
    grace_periods_granted: u32,
}
```

**How Oracle Reputation is Earned:**
| Action | Reputation Change |
|--------|-------------------|
| Grant appropriate grace period | +1 to +10 |
| Grant unnecessary grace period | -5 to -20 |
| Grant during real emergency | +10 to +30 |
| Fraudulent grants (caught) | -100 (removal) |

### Token Acquisition Methods

#### 1. Work Contribution (Primary)
```
submit_work_claim → verifiers approve → contribution_hours ↑
```
- Worker submits claim with evidence
- 2+ verifiers assigned (from same or different trust)
- If approved: worker earns hours
- `multiplier` affects reward (BasicCare=1.0, SkilledCare=1.5, EmergencyCare=2.0)

#### 2. Direct Transfer
```rust
transfer(from: Address, to: Address, amount: U256)
```
- Peer-to-peer within same trust
- No fees (governed by trust rules)
- Limited by sender's balance

#### 3. Cross-Trust Exchange
```rust
cross_trust_swap(from: Address, dest_trust: Address, amount: U256)
calculate_exchange_rate(source_trust: Address, dest_trust: Address) -> u64
```
- Exchange tokens between different trusts
- Rate based on relative demurrage rates
- May include exchange fee

#### 4. Minting (Admin Only)
```rust
mint(admin: Address, to: Address, amount: U256)
```
- Only contract admin can mint
- For initial supply or emergency situations
- Subject to governance approval

### Demurrage Mechanics

#### How It Works
```
Annual demurrage rate: 5% (500 bps)
Demurrage period: 365 days (applied daily or monthly)

Daily calculation:
  daily_rate = 0.05 / 365 = 0.000137 (0.0137% per day)

Example with 1000 tokens:
  Day 1:  1000 - (1000 × 0.000137) = 999.863
  Day 30: 996.037
  Day 90: 988.194
  Day 365: 950.000 (5% lost over year)
```

#### Economic Implications

| Effect | Description |
|--------|-------------|
| **Anti-Hoarding** | Holding tokens loses value over time |
| **Circulation Incentive** | Encourages spending and investment |
| **Work Incentive** | Must contribute to offset demurrage |
| **Time Preference** | Future value discounted, present value higher |

### Staking System

#### Verifier Staking
```
register_verifier(verifier, trust_id)
```
- Requires minimum stake (e.g., 1000 tokens)
- Stake locked while registered
- Slashed for fraud: lose portion or all stake
- Earns fees from verifying claims

#### Oracle Staking
```
register_oracle(oracle)
```
- Requires higher stake (e.g., 5000 tokens)
- Stake locked indefinitely
- Slashed for abusing grace period power
- Earns reputation for good stewardship

### Cross-Trust Exchange Rate Dynamics

#### Rate Calculation Factors
```rust
calculate_exchange_rate(source_trust, dest_trust) -> u64
```

**Factors affecting rate:**
1. **Relative Demurrage Rates**
   - Higher demurrage = faster devaluation = lower value
   - Example: 12% trust tokens worth less than 5% trust tokens

2. **Demand/Supply**
   - Higher member count = higher demand
   - Total supply in each trust

3. **Trust Stability**
   - Age of trust
   - Governance effectiveness
   - Default rate

**Example Exchange:**
```
TestCommunity: 5% annual, 365 days, 2 members, 1M supply
Rural Health: 8% annual, 30 days, 1 member, unknown supply

Rural Health devalues 2.67x faster than TestCommunity
(0.08 / 30 days) vs (0.05 / 365 days)

Exchange rate might be: 1 Rural Health = 0.4 TestCommunity tokens
```

### The Economic Flow

```
┌─────────────────────────────────────────────────────────────┐
│                     ECONOMIC CICLUS                        │
└─────────────────────────────────────────────────────────────┘

                    WORK CONTRIBUTION
                           ↓
            ┌──────────────────────────┐
            │  submit_work_claim()    │
            └──────────────────────────┘
                           ↓
                    VERIFIER REVIEW
                  (requires 2+ verifiers)
                           ↓
              ┌────────────────────────────┐
              │  approve_work_claim()     │
              │  contribution_hours ↑    │
              │  OR                      │
              │  reject_work_claim()      │
              │  reputation impact        │
              └────────────────────────────┘
                           ↓
                    TOKENS EARNED
                           ↓
        ┌──────────────────┬─────────────────────┐
        │   HOLD/SPEND      │   CROSS-TRUST      │
        │   (demurrage)     │   (exchange)       │
        └──────────────────┴─────────────────────┘
                  ↑                ↑
                  │                │
            VALUE DECAYS      ARBITRAGE OPPORTUNITIES
```

---

## Part 2: "Tit for 2 Tats" - Game Theory Primer

### Background: Iterated Prisoner's Dilemma

The **Prisoner's Dilemma** is a classic game theory scenario:

| | Cooperate | Defect |
|---|-----------|--------|
| **Cooperate** | (3, 3) | (0, 5) |
| **Defect** | (5, 0) | (1, 1) |

**Nash Equilibrium:** Both defect (1, 1) - even though both cooperating would be better (3, 3).

### Tit for Tat (TFT)

Developed by Anatol Rapoport, TFT is a strategy for iterated prisoner's dilemma:

**Rule:** "Do what your opponent did last round"

```
Round 1: Cooperate (nice opening)
Round 2: Copy opponent's move from Round 1
Round 3: Copy opponent's move from Round 2
...
```

**Properties:**
- **Nice:** Starts with cooperation
- **Provocable:** Retaliates against defection
- **Forgiving:** Returns to cooperation if opponent does
- **Clear:** Easy for opponent to understand

**Success:** TFT won Axelrod's tournaments, beating much more complex strategies.

### Tit for 2 Tats (TF2T)

A **more forgiving** variation:

**Rule:** "Defect only if opponent defected TWO times in a row"

```
Round 1: Cooperate
Round 2: Cooperate (even if opponent defected)
Round 3: Cooperate (if opponent cooperated once in 1-2)
          Defect (if opponent defected BOTH times)
...
```

**Properties:**
- **More forgiving** than TFT
- **Tolerates occasional mistakes**
- **Better in noisy environments** (where defection might be accidental)
- **Still provocable** against consistent defection

**When TF2T does better than TFT:**
- When the opponent occasionally defects by mistake
- In "noisy" communication environments
- When rebuilding trust after conflict

### TFT vs TF2T Example

| Round | Opponent | TFT Response | TF2T Response |
|-------|----------|--------------|----------------|
| 1 | Defect | Cooperate | Cooperate |
| 2 | Cooperate | Defect | Cooperate |
| 3 | Defect | Cooperate | Cooperate |
| 4 | Cooperate | Defect | Cooperate |
| 5 | Cooperate | Cooperate | Cooperate |

**Result with TFT:** Cycle of D-C-D-C...
**Result with TF2T:** Returns to cooperation at Round 5

---

## Part 3: Experiment Design - TFT/TF2T in Mutual Credit System

### Objective

Evaluate how the Stellar mutual credit contract performs under game-theoretical stress with mixed "good" (cooperative) and "bad" (defective) actors using TFT and TF2T strategies.

### Experiment Parameters

#### Environment
- **Network:** Stellar Testnet
- **Contract:** CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX
- **Trust:** Create "GameTheoryTest" trust
- **Duration:** 100 rounds of interactions
- **Demurrage:** 5% annual, 90-day periods

#### Actors

| Type | Strategy | Behavior | Quantity |
|------|----------|----------|----------|
| **Always Cooperate (AC)** | Pure cooperator | Always approves claims | 2 |
| **Always Defect (AD)** | Pure defector | Always rejects claims | 2 |
| **Tit for Tat (TFT)** | Reciprocal | Mirrors last move | 4 |
| **Tit for 2 Tats (TF2T)** | Forgiving | Defects only after 2 consecutive defections | 4 |
| **Random (RAND)** | Random | 50/50 approve/reject | 2 |
| **Grim Trigger (GRIM)** | Vindictive | Defects forever after first defection | 2 |

**Total:** 16 verifiers

#### Workers
- 8 workers submitting work claims
- 100 work claims per simulation
- Random work type (BasicCare, SkilledCare, Training, EmergencyCare)
- Random "quality" (honest or fraudulent) - 10% fraudulent

### Simulation Design

#### Phase 1: Setup
```rust
// Create test trust with specific parameters
register_trust(
    governor: admin,
    name: "GameTheoryTest",
    annual_rate_bps: 500,  // 5%
    demurrage_period_days: 90  // Faster cycle for experiment
)

// Register all verifiers
for each verifier in [AC, AD, TFT, TF2T, RAND, GRIM]:
    register_verifier(verifier, "GameTheoryTest")
```

#### Phase 2: Simulation Loop
```python
for round in range(100):
    for worker in workers:
        # Worker submits claim
        claim_id = submit_work_claim(
            worker=worker,
            work_type=random(),
            minutes=random(30, 480),
            evidence_hash=generate_evidence()
        )

        # Assign 2 random verifiers
        verifiers = random.sample(verifier_list, 2)

        # Each verifier reviews and votes
        for verifier in verifiers:
            if worker.is_fraudulent:
                # Workers who submit fraudulent work occasionally
                if verifier.strategy == "AC":
                    approve()
                elif verifier.strategy == "AD":
                    reject()
                elif verifier.strategy == "TFT":
                    # Check what other verifier did last time
                    if last_interaction(verifier) == "approve":
                        approve()
                    else:
                        reject()
                # ... etc
            else:
                # Honest work - should be approved
                if verifier.strategy == "AC":
                    approve()
                elif verifier.strategy == "AD":
                    reject()
                # ... etc
```

#### Phase 3: Metrics Tracking

```rust
// Track for each verifier
struct VerifierStats {
    total_reviews: u32,
    approvals: u32,
    rejections: u32,
    reputation_score: u32,
    earnings: u64,  // from verification fees
    final_reputation: u32,
}

// Track for each worker
struct WorkerStats {
    claims_submitted: u32,
    claims_approved: u32,
    claims_rejected: u32,
    final_balance: u64,
    final_hours: u64,
    fraudulent_claims: u32,
}
```

### Expected Outcomes by Strategy

#### Always Cooperate (AC)
- **Approvals:** ~90% (approves everything, including some fraud)
- **Reputational Impact:**
  - Honest approvals: +reputation
  - Fraud approvals: -reputation
  - Net: Slightly negative (if fraud detected)
- **Vulnerability:** Exploited by AD and GRIM

#### Always Defect (AD)
- **Approvals:** ~0% (rejects everything)
- **Reputational Impact:**
  - Honest rejections: -reputation
  - Fraud rejections: +reputation
  - Net: Negative (rejects valid work)
- **Isolation:** Others stop cooperating with them

#### Tit for Tat (TFT)
- **Approvals:** Depends on history
- **Reputational Impact:**
  - Reciprocates cooperation → stable relationships
  - Retaliates against defection
  - Can get stuck in cycles
- **Vulnerability:** GRIM (once defect, forever defect)
- **Strengths:** Works well with TFT, TF2T

#### Tit for 2 Tats (TF2T)
- **Approvals:** More forgiving
- **Reputational Impact:**
  - Tolerates accidental defection
  - Returns to cooperation faster
  - Better in noisy environments
- **Advantages:** Resistant to Grim cycles
- **Expected:** Highest reputation in mixed environments

#### Grim Trigger (GRIM)
- **Approvals:** Cooperative until first defection, then always defect
- **Reputational Impact:**
  - Initially cooperative
  - After trigger: always defects (even if other returns to cooperate)
  - Can be exploited
- **Vulnerability:** TFT can trigger it accidentally

### Hypotheses

#### H1: TF2T Dominance
**Hypothesis:** TF2T verifiers will achieve highest reputation scores in mixed environments.

**Reasoning:** TF2T is more forgiving than TFT, allowing it to:
- Recover from accidental defections
- Not get permanently triggered by GRIM
- Maintain cooperation with TFT players
- Not be exploited by AD (still retaliates, just after 2)

#### H2: TFT Cycles with GRIM
**Hypothesis:** TFT players will suffer when paired with GRIM.

**Scenario:**
```
Round 1: GRIM cooperates, TFT cooperates
Round 2: TFT accidentally defects (10% noise)
Round 3: GRIM defects forever, TFT defects
Round 4+: TFT keeps defecting (mirroring GRIM)
```

**Outcome:** Both lock in defection, reputation suffers.

#### H3: AC and AD Punished
**Hypothesis:** Pure strategies (AC and AD) will have lowest final reputation.

**Reasoning:**
- AC: Approves fraud → reputation destroyed
- AD: Rejects valid work → reputation destroyed
- System rewards nuanced judgment

#### H4: Reciprocity Maximizes Reputation
**Hypothesis:** TFT-like strategies (reciprocal cooperators) achieve best balance of:
- Approving honest work
- Rejecting fraudulent work
- Maintaining cooperative relationships

### Success Metrics

| Metric | Target |
|--------|--------|
| **TF2T final reputation** | Highest among all strategies |
| **AC final reputation** | Low (exploited by fraud) |
| **AD final reputation** | Lowest (rejects valid work) |
| **TFT final reputation** | High (but hurt by GRIM) |
| **GRIM final reputation** | Low (fewer cooperation partners) |

### Implementation Steps

#### Step 1: Contract Deployment (If Unlocked)
```bash
# 1. Fund accounts
soroban keys generate game_theory_admin --fund
soroban keys generate test_worker_1 --fund
# ... (16 verifiers + 8 workers)

# 2. Create trust
soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account game_theory_admin \
  --network testnet \
  -- register_trust \
    --governor $ADMIN \
    --name "GameTheoryTest" \
    --annual_rate_bps 500 \
    --demurrage_period_days 90
```

#### Step 2: Simulation Script
```python
#!/usr/bin/env python3
import asyncio
import random
from stellar_sdk import Server, Keypair

class Verifier:
    def __init__(self, name, strategy, keypair):
        self.name = name
        self.strategy = strategy  # AC, AD, TFT, TF2T, GRIM, RAND
        self.keypair = keypair
        self.last_moves = {}  # Track what others did

    def decide(self, other_verifier, claim_quality):
        if self.strategy == "AC":
            return "approve"

        elif self.strategy == "AD":
            return "reject"

        elif self.strategy == "TFT":
            last = self.last_moves.get(other_verifier, "cooperate")
            return "approve" if last == "approve" else "reject"

        elif self.strategy == "TF2T":
            # Count consecutive defections
            last_moves = self.last_moves.get(other_verifier, [])
            consecutive_defections = 0
            for move in reversed(last_moves[-2:]):
                if move == "reject":
                    consecutive_defections += 1
                else:
                    break
            return "reject" if consecutive_defections >= 2 else "approve"

        elif self.strategy == "GRIM":
            if self.defected_by(other_verifier):
                return "reject"
            return "approve"

        elif self.strategy == "RAND":
            return random.choice(["approve", "reject"])

async def run_simulation():
    # Setup
    contract_id = "CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX"
    server = Server("https://soroban-testnet.stellar.org")

    # Simulation loop
    for round in range(100):
        for worker in workers:
            # Submit work claim
            claim_id = await submit_claim(worker, ...)

            # Assign verifiers
            verifiers = random.sample(all_verifiers, 2)

            # Get decisions
            decisions = {}
            for verifier in verifiers:
                decision = verifier.decide(
                    other_verifier=v1 if v2 == verifier else v2,
                    claim_quality=worker.quality
                )
                decisions[verifier] = decision

            # Count approvals
            approvals = sum(1 for d in decisions.values() if d == "approve")

            if approvals >= 2:
                await approve_claim(claim_id, approvals)

            # Update verifiers' memory
            for verifier in verifiers:
                for other in verifiers:
                    if other != verifier:
                        verifier.last_moves[other] = decisions[other]

    # Report results
    print_reputation_scores()
```

### Expected Results Summary

| Strategy | Final Reputation | Reasoning |
|----------|-----------------|------------|
| **TF2T** | 🥇 Highest | Forgiving, cooperative, resists exploitation |
| **TFT** | 🥈 High | Cooperative but vulnerable to GRIM |
| **RAND** | ⚖️ Medium | Random walk, might fail up or down |
| **GRIM** | ⬇️ Low-Medium | Fewer partners once triggered |
| **AC** | ⬇️ Low | Approves fraud, loses reputation |
| **AD** | ⬇️ Lowest | Rejects valid work, loses reputation |

### Key Insights

1. **Reciprocity Wins:** TFT/TF2T outperform pure strategies
2. **Forgiveness Helps:** TF2T > TFT in noisy/accident-prone environments
3. **Grim Tragedy:** GRIM triggers create permanent defection cycles
4. **System Resilience:** Mixed strategies make system robust against bad actors

---

## Part 4: Real-World Implications

### For Trust Communities

1. **Diverse Verifier Pool:**
   - Don't use only TFT verifiers (vulnerable to GRIM)
   - Include TF2T for forgiveness
   - Remove always-defect verifiers through reputation system

2. **Reputation as Signal:**
   - High reputation → more verification assignments → more fees
   - Low reputation → fewer assignments → less income
   - Market mechanism incentivizes good behavior

3. **Bad Actor Mitigation:**
   - Fraud reports destroy reputation
   - Low reputation = can't verify
   - Stake slashing for malicious behavior
   - System is self-policing

### For Token Exchange

1. **Arbitrage Opportunities:**
   - Cross-trust rate differences create arbitrage
   - TF2T players might build trust across communities
   - High-reputation verifiers can work across trusts

2. **Demurrage as Circulation Driver:**
   - Can't hoard tokens
   - Must spend or invest
   - Encourages cross-trust exchange
   - Velocity of money increases

### For Governance

1. **Verifier Selection:**
   - Top reputation verifiers become oracles
   - Oracles grant grace periods (emergency support)
   - Political power within trust

2. **Proposal Voting:**
   - High reputation → more voting weight?
   - Or reputation is separate from voting
   - TFT/TF2T dynamics could affect proposal outcomes

---

## Conclusion

The Stellar mutual credit contract implements a sophisticated **reputation economy** that aligns incentives through:

1. **Verification Staking:** Skin in the game
2. **Reputation Scoring:** Track record of behavior
3. **Demurrage:** Anti-hoarding mechanism
4. **Cross-Trust Exchange:** Inter-community trade
5. **Grace Periods:** Emergency safety net

In a **Tit for 2 Tats** environment:
- **Forgiving strategies (TF2T)** thrive
- **Punitive strategies (GRIM)** create permanent damage
- **Pure strategies (AC, AD)** are exploited
- **Reciprocal strategies (TFT, TF2T)** dominate

The system's **reputation-based verification** creates a **self-enforcing cooperative ecosystem** where good behavior is rewarded and bad actors are naturally excluded.

---

## Part 5: Scaling to 1000 Rounds - Stability Analysis

### Overview

Scaling from 100 to 1000 rounds (10x increase) fundamentally changes the experiment's dynamics. Longer time horizons reveal:
- **True convergence behavior** of strategies
- **Long-term equilibrium states** in reputation systems
- **Demurrage impact** on token economics
- **Noise resilience** of forgiveness strategies
- **Strategy evolution** as bad actors are filtered out

### Scaling Considerations

#### Computational Requirements

| Aspect | 100 Rounds | 1000 Rounds | Scaling Factor |
|--------|------------|-------------|----------------|
| **Transactions** | ~200 (2 claims/round) | ~2,000 | 10x |
| **Contract Calls** | ~600 (3 per claim) | ~6,000 | 10x |
| **Storage Operations** | ~400 | ~4,000 | 10x |
| **Execution Time** | ~30-60 minutes | ~5-10 hours | 10x |
| **Cost (Testnet)** | Free | Free | - |
| **Cost (Mainnet)** | ~0.1 XLM | ~1 XLM | 10x |

#### Technical Modifications

```python
# Modified simulation parameters
SIMULATION_CONFIG = {
    "rounds": 1000,  # Increased from 100
    "claims_per_round": 2,
    "verifiers_per_claim": 2,
    "demurrage_period_days": 90,  # Same
    "checkpoint_interval": 100,  # Save state every 100 rounds
    "metrics_interval": 50,  # Record metrics every 50 rounds
}
```

**Why Checkpointing Matters:**
- Stellar testnet may have intermittent issues
- Long-running simulations need crash recovery
- Allows analyzing intermediate states
- Enables early stopping if convergence reached

### Stability Effects

#### 1. Reputation Convergence

**100 Rounds:**
- Reputation scores still volatile
- Hard to distinguish strategy effectiveness
- Noise significantly impacts rankings

**1000 Rounds:**
- Reputation scores converge to stable equilibria
- Clear separation between strategies
- Law of large numbers reduces noise impact

**Convergence Timeline by Strategy:**

| Strategy | Rounds to Convergence | Stability at 100 | Stability at 1000 |
|----------|----------------------|------------------|-------------------|
| **AC** | 50-100 | Volatile | Converged low |
| **AD** | 30-50 | Volatile | Converged lowest |
| **TFT** | 200-300 | Still adjusting | Stable high |
| **TF2T** | 150-250 | Still adjusting | Stable highest |
| **GRIM** | 100-200 | Depends on triggers | Stable medium-low |
| **RAND** | Never | Random walk | Mean-reverting |

**Key Insight:** 100 rounds is insufficient for TFT/TF2T to show their true superiority. These strategies need time to:
1. Build cooperative relationships
2. Recover from initial mistakes
3. Outlast exploitative strategies

#### 2. Demurrage Impact Over 1000 Rounds

With 5% annual demurrage applied every 90 days:

```
Round 0:    Initial balance: 1000 tokens
Round 4:    First demurrage applied (360 days ≈ 1 year)
            Balance: 1000 × 0.95 = 950 tokens
Round 8:    Second demurrage
            Balance: 950 × 0.95 = 902.5 tokens
Round 12:   Third demurrage
            Balance: 902.5 × 0.95 = 857.4 tokens
...
Round 1000: ~250 years simulated
            Multiple demurrage cycles complete
```

**Stability Implications:**

| Metric | 100 Rounds | 1000 Rounds |
|--------|------------|-------------|
| **Demurrage Cycles** | 0-1 | ~25 |
| **Token Velocity** | Not measurable | Clear pattern emerges |
| **Hoarding Penalty** | Minimal | Significant (99% loss) |
| **Work Incentive** | Low pressure | High pressure |
| **Exchange Behavior** | Limited data | Clear arbitrage patterns |

**New Dynamic at 1000 Rounds:**
- Verifiers MUST participate actively
- Staked tokens depreciate significantly
- Cross-trust exchange becomes essential
- System reveals true circulation requirements

#### 3. Noise Resilience

Assuming 10% accidental defection rate (noise):

**100 Rounds:**
- Expected accidents: 10 per verifier
- TFT vs TF2T difference: marginal
- One accident can significantly impact ranking

**1000 Rounds:**
- Expected accidents: 100 per verifier
- Law of large numbers applies
- TF2T's forgiveness becomes decisive advantage

**Simulated TF2T Advantage:**

```python
# Noise impact over different horizons
def calculate_noise_impact(rounds, noise_rate=0.10):
    accidents = rounds * noise_rate

    # TFT: retaliates for each accident
    tft_extra_defections = accidents

    # TF2T: retaliates only for 2+ consecutive accidents
    # Probability of n consecutive accidents: noise_rate^n
    tf2t_extra_defections = accidents * (noise_rate ** 2)

    return tft_extra_defections, tf2t_extra_defections

# 100 rounds
tft_100, tf2t_100 = calculate_noise_impact(100, 0.10)
# TFT: 10 extra defections, TF2T: 0.1 extra defections

# 1000 rounds
tft_1000, tf2t_1000 = calculate_noise_impact(1000, 0.10)
# TFT: 100 extra defections, TF2T: 1 extra defection
```

**Result:** At 1000 rounds, TF2T's advantage is ~100x clearer than at 100 rounds.

#### 4. Bad Actor Filtering

**100 Rounds:**
- Bad actors (AD, AC) may still appear viable
- Reputation system still filtering
- Some exploitation possible

**1000 Rounds:**
- Bad actors reputation destroyed
- System self-purges
- Only cooperative strategies remain

**Reputation Trajectory:**

```
Reputation Score Over Time

100 | AC    TF2T        TFT
    |  \    /|\         /|\
 75 |   \  / | \       / | \
    |    \/  |  \     /  |  \
 50 |     \  |   \   /   |   \
    |      \ |    \ /    |    \
 25 |       \|     X     |     \___
    |        \   /   \   |         \___
  0 |________\__/_____\__|_____________\_____ AD
    0        100     200           500    1000
              Rounds
```

### Modified Hypotheses for 1000 Rounds

#### H1 (Revised): TF2T Dominance Increases with Scale

**Original:** TF2T will have highest reputation at 100 rounds.

**Revised:** TF2T's advantage over TFT grows from ~5% at 100 rounds to ~20% at 1000 rounds.

**Reasoning:**
- More accidents occur over 1000 rounds
- TFT's sensitivity to noise compounds
- TF2T's forgiveness provides cumulative advantage

**Predicted Scores:**
```
100 Rounds:  TF2T: 72,  TFT: 68,  GRIM: 55
1000 Rounds: TF2T: 81,  TFT: 65,  GRIM: 42
```

#### H2 (New): System Self-Stabilization

**Hypothesis:** The system will reach a stable equilibrium where:
- All AD and AC verifiers drop below minimum reputation threshold
- Remaining verifiers are TFT, TF2T, and some GRIM
- New verifiers entering adopt successful strategies

**Timeline:**
- Round 0-200: Chaotic phase (all strategies competitive)
- Round 200-500: Filtering phase (pure strategies decline)
- Round 500-1000: Equilibrium phase (cooperative strategies dominate)

#### H3 (New): Cross-Trust Exchange Emergence

**Hypothesis:** At 1000 rounds, cross-trust exchange becomes:
- Essential for token survival (demurrage forces spending)
- Strategic for high-reputation verifiers
- Profitable through arbitrage opportunities

**Mechanism:**
1. Verifiers earn tokens in one trust
2. Demurrage erodes value if held
3. Exchange to other trust with better rate
4. Higher circulation velocity
5. System-wide efficiency improves

#### H4 (New): Reputation Inequality

**Hypothesis:** 1000 rounds reveals "rich get richer" dynamics:
- Top verifiers accumulate disproportionate reputation
- New entrants face barriers to entry
- System may become oligarchic

**Mitigation:**
- Reputation decay for inactivity
- Periodic reputation resets (epoch-based)
- Rotation mechanisms for verifier assignment

### Implementation Changes for 1000 Rounds

#### 1. Checkpoint System

```python
import json
from pathlib import Path

class SimulationState:
    def __init__(self, checkpoint_dir="checkpoints"):
        self.checkpoint_dir = Path(checkpoint_dir)
        self.checkpoint_dir.mkdir(exist_ok=True)
        self.current_round = 0

    def save_checkpoint(self, round_num, verifiers, workers, metrics):
        """Save state every 100 rounds"""
        if round_num % 100 != 0:
            return

        state = {
            "round": round_num,
            "verifiers": [
                {
                    "name": v.name,
                    "strategy": v.strategy,
                    "reputation": v.reputation_score,
                    "last_moves": v.last_moves
                }
                for v in verifiers
            ],
            "workers": [
                {
                    "name": w.name,
                    "balance": w.balance,
                    "hours": w.contribution_hours
                }
                for w in workers
            ],
            "metrics": metrics
        }

        filepath = self.checkpoint_dir / f"checkpoint_{round_num}.json"
        with open(filepath, 'w') as f:
            json.dump(state, f, indent=2)

        print(f"✓ Checkpoint saved at round {round_num}")

    def load_checkpoint(self, round_num):
        """Resume from checkpoint if simulation crashed"""
        filepath = self.checkpoint_dir / f"checkpoint_{round_num}.json"
        if not filepath.exists():
            return None

        with open(filepath, 'r') as f:
            return json.load(f)
```

#### 2. Progress Monitoring

```python
class ProgressTracker:
    def __init__(self, total_rounds=1000):
        self.total_rounds = total_rounds
        self.start_time = None
        self.milestones = {
            100: "10% - Initial patterns emerging",
            250: "25% - Filtering begins",
            500: "50% - Mid-point review",
            750: "75% - Equilibrium approaching",
            1000: "100% - Final state"
        }

    def start(self):
        self.start_time = time.time()

    def update(self, round_num):
        if round_num in self.milestones:
            elapsed = time.time() - self.start_time
            avg_time = elapsed / round_num
            remaining = (self.total_rounds - round_num) * avg_time

            print(f"\n{'='*60}")
            print(f"Round {round_num}/{self.total_rounds}: {self.milestones[round_num]}")
            print(f"Elapsed: {elapsed/60:.1f}m | ETA: {remaining/60:.1f}m")
            print(f"{'='*60}\n")
```

#### 3. Adaptive Stopping

```python
class ConvergenceDetector:
    """Stop early if system has converged"""

    def __init__(self, window=50, threshold=0.01):
        self.window = window
        self.threshold = threshold
        self.reputation_history = []

    def check_convergence(self, current_reputations):
        """Check if reputations have stabilized"""
        self.reputation_history.append(current_reputations)

        if len(self.reputation_history) < self.window:
            return False

        # Compare last window rounds
        recent = self.reputation_history[-self.window:]
        changes = []

        for i in range(1, len(recent)):
            for verifier in recent[i]:
                prev_score = recent[i-1][verifier]
                curr_score = recent[i][verifier]
                change = abs(curr_score - prev_score) / prev_score
                changes.append(change)

        avg_change = sum(changes) / len(changes)

        if avg_change < self.threshold:
            print(f"✓ Converged at round {len(self.reputation_history)}")
            print(f"  Average change: {avg_change:.4%} < {self.threshold:.1%}")
            return True

        return False
```

### Expected Results: 100 vs 1000 Rounds

| Strategy | 100 Rounds | 1000 Rounds | Δ |
|----------|------------|-------------|---|
| **TF2T** | 72 (🥇) | 81 (🥇) | +9 |
| **TFT** | 68 (🥈) | 65 (🥈) | -3 |
| **RAND** | 52 (⚖️) | 48 (⚖️) | -4 |
| **GRIM** | 55 (⚖️) | 42 (⚠️) | -13 |
| **AC** | 38 (⬇️) | 12 (💀) | -26 |
| **AD** | 15 (💀) | 0 (💀) | -15 |

**Key Observations:**

1. **TF2T extends lead:** From +4 to +16 over TFT
2. **TFT slightly declines:** Vulnerability to GRIM compounds
3. **GRIM crashes:** Permanent defections destroy reputation
4. **AC collapses:** Fraud approval accumulates
5. **AD eliminated:** Rejection of valid work is unsustainable

### Stability Analysis

#### System-Level Metrics

| Metric | 100 Rounds | 1000 Rounds | Interpretation |
|--------|------------|-------------|----------------|
| **Approval Rate** | 68% | 76% | ↑ System becomes more cooperative |
| **Fraud Detection** | 45% | 92% | ↑ Bad actors filtered out |
| **Reputation Variance** | High | Low | ↓ System stabilizes |
| **Token Velocity** | Unknown | Measurable | ↑ Circulation increases |
| **Cross-Trust Exchange** | Minimal | Significant | ↑ Inter-trust trade emerges |

#### Convergence Patterns

**Phase 1: Exploration (Rounds 0-200)**
- All strategies competitive
- High volatility in rankings
- Noise dominates signals
- No clear winners

**Phase 2: Filtering (Rounds 200-500)**
- Pure strategies decline (AC, AD)
- Cooperative strategies rise (TFT, TF2T)
- Reputation variance decreases
- Clear ranking emerges

**Phase 3: Equilibrium (Rounds 500-1000)**
- Stable ranking established
- TF2T dominates
- New verifiers adopt successful strategies
- System self-corrects to disturbances

### Practical Recommendations

#### For 100-Round Experiments
- Use for rapid prototyping
- Test strategy implementations
- Identify obvious bugs
- Not suitable for final conclusions

#### For 1000-Round Experiments
- Use for definitive results
- Publish final findings
- Analyze long-term dynamics
- Reveal true system behavior

#### Resource Optimization

```python
# Hybrid approach: run 100 rounds first, then extend
def run_adaptive_simulation():
    # Phase 1: Quick test (100 rounds)
    results_100 = run_simulation(rounds=100)
    analyze_quick_results(results_100)

    # Phase 2: Extend promising configurations
    if results_100["viable"]:
        results_1000 = run_simulation(
            rounds=1000,
            initial_state=results_100["final_state"],
            resume_from_round=100
        )
        return results_1000
    else:
        return results_100
```

### Conclusion: Scale Matters

**100 rounds reveals:**
- Initial dynamics
- Strategy bugs
- Rough rankings

**1000 rounds reveals:**
- True convergence
- Stable equilibria
- Long-term viability
- System resilience

**For this Stellar contract:**
- 100 rounds ≈ 6 months of real-world operation
- 1000 rounds ≈ 5 years of real-world operation
- Community sustainability requires long-term view

**Recommendation:** Run 1000-round experiments for publication-quality results. Use 100-round experiments for development and testing.

---

**Report End**

*Generated by Claude Code Security Tester*
*Repository: security-tester*
