# Time-Standard Token - Mermaid Diagrams

**Source:** [time-standard-token-design.md](./time-standard-token-design.md)

All diagrams in Mermaid.js syntax for easy visualization and implementation.

---

## 1. Complete System Overview

```mermaid
graph TB
    subgraph Protocol["TIME-STANDARD PROTOCOL (Base Layer)"]
        P1["1 TOK = 30 min work"]
        P2["Rate range: 5-15% annually"]
        P3["Verification standards"]
        P4["Inter-change formulas"]
    end

    subgraph TrustA["COMMUNITY TRUST A (Urban Care)"]
        TA1["Rate: 12% annual<br/>Period: Monthly<br/>Focus: Elderly care<br/>Members: 500"]
    end

    subgraph TrustB["COMMUNITY TRUST B (Rural Farm)"]
        TB1["Rate: 8% annual<br/>Period: Quarterly<br/>Focus: Agriculture<br/>Members: 150"]
    end

    subgraph TrustC["COMMUNITY TRUST C (Crisis Zone)"]
        TC1["Rate: 15% annual<br/>Period: Quarterly<br/>Focus: Recovery<br/>Temporary: 2 years"]
    end

    subgraph Verifiers["VERIFICATION NETWORK"]
        V1["Work Verifiers"]
        V2["Grace Oracles"]
        V3["Trust Governors"]
        V4["Cross-Trust Auditors"]
    end

    subgraph Exchange["INTER-TRUST EXCHANGE"]
        E1["Rate-Adjusted Swap<br/>Formula: 1 × (1-rA)/(1-rB)"]
        E2["1:1 Base (Time Standard)"]
        E3["Decay Adjustment"]
    end

    subgraph DAO["NETWORK DAO"]
        D1["Dispute Resolution"]
        D2["Protocol Upgrades"]
        D3["Emergency Coordination"]
    end

    subgraph Workers["WORKERS & HOLDERS"]
        W1["Move freely between Trusts"]
        W2["Earn through work"]
        W3["Spend on meals"]
    end

    Protocol --> TrustA
    Protocol --> TrustB
    Protocol --> TrustC
    TrustA --> Verifiers
    TrustB --> Verifiers
    TrustC --> Verifiers
    TrustA --> Exchange
    TrustB --> Exchange
    TrustC --> Exchange
    Exchange --> Workers
    Verifiers --> DAO
    Exchange --> DAO
```

---

## 2. Input Side: Work-to-Token Flow

```mermaid
flowchart LR
    A[Worker performs<br/>30min care/agri work] --> B[Submit evidence<br/>photo/GPS/notes]
    B --> C[Smart Contract<br/>assigns verifiers]
    C --> D{Verification}
    D -->|Approved| E[+1 TOK minted<br/>to worker wallet]
    D -->|Rejected| F[Feedback provided<br/>to worker]
    E --> G[Demurrage clock<br/>starts]
    F --> H[Worker can<br/>resubmit]

    style E fill:#90EE90,stroke:#006400,stroke-width:2px
    style F fill:#FFB6C1,stroke:#8B0000,stroke-width:2px
    style G fill:#FFD700,stroke:#B8860B,stroke-width:2px
```

---

## 3. Output Side: Demurrage Decay Model

```mermaid
graph LR
    subgraph Input["START: 100 TOK Balance"]
        A["Initial State:<br/>100 TOK"]
    end

    subgraph Month1["MONTH 1"]
        B1["1% decay applied<br/>-1 TOK<br/>99 TOK remaining"]
    end

    subgraph Month3["MONTH 3"]
        B2["1% decay applied<br/>-0.98 TOK<br/>97.03 TOK remaining"]
    end

    subgraph Month6["MONTH 6"]
        B3["1% decay applied<br/>-0.94 TOK<br/>93.24 TOK remaining"]
    end

    subgraph Month12["MONTH 12"]
        B4["1% decay applied<br/>-0.89 TOK<br/>87.75 TOK remaining"]
    end

    subgraph Result["END OF YEAR 1"]
        C["Total lost: 12.3%<br/>Remaining: 87.75 TOK<br/>Rate: 12.7% annually<br/>(compounded)"]
    end

    A --> Month1
    Month1 --> Month3
    Month3 --> Month6
    Month6 --> Month12
    Month12 --> Result

    style A fill:#87CEEB,stroke:#4682B4,stroke-width:2px
    style C fill:#90EE90,stroke:#006400,stroke-width:2px
```

