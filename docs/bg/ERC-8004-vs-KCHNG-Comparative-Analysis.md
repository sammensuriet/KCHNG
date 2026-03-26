# ERC-8004 vs KCHNG: Comparative Analysis of On-Chain Trust & Identity Systems

**Date:** February 7, 2026
**Prepared for:** CLI-QA Research

---

## Executive Summary

This document provides a comparative analysis between **ERC-8004** (Ethereum's AI Agent Identity Standard) and **KCHNG** (a Stellar-based community currency implementing the Wörgl demurrage model with role-based reputation).

### High-Level Comparison

| Dimension | ERC-8004 | KCHNG |
|-----------|----------|-------|
| **Primary Purpose** | AI Agent Identity & Trust | Community Currency with Time-Standard Economics |
| **Blockchain** | Ethereum/EVM (also Avalanche C-Chain) | Stellar/Soroban |
| **Target Entity** | Autonomous AI Agents | Human Workers in Care/Agriculture |
| **Economic Model** | Agent marketplace with reputation scores | Time-backed currency with demurrage |
| **Reputation Scope** | Single-score success rate | Role-based (Domain → Aspect → Role) |
| **Maturity** | Production (30,000+ agents, Jan 2026) | Development/Testing phase |

---

## 1. Conceptual Overview

### 1.1 ERC-8004: Trustless AI Agents

```
┌─────────────────────────────────────────────────────────────────┐
│                    ERC-8004 Architecture                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌──────────────────┐  ┌──────────────────┐  ┌──────────────┐ │
│  │  Identity        │  │  Reputation      │  │  Validation  │ │
│  │  Registry        │  │  Registry        │  │  Registry    │ │
│  └──────────────────┘  └──────────────────┘  └──────────────┘ │
│         │                      │                     │          │
│         ▼                      ▼                     ▼          │
│  "Who are you?"    "Should I trust you?"    "Are you legit?"   │
│                                                                 │
│  Primary Use Cases:                                           │
│  • DeFi trading bots                                           │
│  • DAO treasury management                                     │
│  • Gaming NPCs                                                 │
│  • Cross-agent collaboration                                   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 1.2 KCHNG: Time-Standard Community Currency

```
┌─────────────────────────────────────────────────────────────────┐
│                    KCHNG Architecture                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  INPUT SIDE                    OUTPUT SIDE                      │
│  ┌──────────────┐              ┌──────────────┐                │
│  │ Work         │              │ Demurrage    │                │
│  │ Verification │              │ (8-12%       │                │
│  │ (Multi-ver.) │──→ KCHNG ───→│  annual)     │                │
│  └──────────────┘              └──────────────┘                │
│         │                                                      │
│         ▼                                                      │
│  ┌─────────────────────────────────────────────────────┐       │
│  │  Reputation System (Domain → Aspect → Role)         │       │
│  │  • General Score: 0-1000                            │       │
│  │  • Role Scores: dining:guest, ride_sharing:driver   │       │
│  │  • Starting Default: 500 (neutral)                 │       │
│  └─────────────────────────────────────────────────────┘       │
│                                                                 │
│  Primary Use Cases:                                           │
│  • Care work compensation                                      │
│  • Agricultural labor coordination                              │
│  • Community meal exchange                                     │
│  • Federated trust management                                  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## 2. Detailed Comparison

### 2.1 Problem Domain

| Aspect | ERC-8004 | KCHNG |
|--------|----------|-------|
| **Problem Solved** | "How do we trust autonomous AI agents?" | "How do we create a fair, circulating community currency?" |
| **Entity Type** | Non-human (AI agents) | Human (workers, verifiers) |
| **Primary Context** | Digital services, DeFi, gaming | Local care work, agriculture, food systems |
| **Economic Basis** | Service fees, transaction value | Time (30 min = 1000 KCHNG = 1 meal) |

### 2.2 Reputation System Comparison

#### ERC-8004 Reputation Registry

```
┌─────────────────────────────────────────────────────────────┐
│  ERC-8004 Reputation Data Structure                         │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Agent: 0x123...                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  successRate: 94.2%                                 │   │
│  │  totalInteractions: 1,247                          │   │
│  │  averageRating: 4.7 / 5.0                          │   │
│  │  feedbackHistory: [...]                            │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  Scoring: Single aggregate score + feedback history         │
└─────────────────────────────────────────────────────────────┘
```

#### KCHNG Role-Based Reputation

```
┌─────────────────────────────────────────────────────────────┐
│  KCHNG Reputation Data Structure                            │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Verifier: G...                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  reputation_score: 720 (general trust)              │   │
│  │  verified_claims: 156                               │   │
│  │  rejected_claims: 8                                 │   │
│  │  fraud_reports: 0                                   │   │
│  │                                                      │   │
│  │  aspect_scores: {                                   │   │
│  │    "dining:guest": 850,                            │   │
│  │    "dining:host": 400,                             │   │
│  │    "ride_sharing:driver": 920,                     │   │
│  │    "verification:oracle": 680                      │   │
│  │  }                                                  │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  Scoring: Three-level hierarchy with role-specific scores   │
└─────────────────────────────────────────────────────────────┘
```

### 2.3 Reputation Feature Matrix

| Feature | ERC-8004 | KCHNG |
|---------|----------|-------|
| **General Score** | ✅ Success rate (0-100%) | ✅ Reputation score (0-1000) |
| **Role-Specific Scores** | ❌ Not supported | ✅ Domain → Aspect → Role |
| **Historical Tracking** | ✅ Feedback history | ✅ verified/rejected counts |
| **Fraud Detection** | ⚠️ Through validation registry | ✅ fraud_reports counter |
| **Neutral Starting Point** | ⚠️ Varies by implementation | ✅ 500 (neutral) for new roles |
| **Multi-Context Identity** | ❌ Single context | ✅ Multiple independent roles |

### 2.4 Governance & Verification

| Aspect | ERC-8004 | KCHNG |
|--------|----------|-------|
| **Verification Model** | Validation registry with attestations | Multi-verifier voting system |
| **Governance** | Open registry,任何人可以register | Federated Community Trusts with Governors |
| **Dispute Resolution** | Through validation registry | Community DAO + Trust leadership |
| **Cross-Entity Trust** | Agent-to-agent reputation | Cross-trust exchange with rate adjustment |

---

## 3. Architectural Comparison

### 3.1 Smart Contract Architecture

#### ERC-8004 (Solidity/EVM)

```solidity
contract ERC8004 {
    // Identity Registry
    struct Agent {
        address owner;
        string name;
        string[] capabilities;
        bytes metadata;
    }

    // Reputation Registry
    struct Reputation {
        uint256 successRate;      // 0-100 basis points
        uint256 totalJobs;
        uint256 averageRating;
    }

    // Validation Registry
    struct Validation {
        bool kycVerified;
        bytes auditReport;
        mapping(address => bool) attestations;
    }
}
```

#### KCHNG (Rust/Soroban)

```rust
pub struct VerifierData {
    pub trust_id: Option<Address>,
    pub stake: U256,
    pub reputation_score: u32,              // 0-1000 (general)
    pub verified_claims: u32,
    pub rejected_claims: u32,
    pub fraud_reports: u32,
    pub aspect_scores: Map<Bytes, u32>,     // "aspect:role" → score
}
```

### 3.2 Hierarchy Comparison

```
ERC-8004:                    KCHNG:
┌─────────────┐             ┌─────────────────────────────────┐
│   Agent     │             │  Domain (Transportation)       │
└──────┬──────┘             │    └── Aspect (Ride-sharing)   │
       │                    │         ├── Role (Driver)       │
       ▼                    │         └── Role (Passenger)    │
┌─────────────┐             │                                 │
│ Reputation  │             │  Domain (Hospitality)          │
│ (single)    │             │    └── Aspect (Dining)         │
└─────────────┘             │         ├── Role (Guest)        │
                            │         └── Role (Host)         │
                            └─────────────────────────────────┘

Flat model vs. 3-level hierarchical model
```

---

## 4. Economic Model Comparison

### 4.1 Incentive Structures

| Dimension | ERC-8004 | KCHNG |
|-----------|----------|-------|
| **Value Creation** | Agent services rendered | Work time contributed |
| **Token Economics** | Marketplace pricing | Time-standard (30 min = 1000 KCHNG) |
| **Circulation Driver** | Reputation → more jobs | Demurrage (8-12% annual decay) |
| **Stake Requirement** | Agent registration stake | Verifier stake (100-2000 KCHNG) |

### 4.2 Economic Flow

```
ERC-8004 Economic Flow:          KCHNG Economic Flow:
                                 ┌──────────────┐
┌──────────────┐                 │              │
│   User       │                 │  Work        │
│      ↓       │                 │  (30 min)    │
│  Find Agent  │                 │      ↓       │
│      ↓       │                 │  +1000 KCHNG │
│  Hire Agent  │                 │      ↓       │
│      ↓       │                 │  Demurrage   │
│  Pay on      │                 │  (12%/yr)    │
│  Completion  │                 │      ↓       │
└──────────────┘                 │  Circulate   │
                                 │      ↓       │
Service economy with             │  Redeem Meal │
reputation-based selection       └──────────────┘

Time-based economy with
forced circulation via demurrage
```

---

## 5. Use Case Analysis

### 5.1 Applicability to Different Scenarios

| Scenario | ERC-8004 Better For | KCHNG Better For |
|----------|---------------------|------------------|
| **DeFi Trading** | ✅ AI trading bots | ❌ Not designed for this |
| **Gaming NPCs** | ✅ On-chain game agents | ❌ Not designed for this |
| **Care Work** | ⚠️ Could be adapted | ✅ Designed for this |
| **Agriculture** | ⚠️ Could be adapted | ✅ Designed for this |
| **Local Food Systems** | ❌ Not applicable | ✅ Core use case |
| **DAO Treasury** | ✅ Autonomous management | ⚠️ Different focus |

### 5.2 Cross-Pollination Opportunities

#### ERC-8004 Features That Could Enhance KCHNG

| Feature | How It Could Help KCHNG |
|---------|------------------------|
| **Validation Registry** | Formal KYC/audit verification for high-stake verifiers |
| **Agent Discovery API** | Standardized way to find verifiers by capability |
| **Cross-Agent Reputation** | Verifier reputation across multiple Community Trusts |

#### KCHNG Features That Could Enhance ERC-8004

| Feature | How It Could Help ERC-8004 |
|---------|--------------------------|
| **Role-Based Reputation** | Context-specific scoring (e.g., an agent good at arbitrage but poor at lending) |
| **Multi-Verifier Voting** | More robust verification of agent capabilities |
| **Demurrage Economics** | Incentivize active agent participation vs. hoarding |

---

## 6. Technical Implementation Comparison

### 6.1 Blockchain Platform Considerations

| Aspect | ERC-8004 (Ethereum/EVM) | KCHNG (Stellar/Soroban) |
|--------|------------------------|-------------------------|
| **Smart Contract Language** | Solidity, Vyper | Rust (WASM) |
| **Transaction Cost** | Higher (gas fees) | Lower (resource-based) |
| **Finality Time** | ~12 seconds (L1) | ~3-5 seconds |
| **Cross-Chain Portability** | High (EVM standard) | Low (Soroban-specific) |
| **Developer Ecosystem** | Largest | Growing but smaller |

### 6.2 Data Storage Efficiency

| Metric | ERC-8004 | KCHNG |
|--------|----------|-------|
| **Per-Entity Storage** | ~200-400 bytes | ~300-600 bytes (with roles) |
| **Update Cost** | Moderate gas | Lower operations |
| **Scalability** | Proven at 30k+ agents | Designed for thousands of verifiers |

---

## 7. Strategic Recommendations

### 7.1 For KCHNG Development

| Recommendation | Rationale |
|----------------|-----------|
| **1. Adopt ERC-8004's validation approach** | Formal attestation system would strengthen verifier verification |
| **2. Consider role-based reputation as differentiator** | This is a genuine innovation over ERC-8004's flat model |
| **3. Document reputation calculation methodology** | Clear formulas help build trust in scoring |
| **4. Explore cross-trust reputation portability** | Allow verifiers to carry reputation between Community Trusts |

### 7.2 For ERC-8004 Adoption in Similar Contexts

| Recommendation | Rationale |
|----------------|-----------|
| **1. Consider role-based reputation extension** | Multi-dimensional scoring better reflects real-world nuance |
| **2. Add time-based activity requirements** | Prevent stale or abandoned agents from clogging the registry |
| **3. Implement federated verification** | Similar to KCHNG's Community Trusts for better scalability |

### 7.3 For Blockchain Selection

| Scenario | Recommended Platform |
|----------|---------------------|
| **AI Agent Marketplace** | Ethereum (ERC-8004) - mature ecosystem |
| **Community Currency System** | Stellar (KCHNG) - designed for payments |
| **Hybrid Approach** | Consider bridge or cross-chain messaging |

---

## 8. Convergence Possibilities

### 8.1 Theoretical Integration

```
┌─────────────────────────────────────────────────────────────────┐
│                  Potential Hybrid Architecture                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────────────┐         ┌─────────────────────┐       │
│  │   ERC-8004          │         │   KCHNG             │       │
│  │   (Ethereum)        │  ←↕→   │   (Stellar)         │       │
│  │                     │         │                     │       │
│  │  • AI Agent ID      │         │  • Human Verifier   │       │
│  │  • Basic Reputation │         │  • Role-Based Rep   │       │
│  └─────────────────────┘         └─────────────────────┘       │
│              │                              │                   │
│              ▼                              ▼                   │
│  ┌─────────────────────────────────────────────────────┐       │
│  │              Bridge/Attestation Layer               │       │
│  │  • Cross-chain reputation verification              │       │
│  │  • Agent-to-human service coordination             │       │
│  │  • Hybrid economic models (agent + human labor)    │       │
│  └─────────────────────────────────────────────────────┘       │
│                                                                 │
│  Use Case: AI agents coordinate human work through KCHNG,     │
│            while humans verify agent outputs                   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 8.2 Shared Philosophical Principles

| Principle | ERC-8004 | KCHNG |
|-----------|----------|-------|
| **Decentralization** | ✅ No central authority | ✅ Federated trusts |
| **Reputation-Based Trust** | ✅ Core mechanism | ✅ Core mechanism |
| **Economic Participation** | ✅ Agents as economic actors | ✅ Workers as economic participants |
| **Community Governance** | ⚠| Emerging | ✅ DAO + Trust Governors |

---

## 9. Conclusion

### 9.1 Summary of Key Differences

| Dimension | ERC-8004 | KCHNG |
|-----------|----------|-------|
| **Entity Type** | AI Agents | Human Workers |
| **Reputation Model** | Flat, single-score | Hierarchical, role-based |
| **Economic Model** | Service marketplace | Time-standard with demurrage |
| **Maturity** | Production (30k+ agents) | Development |
| **Primary Use Case** | Digital services automation | Community economic coordination |

### 9.2 Complementary Strengths

- **ERC-8004** excels at:
  - Standardized agent discovery
  - Cross-platform interoperability (EVM)
  - Simple, easy-to-understand reputation
  - Rapid ecosystem growth

- **KCHNG** excels at:
  - Multi-dimensional reputation tracking
  - Community-focused economic design
  - Time-based value anchoring
  - Federated governance flexibility

### 9.3 Final Assessment

**ERC-8004 and KCHNG serve fundamentally different purposes but share the philosophical foundation of on-chain reputation as a trust mechanism.**

ERC-8004 is optimized for **AI agent discoverability and trust** in a global marketplace context. KCHNG is optimized for **human community economic coordination** with sophisticated multi-context reputation tracking.

The systems are not directly competitive but could potentially inform each other's evolution:
- ERC-8004 could benefit from KCHNG's role-based reputation granularity
- KCHNG could benefit from ERC-8004's standardized validation and discovery patterns

---

## 10. References

### ERC-8004 Sources
- [ERC-8004 Explained](https://learn.backpack.exchange/articles/erc-8004-explained)
- [Build Trustless Agents with EigenCloud](https://docs.eigencloud.xyz/eigenai/howto/build-trustless-agents)
- [vistara-apps/erc-8004-example](https://github.com/vistara-apps/erc-8004-example)

### KCHNG Documentation
- `./docs/time-standard-token-design.md`
- `./docs/2026-01-15_role-based-reputation-design.md`
- `./docs/PRD.md`

---

**Document Version:** 1.0
**Last Updated:** February 7, 2026
**Status:** Draft Analysis