---

## 4. Comparison: Wrong vs Correct Demurrage Rates

```mermaid
graph TB
    subgraph Wrong["WRONG RECOMMENDATION (Initial Error)"]
        direction TB
        W1["2% per 7 days"]
        W2["Annual Rate: ~104%"]
        W3["Result: 100 TOK → 16 TOK/year<br/>84% loss!"]
        W4["Verdict: ✘ Certain collapse"]
        style W4 fill:#FFB6C1
    end

    subgraph Correct["CORRECT RECOMMENDATION (Historically Proven)"]
        direction TB
        C1["1% per 30 days"]
        C2["Annual Rate: ~12.7%"]
        C3["Result: 100 TOK → 87.75 TOK/year<br/>12.3% loss"]
        C4["Verdict: ✓ Proven successful"]
        style C4 fill:#90EE90
    end
```

---

## 5. Community Trusts Network Architecture

```mermaid
graph TB
    subgraph Network["TIME-STANDARD FEDERATED NETWORK"]
        direction TB

        subgraph Protocol["BASE PROTOCOL"]
            BP1["1 TOK = 30 min work"]
            BP2["Rate range: 5-15% annually"]
            BP3["Verification standards"]
            BP4["Governance framework"]
        end

        subgraph TrustA["TRUST A: Urban Elder Care"]
            TA["Rate: 12% annual<br/>Period: Monthly<br/>Members: 500<br/>Verifiers:<br/>• 3 care homes<br/>• 2 hospitals<br/>• Social workers"]
        end

        subgraph TrustB["TRUST B: Rural Agriculture"]
            TB["Rate: 8% annual<br/>Period: Quarterly<br/>Members: 150<br/>Verifiers:<br/>• Co-op board<br/>• 3 farm stores<br/>• Food bank"]
        end

        subgraph TrustC["TRUST C: Crisis Recovery Zone"]
            TC["Rate: 15% annual<br/>Period: Quarterly<br/>Duration: 2 years temp<br/>Verifiers:<br/>• Relief NGOs<br/>• Reconstruction auth<br/>• Community council"]
        end

        subgraph DAO["NETWORK DAO"]
            D["• Dispute resolution<br/>• Protocol upgrades<br/>• Emergency coordination<br/>• Cross-trust audits"]
        end

        subgraph Exchange["INTER-TRUST EXCHANGE"]
            X["Rate-adjusted swap:<br/>Rate = 1 × (1-rA)/(1-rB)<br/><br/>Example:<br/>Trust A (12%) → Trust B (8%)<br/>1 TOK_A = 0.957 TOK_B"]
        end
    end

    Protocol --> TrustA
    Protocol --> TrustB
    Protocol --> TrustC
    TrustA --> Exchange
    TrustB --> Exchange
    TrustC --> Exchange
    TrustA --> DAO
    TrustB --> DAO
    TrustC --> DAO
    Exchange --> DAO
```

---

## 6. Verification Flow (Sequence Diagram)

```mermaid
sequenceDiagram
    participant W as Worker
    participant SC as Smart Contract
    participant V1 as Verifier 1
    participant V2 as Verifier 2
    participant R as Reward Pool

    W->>SC: Submit work evidence<br/>(30 min care work)
    SC->>V1: Assign verification request
    SC->>V2: Assign verification request

    par Parallel Verification
        V1->>V1: Review evidence<br/>(photo, GPS, time)
        V2->>V2: Review evidence<br/>(photo, GPS, time)
    end

    alt Evidence Valid
        V1->>SC: Approve claim
        V2->>SC: Approve claim
        SC->>W: Mint +1 TOK
        SC->>R: Reward verifiers<br/>+0.05 TOK each
    else Evidence Invalid
        V1->>SC: Reject claim
        V2->>SC: Reject claim
        SC->>W: Return feedback<br/>(can resubmit)
    end

    Note over W,R: Incentive: Verifiers earn TOK<br/>for accurate validation
```

---

## 7. Grace Period State Diagram

```mermaid
stateDiagram-v2
    [*] --> Normal: Account created<br/first TOK earned

    Normal --> Emergency: Emergency occurs<br/>(illness, accident, etc.)
    Emergency --> GracePeriod: Oracle verifies<br/>& activates grace

    GracePeriod --> GracePeriod: May extend if<br/>recovery ongoing
    GracePeriod --> GraceExtended: Community votes<br/>for extension

    GraceExtended --> Normal: Extension expires<br/>or recovery complete
    GracePeriod --> Normal: Recovery complete

    Normal --> Normal: Account active<br/>demurrage applies

    note right of GracePeriod
        Demurrage PAUSED
        No decay during grace
        Balance protected
    end note

    note right of Normal
        Demurrage ACTIVE
        1% per month decay
        Activity resets timer
    end note
```

---

## 8. Grace Period Timeline

```mermaid
timeline
    title Grace Period Timeline Example
    section Pre-Emergency
        Day -5 : User breaks leg<br/>Hospitalized
    section Emergency Response
        Day 0  : Emergency grace activated<br/>(verified by hospital)
        Day 1-30: Demurrage PAUSED<br/>No decay
    section Extension Period
        Day 31  : User still recovering<br/>Requests extension
        Day 31  : Community approves<br/>30-day extension
        Day 31-60: Extended grace period<br/>Still no demurrage
    section Recovery
        Day 61  : Grace period ends<br/>Normal demurrage resumes
        Day 61+ : User active again<br/>Balance now decays normally
```

---

## 9. Governance Flow

```mermaid
graph TB
    subgraph Proposal["PROPOSAL STAGE"]
        A[Governor proposes<br/>rate change] --> B{Stake held?}
        B -->|Yes| C[Post proposal with<br/>justification & data]
        B -->|No| D[Insufficient stake<br/>rejected]
    end

    subgraph Review["REVIEW STAGE (7 days)"]
        C --> E[Public discussion period]
        E --> F[Expert testimony]
        E --> G[Counter-proposals]
        E --> H[Effect simulation]
    end

    subgraph Vote["VOTE STAGE (3 days)"]
        F --> I{Community vote}
        G --> I
        H --> I
        I -->|Pass ≥60%| J[Quorum check:<br/>40% participation]
        I -->|Fail <60%| K[Proposal rejected]
        J -->|<40%| K
        J -->|≥40%| L[Approval granted]
    end

    subgraph Implement["IMPLEMENTATION (30 days)"]
        L --> M[Smart contract<br/>scheduled update]
        M --> N[All members notified]
        N --> O[Rate change<br/>implemented]
    end

    style L fill:#90EE90
    style K fill:#FFB6C1
    style O fill:#87CEEB
```

---

## 10. Cross-Trust Exchange Flow

```mermaid
flowchart LR
    A[Holder from Trust B<br/>100 TOK_B balance] --> B[Travels to<br/>Trust A area]
    B --> C[Wants to purchase<br/>local meal]
    C --> D[Exchange Smart Contract]

    D --> E{Calculate rate}
    E --> F["Trust B: 8% annual<br/>Trust A: 12% annual"]

    F --> G["Apply formula:<br/>Rate = 1 × (1-0.08)/(1-0.12)<br/>= 0.957"]

    G --> H[Exchange executes:<br/>100 TOK_B → 95.7 TOK_A]

    H --> I[Holder now has<br/>95.7 TOK_A balance]
    I --> J[Spend 95.7 TOK_A<br/>on local meal]

    style D fill:#87CEEB
    style G fill:#FFD700
    style J fill:#90EE90
```

---

## 11. Trust Rate Decision Framework

```mermaid
graph TB
    subgraph Factors["FACTORS INFLUENCING RATE CHOICE"]
        direction LR

        subgraph HighRate["HIGH RATE (10-12%)<br/>More circulation needed"]
            HR1[Low velocity<br/>High hoarding]
            HR2[Highly seasonal<br/>work]
            HR3[Large community<br/>500+ members]
            HR4[High urgency<br/>needs]
            HR5[Lower trust levels<br/>need oversight]
            HR6[High food costs<br/>in area]
        end

        subgraph LowRate["LOW RATE (8-9%)<br/>Gentle circulation"]
            LR1[High velocity<br/>Rapid circulation]
            LR2[Year-round<br/>steady work]
            LR3[Small community<br/><200 members]
            LR4[Stable economy<br/>established]
            LR5[High trust levels<br/>self-policing]
            LR6[Lower food costs<br/>abundant]
        end
    end

    subgraph Decision["DECISION PROCESS"]
        direction TB
        D1[Assess local factors]
        D2[Calculate optimal rate]
        D3{Within 5-15% range?}
        D3 -->|Yes| D4[Propose to community]
        D3 -->|No| D5[Adjust and re-calculate]
        D4 --> D6{Vote passes?}
        D6 -->|Yes| D7[Implement rate]
        D6 -->|No| D5
    end

    HighRate --> Decision
    LowRate --> Decision

    style HighRate fill:#FFB6C1
    style LowRate fill:#90EE90
    style D7 fill:#87CEEB
```

---

## 12. System Flow: Work to Meal

```mermaid
flowchart TB
    subgraph Creation["TIME IN: CURRENCY CREATION"]
        A[Worker does<br/>30 min care work] --> B[Evidence submitted]
        B --> C[Verifier confirms]
        C --> D[+1 TOK minted]
    end

    subgraph Circulation["CIRCULATION PHASE"]
        D --> E[Holder decides<br/>when to spend]
        E --> F{Holding time?}
        F -->|<7 days| G[No demurrage]
        F -->|>7 days| H[Demurrage applies<br/>1% per month]
        H --> I[Balance gradually<br/>decreases]
        G --> J[Balance unchanged]
        I --> J
        J --> K[Can spend anytime<br/>on community meals]
    end

    subgraph Redemption["TIME OUT: MEAL REDEMPTION"]
        K --> L[Holder redeems<br/>1 TOK for meal]
        L --> M[Meal provided<br/>by partner]
        M --> N[TOK destroyed/<br/>removed from supply]
    end

    style D fill:#90EE90
    style N fill:#87CEEB
```

---

## 13. Protocol Constraints

```mermaid
graph TB
    subgraph Safe["SAFE ZONE (Optimal)"]
        S1["8-12% annually"]
        S2["Proven successful"]
        S3["Wörgl, Chiemgauer"]
    end

    subgraph Minimum["MINIMUM BOUNDARY"]
        M1["5% annually"]
        M2["Protocol minimum"]
        M3["Below = insufficient<br/>velocity"]
    end

    subgraph Maximum["MAXIMUM BOUNDARY"]
        X1["15% annually"]
        X2["Protocol maximum"]
        X3["Above = rejection<br/>risk"]
    end

    subgraph Crisis["CRISIS EXCEPTION"]
        C1["Temporary >15%"]
        C2["Requires 80% supermajority"]
        C3["Max 2 years duration"]
        C4["Auto-revert clause"]
    end

    M1 --> Safe
    Safe --> X1
    X1 -.->|exception| Crisis

    style Safe fill:#90EE90
    style Minimum fill:#FFD700
    style Maximum fill:#FFB6C1
    style Crisis fill:#87CEEB
```

---

## 14. Verification Roles & Permissions

```mermaid
graph TB
    subgraph Roles["VERIFICATION ROLES"]
        direction TB

        subgraph WV["WORK VERIFIERS"]
            WV1["Confirm work evidenced"]
            WV2["Validate domain quality"]
            WV3["Stake: 100 TOK"]
            WV4["Req: 50+ verified hours"]
        end

        subgraph GO["GRACE ORACLES"]
            GO1["Verify emergency claims"]
            GO2["Confirm illness/docs"]
            GO3["Stake: 500 TOK"]
            GO4["Req: Community approval"]
        end

        subgraph TG["TRUST GOVERNORS"]
            TG1["Set local rates"]
            TG2["Manage parameters"]
            TG3["Stake: 1000 TOK"]
            TG4["Req: Elected by members"]
        end

        subgraph CA["CROSS-TRUST AUDITORS"]
            CA1["Audit other trusts"]
            CA2["Ensure compliance"]
            CA3["Stake: 2000 TOK"]
            CA4["Req: Random assignment"]
        end
    end

    subgraph Permissions["PERMISSIONS"]
        direction LR
        P1[Work verification]
        P2[Grace activation]
        P3[Rate changes]
        P4[Cross-trust audit]
    end

    WV --> P1
    GO --> P2
    TG --> P3
    CA --> P4

    style WV fill:#90EE90
    style GO fill:#87CEEB
    style TG fill:#FFD700
    style CA fill:#FFB6C1
```

---

## 15. Implementation Roadmap

```mermaid
gantt
    title Time-Standard Token Implementation Roadmap
    dateFormat  YYYY-MM-DD
    section Phase 1: Foundation
    Protocol specs          :p1a, 2025-01-01, 30d
    Legal structure         :p1b, 2025-01-15, 30d
    Smart contract dev      :p1c, 2025-01-01, 60d
    Verification framework  :p1d, 2025-02-01, 30d

    section Phase 2: Pilot
    Onboard first Trust     :p2a, 2025-04-01, 30d
    Recruit verifiers       :p2b, 2025-04-15, 30d
    Launch pilot            :p2c, 2025-05-01, 90d
    Monitor metrics         :p2d, 2025-05-01, 90d

    section Phase 3: Expansion
    Onboard 2-3 Trusts      :p3a, 2025-08-01, 60d
    Cross-trust exchange    :p3b, 2025-09-01, 30d
    Deploy governance       :p3c, 2025-09-15, 30d
    Scale to 500+ members   :p3d, 2025-10-01, 90d

    section Phase 4: Network
    Open to new Trusts      :p4a, 2026-01-01, 180d
    Establish DAO          :p4b, 2026-02-01, 60d
    Scale to 2000+ members  :p4c, 2026-04-01, 180d
```

---

## 16. Anti-Fraud Measures

```mermaid
graph TB
    subgraph Prevention["PREVENTION LAYERS"]
        direction TB

        subgraph L1["LAYER 1: Submission"]
            L1A[Multiple verifiers<br/>randomly assigned]
            L1B[GPS verification<br/>location matching]
            L1C[Time-window check<br/>reasonable timing]
        end

        subgraph L2["LAYER 2: Verification"]
            L2A[Verifier reputation<br/>score tracking]
            L2B[Cross-verifier<br/>consensus required]
            L2C[Spot-check audits<br/>by cross-trust]
        end

        subgraph L3["LAYER 3: Enforcement"]
            L3A[Fraud = stake<br/>slashing]
            L3B[Permanent ban<br/>for repeat offenders]
            L3C[Community notification<br/>of bad actors]
        end
    end

    L1 --> L2
    L2 --> L3

    style L1 fill:#87CEEB
    style L2 fill:#FFD700
    style L3 fill:#FFB6C1
```

---

## 17. Economic Flow Diagram

```mermaid
flowchart LR
    subgraph Workers["WORKERS"]
        W1[Time spent<br/>working]
    end

    subgraph Creation["CURRENCY CREATION"]
        C1[Work evidenced]
        C2[Verified]
        C3[TOK minted]
    end

    subgraph Circulation["CIRCULATION"]
        V1[TOK held]
        V2[Demurrage<br/>applies]
        V3[Velocity<br/>increases]
    end

    subgraph Redemption["REDEMPTION"]
        R1[TOK spent]
        R2[Meal provided]
        R3[TOK destroyed]
    end

    Workers --> Creation
    Creation --> Circulation
    Circulation --> Redemption
    Redemption -.->|Feedback| Workers

    style C3 fill:#90EE90
    style V3 fill:#87CEEB
    style R3 fill:#FFD700
```

---

## 18. Trust Decision Tree

```mermaid
graph TB
    A[Assess Community Needs] --> B{Work velocity?}
    B -->|Low| C[Consider 10-12% rate]
    B -->|High| D[Consider 8-9% rate]

    C --> E{Seasonality?}
    E -->|Highly seasonal| C1[Add 1-2% premium]
    E -->|Year-round| C2[Base rate OK]

    D --> F{Community size?}
    F -->|Large 500+| D1[Add 1% for scale]
    F -->|Small <200| D2[Base rate OK]

    C1 --> G{Within 5-15%?}
    C2 --> G
    D1 --> G
    D2 --> G

    G -->|Yes| H[Propose to community]
    G -->|No| I[Adjust calculation]

    H --> J{Vote result}
    J -->|Pass| K[Implement]
    J -->|Fail| I

    style K fill:#90EE90
    style I fill:#FFB6C1
```

---

## Usage Instructions

### Viewing Diagrams

1. **Online**: Copy any diagram code and paste into [Mermaid Live Editor](https://mermaid.live)
2. **GitHub/GitLab**: Diagrams render automatically in markdown files
3. **VS Code**: Install Mermaid Preview extension
4. **Documentation**: Most modern documentation tools support Mermaid

### Editing Diagrams

- All diagrams use standard Mermaid.js syntax
- Modify text within brackets to update labels
- Adjust styling with `style` directives
- Test changes in Mermaid Live Editor before committing

### Export Options

- PNG/SVG: Export from Mermaid Live Editor
- PDF: Print from browser
- Code: Copy directly for integration

---

**Source Document:** [time-standard-token-design.md](./time-standard-token-design.md)
**Version:** 1.0
**Date:** 2025-01-01
