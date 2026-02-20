# KCHNG User Journeys by Work Category

**Document Version**: 1.0
**Last Updated**: 2026-02-20
**Purpose**: Marketing and product guidance for user acquisition, engagement, and retention

---

## Executive Summary

This document maps user journeys for KCHNG (kachi.ng) across four primary work categories. Each journey details how users discover, join, earn, spend, and govern within the platform.

**Core Economic Equation**: `30 min verified work = 1000 KCHNG = 1 community meal`

**Platform Architecture**:
- Stellar blockchain with Soroban smart contracts
- Wallet connection via Stellar wallets (Freighter, Lobstr, etc.)
- Trusts = community organizations with custom demurrage rates (5-15% annually)
- Work verification by community peers (minimum 2 verifiers per claim)
- Governance through proposal creation and voting

---

## Work Category Overview

| Category | Work Type (Contract) | Multiplier | Example Activities |
|----------|---------------------|------------|-------------------|
| Primary Care | Basic (0) | 1.0x | Elder care, farming, cooking, childcare |
| Product Manufacturing | Skilled (1) | 1.3x | Crafting goods, food processing, textiles |
| Service Economy | Training (2) | 1.5x | Teaching, tour guiding, hospitality training |
| Experience Economy | Emergency (3) | 2.0x | Cultural events, crisis response, urgent care |

---

## 1. PRIMARY CARE - User Journey

### Persona: Maria, 52, Elder Care Worker

**Background**: Maria provides daily care for elderly community members. She cooks meals, assists with mobility, and provides companionship. She has been doing this work informally for years but lacks formal recognition or consistent compensation.

### 1.1 Discovery and Onboarding

| Stage | Action | Platform Feature | Touchpoint |
|-------|--------|------------------|------------|
| **Awareness** | Hears about KCHNG from community health worker | Word of mouth | Local clinic partnership |
| **Interest** | Visits kachi.ng landing page | Landing page hero section | Website |
| **Understanding** | Reads "30 min work = 1000 KCHNG = 1 meal" | Value proposition banner | Website |
| **Consideration** | Explores "Why KCHNG?" benefits | Benefits grid | Website |
| **Action** | Clicks "Dashboard" to connect wallet | CTA section | Website |

**Discovery Channels**:
- Community health worker referrals
- Local clinic/medical center posters
- Food bank partnerships
- Community center workshops
- WhatsApp group mentions

### 1.2 Wallet Connection and Trust Joining

**Step-by-Step Flow**:

```
1. Dashboard Page
   - Sees "Please connect your wallet" message
   - Clicks "Connect Wallet" button
   - StellarWalletsKit modal opens

2. Wallet Selection
   - Chooses Freighter (recommended for beginners)
   - Redirects to wallet for approval
   - Returns to dashboard with connected state

3. Account Initialization
   - Contract automatically creates account data structure
   - Balance: 0 KCHNG
   - Trust membership: None

4. Browse Trusts (/trusts)
   - Views available community trusts
   - Sees "Urban Elder Care Trust" with 12% rate, 234 members
   - Clicks "Join Trust"
   - Signs transaction
   - Becomes trust member

5. Trust Benefits
   - Access to trust-specific verifiers
   - Participation in trust governance
   - Community meal provider network
```

**Pain Points**:
- Wallet creation can be intimidating for non-technical users
- Transaction signing terminology is confusing
- Understanding demurrage rates requires education

**Opportunities**:
- Guided wallet creation wizard
- Video tutorials in local languages
- Trust recommendation engine based on location/work type

### 1.3 Work Submission Process

**Evidence Requirements for Primary Care**:

| Evidence Type | How to Capture | IPFS Upload |
|--------------|----------------|-------------|
| Photo | Smartphone photo of care activity | Via platform upload |
| GPS Location | Automatic from phone | Included in claim |
| Time Log | Manual entry (15 min minimum) | Form submission |
| Witness Statement | Optional verifier pre-approval | Text field |

**Submission Flow**:

```
1. Navigate to /work
2. Select "Submit Work" tab
3. Choose Work Type: "Basic Care/Agriculture" (1.0x)
4. Enter Minutes Worked: 90
5. Upload evidence to IPFS (photo of elder care activity)
6. Enter GPS coordinates (auto-filled or manual)
7. Preview calculation:
   - 90 minutes / 30 = 3 base units
   - 3 x 1000 = 3000 KCHNG
   - At 1.0x multiplier = 3000 KCHNG = 3 meals
8. Submit claim
9. Wait for verification (minimum 2 verifiers from trust)
```

**Verification Process**:

```
Claim Status Timeline:
Day 0: Submitted (Pending)
Day 1-3: Verifiers assigned randomly from trust
Day 1-3: Each verifier reviews:
  - Photo evidence quality
  - GPS within expected location
  - Time claim reasonable for activity
  - Worker reputation history
Day 3: 2 approvals received
Day 3: Status changes to Approved
Day 3: 3000 KCHNG minted to Maria's account
```

**Pain Points**:
- Photo upload requires IPFS understanding
- Wait time for verification creates anxiety
- No immediate feedback on evidence quality

**Opportunities**:
- Evidence quality checklist before submission
- Real-time verification progress tracker
- Push notifications for status changes

### 1.4 Earning KCHNG

**Calculation Formula**:
```
Tokens = (minutes_worked / 30) * 1000 * multiplier / 100

Example (Maria's 90-minute care session):
= (90 / 30) * 1000 * 1.0
= 3 * 1000 * 1.0
= 3000 KCHNG
```

**Weekly Earning Potential**:

| Hours/Week | Minutes | KCHNG Earned | Meals Equivalent |
|------------|---------|--------------|------------------|
| 5 hours | 300 | 10,000 | 10 meals |
| 10 hours | 600 | 20,000 | 20 meals |
| 20 hours | 1200 | 40,000 | 40 meals |
| 40 hours | 2400 | 80,000 | 80 meals |

**Account Dashboard View**:

```
Balance: 15,000 KCHNG
Last Activity: 2 days ago
Circulation Status: Active
Trust: Urban Elder Care Trust
Contribution Hours: 12.5h
Grace Periods Used: 0/3
```

### 1.5 Spending KCHNG

**Spending Options for Primary Care Workers**:

| Option | Process | KCHNG Cost |
|--------|---------|------------|
| Community Meal | Visit meal provider, scan QR code | 1000 KCHNG |
| Groceries | Partner food stores | Variable |
| Transportation | Local transport credits | 500 KCHNG/trip |
| Healthcare | Community clinic services | 2000 KCHNG/visit |
| Transfer to Peer | P2P transfer via wallet | Any amount |

**Transaction Flow**:

```
1. Maria visits community meal provider
2. Provider displays QR code (their wallet address)
3. Maria scans with wallet app
4. Enters amount: 1000 KCHNG
5. Signs transaction
6. Provider receives 1000 KCHNG
7. Maria receives meal
8. Both accounts update last_activity timestamp
9. Demurrage clock resets for Maria
```

**Demurrage Impact**:

```
Scenario: Maria earns 10,000 KCHNG, inactive for 30 days
Trust Rate: 12% annual (1% monthly)

After 30 days inactive:
- Demurrage applied: ~1% = 100 KCHNG
- Remaining balance: 9,900 KCHNG
- Meals still redeemable: 9.9

Incentive: Spend or transfer within 7 days to avoid any decay
```

### 1.6 Governance Participation

**Voting Rights**: All trust members can vote on proposals

**Governance Actions**:

| Action | Requirement | Process |
|--------|-------------|---------|
| View Proposals | Trust membership | Navigate to /governance |
| Vote on Proposal | Trust membership, proposal in voting period | Click "Vote" button |
| Create Proposal | Trust membership | Submit via "Create Proposal" form |

**Proposal Types Relevant to Primary Care**:
- Rate Change: Adjust trust demurrage rate
- Trust Parameters: Modify verification requirements
- Emergency: Crisis measures (e.g., natural disaster response)

**Voting Flow**:

```
1. Navigate to /governance
2. See proposal: "Reduce minimum verifiers from 2 to 1 for basic care"
3. Review 7-day discussion period comments
4. Proposal enters 3-day voting period
5. Click "Vote" button
6. Select "For" or "Against"
7. Sign transaction
8. Vote recorded on-chain
9. After voting period: see results
10. If approved: 30-day implementation notice
```

### 1.7 Pain Points and Opportunities Summary

**Pain Points**:

| Pain Point | Severity | Impact |
|------------|----------|--------|
| Technical barrier (wallet) | High | Prevents onboarding |
| IPFS evidence upload | Medium | Confusion, abandoned claims |
| Verification wait time | Medium | User anxiety |
| Demurrage complexity | Low | Misunderstanding value |
| Limited spending options | Medium | Token utility concern |

**Opportunities**:

| Opportunity | Impact | Implementation |
|-------------|--------|----------------|
| Mobile-first wallet onboarding | High | In-app wallet creation |
| Evidence capture assistant | High | Photo guidelines, quality checker |
| Verification progress UI | Medium | Real-time status updates |
| Meal provider map | High | Directory with locations |
| Care worker community | Medium | Trust-specific forums |

---

## 2. PRODUCT MANUFACTURING - User Journey

### Persona: Joseph, 38, Local Furniture Maker

**Background**: Joseph runs a small workshop crafting furniture and home goods from reclaimed wood. He has steady local customers but struggles with seasonal income fluctuation and lack of access to formal credit.

### 2.1 Discovery and Onboarding

| Stage | Action | Platform Feature | Touchpoint |
|-------|--------|------------------|------------|
| **Awareness** | Sees KCHNG flyer at local hardware store | Print marketing | Hardware store partnership |
| **Interest** | Scans QR code to visit kachi.ng | QR code campaign | Physical flyer |
| **Understanding** | Watches 2-min explainer video | Video embed | Landing page |
| **Consideration** | Sees "Skilled Labor 1.3x multiplier" | Work types info | Work page |
| **Action** | Connects wallet and joins "Artisan Trust" | Trust joining | /trusts |

**Discovery Channels**:
- Hardware store partnerships
- Craft fair booths
- Local business association meetings
- Maker space communities
- Instagram/local Facebook groups

### 2.2 Work Submission Process

**Evidence Requirements for Manufacturing**:

| Evidence Type | How to Capture | Verification Criteria |
|--------------|----------------|----------------------|
| Finished Product Photo | Clear product image | Visible craftsmanship |
| Workshop/GPS | Photo in workspace | Location verification |
| Time Log | Hours spent crafting | Reasonable for product complexity |
| Materials Receipt | Optional | Proves local sourcing |

**Submission Flow**:

```
1. Joseph completes a dining table (8 hours work)
2. Navigate to /work
3. Select Work Type: "Skilled Care/Heavy Labor" (1.3x)
4. Enter Minutes Worked: 480 (8 hours)
5. Upload photos: finished table + workshop
6. Enter IPFS hash for evidence bundle
7. Preview calculation:
   - 480 minutes / 30 = 16 base units
   - 16 x 1000 = 16,000 base KCHNG
   - At 1.3x multiplier = 20,800 KCHNG = 20.8 meals
8. Submit claim
9. Verification by 2 Artisan Trust verifiers
10. Approval and minting
```

**Verification Considerations for Manufacturing**:
- Product complexity vs. time claimed
- Quality assessment through photos
- Market value cross-reference (optional)
- Repeat claim patterns (fraud detection)

### 2.3 Earning KCHNG

**Multiplier Advantage for Skilled Work**:

| Work Category | Multiplier | 8-Hour Output | Meals |
|--------------|------------|---------------|-------|
| Basic (Care) | 1.0x | 16,000 KCHNG | 16 |
| Skilled (Manufacturing) | 1.3x | 20,800 KCHNG | 20.8 |
| Training (Teaching) | 1.5x | 24,000 KCHNG | 24 |
| Emergency | 2.0x | 32,000 KCHNG | 32 |

**Monthly Earning Scenario**:

```
Joseph's Workshop Output (Month):
- 4 dining tables (8h each) = 32h at 1.3x = 83,200 KCHNG
- 6 chairs (3h each) = 18h at 1.3x = 46,800 KCHNG
- 3 repair jobs (2h each) = 6h at 1.0x = 12,000 KCHNG

Total: 142,000 KCHNG = 142 meals
```

### 2.4 Spending KCHNG

**Spending Options for Manufacturers**:

| Option | Relevance | KCHNG Use |
|--------|-----------|-----------|
| Raw Materials | High | Purchase from partner suppliers |
| Tool Rental | High | Equipment sharing network |
| Worker Meals | High | Feed apprentices/assistants |
| Market Stall Fees | Medium | Local market participation |
| Transfer to Suppliers | High | Pay supply chain partners |

**B2B Transaction Flow**:

```
1. Joseph needs wood from partner lumber yard
2. Lumber yard quotes 15,000 KCHNG
3. Joseph initiates transfer from wallet
4. Signs transaction
5. Lumber yard receives KCHNG
6. Joseph receives materials
7. Both accounts active (demurrage reset)

Cross-Trust Exchange (if needed):
- Joseph in Artisan Trust (10% rate)
- Lumber yard in Cooperative Trust (8% rate)
- Exchange rate: (1 - 0.10) / (1 - 0.08) = 0.978
- Joseph pays 15,335 KCHNG, yard receives 15,000
```

### 2.5 Governance Participation

**Manufacturing-Relevant Governance**:

| Proposal Type | Example | Impact on Joseph |
|--------------|---------|------------------|
| Rate Change | Increase trust rate to 12% | Faster circulation needed |
| Trust Parameters | Raise skilled multiplier to 1.5x | Higher earnings |
| Protocol Upgrade | Add "Apprenticeship" work type | New earning category |
| Emergency | Crisis rate increase for reconstruction | Post-disaster work surge |

**Governance Engagement**:

```
Joseph's Governance Profile:
- Trust: Artisan Trust
- Member since: Month 3
- Proposals voted: 4
- Proposals created: 1 (failed quorum)

Created Proposal: "Recognize 'Materials Preparation' as separate work type"
- Status: Rejected (45% quorum, needed 40% - passed)
- Vote result: 55% against
- Learning: Better community discussion needed before proposal
```

### 2.6 Pain Points and Opportunities Summary

**Pain Points**:

| Pain Point | Severity | Impact |
|------------|----------|--------|
| Time tracking for irregular work | High | Accurate claims difficult |
| Product quality verification | Medium | Subjective assessments |
| Seasonal income variation | High | Cash flow management |
| Limited supplier network | High | Spending options constraint |
| Cross-trust exchange friction | Low | Rate confusion |

**Opportunities**:

| Opportunity | Impact | Implementation |
|-------------|--------|----------------|
| Time tracking app | High | Mobile timer with GPS |
| Product portfolio system | Medium | Link products to claims |
| Supplier directory | High | Curated KCHNG-accepting vendors |
| Seasonal credit (grace periods) | High | Automatic hardship protection |
| Manufacturing trust collective | Medium | Group buying power |

---

## 3. SERVICE ECONOMY - User Journey

### Persona: Amina, 29, Tour Guide and Hospitality Trainer

**Background**: Amina leads cultural tours and trains new hospitality workers. She values knowledge transfer and sees teaching as her primary contribution to the community economy.

### 3.1 Discovery and Onboarding

| Stage | Action | Platform Feature | Touchpoint |
|-------|--------|------------------|------------|
| **Awareness** | Attends KCHNG community workshop | In-person event | Community center |
| **Interest** | Asks questions about "Training" multiplier | Q&A session | Workshop |
| **Understanding** | Learns teaching earns 1.5x multiplier | Educational content | Workshop |
| **Consideration** | Sees tourism workers in local trust | Trust member list | /trusts |
| **Action** | Joins "Hospitality & Tourism Trust" | Trust onboarding | /trusts |

**Discovery Channels**:
- Community workshops
- Tourism association meetings
- Hospitality training programs
- Hotel staff networks
- Cultural center partnerships

### 3.2 Work Submission Process

**Evidence Requirements for Service/Teaching**:

| Evidence Type | How to Capture | Verification Criteria |
|--------------|----------------|----------------------|
| Session Photo | Group photo with participants | Attendance evidence |
| Curriculum/Outline | Text document | Teaching structure |
| GPS Location | Tour route or training venue | Location verification |
| Participant Feedback | Optional testimonials | Quality indicator |
| Time Log | Session duration | Reasonable for content |

**Submission Flow**:

```
1. Amina leads 3-hour cultural walking tour (5 tourists)
2. Navigate to /work
3. Select Work Type: "Training/Teaching" (1.5x)
4. Enter Minutes Worked: 180
5. Upload evidence bundle:
   - Group photo at start location
   - Tour route map (screenshot)
   - Brief description of content covered
6. Preview calculation:
   - 180 minutes / 30 = 6 base units
   - 6 x 1000 = 6,000 base KCHNG
   - At 1.5x multiplier = 9,000 KCHNG = 9 meals
7. Submit claim
```

**Verification for Teaching/Training**:

```
Verification Criteria for Training Claims:
- Session duration reasonable for content
- Participant count matches evidence
- Content aligns with skill category
- Trainer reputation history positive
- GPS consistent with claimed location

Verification Turnaround:
- Tourism trust: 1-2 days (active verifier pool)
- Quick approval for established trainers
- New trainers: additional scrutiny period
```

### 3.3 Earning KCHNG

**Multiplier Structure for Services**:

| Service Type | Classification | Multiplier | Example |
|--------------|---------------|------------|---------|
| Basic guiding | Basic | 1.0x | Simple walking tours |
| Specialized training | Training | 1.5x | Hospitality certification |
| Language instruction | Training | 1.5x | Teaching local dialect |
| Emergency response | Emergency | 2.0x | Tourist medical assistance |
| Skill certification | Training | 1.5x | Formal assessment |

**Monthly Earning Scenario**:

```
Amina's Service Output (Month):
- 8 cultural tours (3h each) = 24h at 1.0x = 48,000 KCHNG
- 4 hospitality training sessions (4h each) = 16h at 1.5x = 48,000 KCHNG
- 2 language lessons (2h each) = 4h at 1.5x = 12,000 KCHNG

Total: 108,000 KCHNG = 108 meals
```

### 3.4 Spending KCHNG

**Spending Options for Service Workers**:

| Option | Relevance | KCHNG Use |
|--------|-----------|-----------|
| Meals during tours | High | Participant lunches |
| Transportation | High | Tour logistics |
| Training materials | High | Handouts, supplies |
| Venue rental | Medium | Training space |
| Marketing | Low | Tour promotion |

### 3.5 Governance Participation

**Service Economy Governance Interests**:

| Area | Interest Level | Typical Proposals |
|------|---------------|-------------------|
| Work type definitions | High | New service categories |
| Training standards | High | Certification requirements |
| Multiplier adjustments | Medium | Rate changes for specialties |
| Cross-trust cooperation | Medium | Tourism network partnerships |

**Example Proposal Created by Amina**:

```
Title: "Add 'Cultural Preservation' as Recognized Work Type"
Type: Trust Parameters
Description: Cultural documentation and preservation activities
should be recognized as contributing to community value.
Proposed classification: Training/Teaching (1.5x)

Status: In Review Period
Days remaining: 4
Community discussion: 12 comments
```

### 3.6 Pain Points and Opportunities Summary

**Pain Points**:

| Pain Point | Severity | Impact |
|------------|----------|--------|
| Variable service duration | Medium | Time tracking complexity |
| Participant verification | Medium | Proving attendance |
| Seasonal tourism fluctuation | High | Income instability |
| Cross-service categorization | Low | Classification confusion |
| Tourist payment integration | High | External currency barrier |

**Opportunities**:

| Opportunity | Impact | Implementation |
|-------------|--------|----------------|
| Session templates | High | Pre-configured claim types |
| Attendance QR system | Medium | Participant check-in |
| Seasonal grace periods | High | Tourism off-season protection |
| Tourist KCHNG purchase | High | Fiat on-ramp for visitors |
| Service portfolio | Medium | Trainer credibility display |

---

## 4. EXPERIENCE ECONOMY - User Journey

### Persona: Kofi, 45, Cultural Event Organizer and Crisis Responder

**Background**: Kofi organizes community cultural events and serves as an emergency response coordinator during crises. His work ranges from celebration planning to urgent community support.

### 4.1 Discovery and Onboarding

| Stage | Action | Platform Feature | Touchpoint |
|-------|--------|------------------|------------|
| **Awareness** | Receives KCHNG for emergency flood response | Direct experience | Crisis response |
| **Interest** | Learns about "Emergency Care" 2.0x multiplier | Word of mouth | Community network |
| **Understanding** | Realizes cultural events also qualify | Platform education | Community forum |
| **Consideration** | Sees event organizers in Creative Trust | Trust discovery | /trusts |
| **Action** | Joins "Community Response & Culture Trust" | Trust onboarding | /trusts |

**Discovery Channels**:
- Crisis response volunteer networks
- Cultural association memberships
- Community event planning groups
- Emergency preparedness organizations
- Festival organizing committees

### 4.2 Work Submission Process

**Evidence Requirements for Experience/Emergency Work**:

| Evidence Type | How to Capture | Verification Criteria |
|--------------|----------------|----------------------|
| Event documentation | Photos/videos | Event occurrence proof |
| Attendance records | Sign-in sheets, headcount | Scale verification |
| GPS/Location | Venue coordinates | Location confirmation |
| Emergency certification | Official documentation | Authority verification |
| Time log | Total hours engaged | Reasonable for scope |

**Dual Work Type Classification**:

```
Cultural Events (Normal Operations):
- Work Type: Basic or Training (1.0x - 1.5x)
- Example: 6-hour festival organization = Basic (1.0x)
- Calculation: 6h = 12,000 KCHNG

Crisis Response (Emergency Operations):
- Work Type: Emergency Care (2.0x)
- Example: 6-hour flood response = Emergency (2.0x)
- Calculation: 6h x 2.0 = 24,000 KCHNG
```

**Emergency Work Verification**:

```
Emergency Claim Process:
1. Crisis event occurs (verified by oracle or community)
2. Emergency workers submit claims with:
   - Official incident reference (if available)
   - Location and time documentation
   - Role description
3. Expedited verification (24-48 hours)
4. Emergency multiplier applied (2.0x)
5. Grace period often granted for affected workers

Oracle Verification:
- Trusted community leaders
- Emergency response coordinators
- Government/NGO representatives
```

### 4.3 Earning KCHNG

**Experience Economy Earning Structure**:

| Activity | Classification | Multiplier | Hours | KCHNG Earned |
|----------|---------------|------------|-------|--------------|
| Festival planning | Basic | 1.0x | 20 | 40,000 |
| Cultural performance | Skilled | 1.3x | 4 | 10,400 |
| Workshop facilitation | Training | 1.5x | 8 | 24,000 |
| Flood response | Emergency | 2.0x | 12 | 48,000 |
| Medical emergency | Emergency | 2.0x | 6 | 24,000 |

**Crisis Response Monthly Example**:

```
Kofi's Mixed Month (Crisis + Events):
Week 1: Festival preparation (15h at 1.0x) = 30,000 KCHNG
Week 2: Festival execution (10h at 1.0x) = 20,000 KCHNG
Week 3: Flood emergency response (20h at 2.0x) = 80,000 KCHNG
Week 4: Community recovery support (15h at 1.3x) = 39,000 KCHNG

Total: 169,000 KCHNG = 169 meals
```

### 4.4 Spending KCHNG

**Spending Options for Experience Economy Workers**:

| Option | Relevance | KCHNG Use |
|--------|-----------|-----------|
| Event supplies | High | Materials, decorations |
| Venue costs | High | Space rental |
| Emergency supplies | High | Crisis response equipment |
| Team meals | High | Volunteer feeding |
| Equipment maintenance | Medium | Tools, gear upkeep |
| Training/certification | Medium | Skill development |

### 4.5 Governance Participation

**Experience Economy Governance Focus**:

| Area | Interest Level | Typical Proposals |
|------|---------------|-------------------|
| Emergency multiplier triggers | Very High | Criteria for 2.0x rate |
| Event work recognition | High | Festival planning categories |
| Grace period policies | Very High | Crisis recovery extensions |
| Cross-trust coordination | High | Regional emergency networks |

**Example Emergency Proposal**:

```
Title: "Automatic Grace Period for Crisis Zone Workers"
Type: Emergency
Description: Workers in officially declared crisis zones should
automatically receive 30-day grace periods without requiring
individual oracle verification.

Proposed implementation:
- Oracle declares crisis zone
- All trust members in zone receive grace
- 30-day duration, extendable by vote
- No impact on yearly grace limits

Status: Voting Period
Votes For: 67
Votes Against: 12
Quorum: 45% (needed 40%)
Projected: Approved
```

### 4.6 Grace Periods (Special Relevance)

**Grace Period Requirements**:
- Minimum 100 contribution hours before eligible
- 90-day cooldown between grace periods
- Maximum 3 grace periods per year
- Activated by trusted community oracles (not self-requested)

**Grace Period Types for Experience Economy**:

| Type | Trigger | Duration | Verification |
|------|---------|----------|--------------|
| Emergency | Crisis event | Up to 90 days | Oracle verified |
| Illness | Health issue | Up to 60 days | Medical documentation |
| Community | Hardship vote | Up to 180 days | Community vote |

**Kofi's Grace Period Experience**:

```
Scenario: Flood affects Kofi's community

Prerequisites:
- Kofi has 150+ contribution hours (eligible)
- No grace period in last 90 days (cooldown clear)

Day 0: Flood event occurs
Day 1: Trust governor or community member contacts oracle
Day 1: Oracle verifies crisis zone declaration
Day 2: Oracle activates 60-day grace period for affected members
Day 2-62: No demurrage applied to Kofi's balance
Day 62: Grace expires, normal demurrage resumes

Impact:
- Balance protected during recovery
- Can focus on response work without penalty
- Community solidarity demonstrated
```

### 4.7 Pain Points and Opportunities Summary

**Pain Points**:

| Pain Point | Severity | Impact |
|------------|----------|--------|
| Emergency verification speed | Critical | Delayed compensation |
| Work type classification | Medium | Multiplier confusion |
| Event income variability | High | Budget planning difficulty |
| Grace period limits | Medium | Extended crisis coverage |
| Oracle availability | High | Verification bottleneck |

**Opportunities**:

| Opportunity | Impact | Implementation |
|-------------|--------|----------------|
| Automatic crisis detection | Critical | Oracle automation |
| Pre-approved event categories | High | Simplified submission |
| Emergency advance payments | Critical | Bridge loans in KCHNG |
| Extended grace for responders | High | Policy adjustment |
| Multi-oracle verification | Medium | Redundancy, speed |

---

## Cross-Cutting Insights

### Platform-Wide Pain Points

| Pain Point | Affected Categories | Severity | Solution Priority |
|------------|---------------------|----------|-------------------|
| Wallet onboarding complexity | All | Critical | 1 |
| IPFS evidence upload | All | High | 2 |
| Verification wait time | All | High | 3 |
| Limited spending network | All | High | 4 |
| Demurrage education | All | Medium | 5 |
| Mobile experience | All | High | 6 |

### Platform-Wide Opportunities

| Opportunity | Impact | Effort | Priority |
|-------------|--------|--------|----------|
| In-app wallet creation | Very High | High | 1 |
| Mobile-native evidence capture | Very High | Medium | 2 |
| Real-time verification status | High | Low | 3 |
| Meal provider directory | Very High | Medium | 4 |
| Transaction push notifications | High | Low | 5 |
| Trust recommendation engine | Medium | Medium | 6 |

### Retention Strategies by Category

| Category | Retention Lever | Implementation |
|----------|-----------------|----------------|
| Primary Care | Community belonging | Trust-specific events, peer recognition |
| Manufacturing | Skill recognition | Portfolio showcase, quality badges |
| Service Economy | Professional development | Certification tracking, training ladder |
| Experience Economy | Mission alignment | Crisis impact reports, community gratitude |

### Acquisition Channels by Category

| Category | Best Channels | Content Focus |
|----------|--------------|---------------|
| Primary Care | Clinics, food banks, community centers | "Value your care work" |
| Manufacturing | Hardware stores, maker spaces, trade associations | "Skills multiply earnings" |
| Service Economy | Tourism boards, training institutes | "Teaching builds community" |
| Experience Economy | Emergency networks, cultural associations | "Crisis response rewards" |

---

## Metrics and KPIs

### User Journey Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Wallet connection rate | 80% of landing visitors | Analytics |
| Trust join rate | 90% of connected users | Contract events |
| First claim submission | 70% within 7 days | Contract events |
| Claim approval rate | 85%+ | Contract events |
| Repeat claims (weekly) | 60% of active users | Contract events |
| Balance spent (monthly) | 50%+ of earned | Transaction volume |
| Governance participation | 40%+ vote on proposals | Contract events |
| 30-day retention | 70% | Cohort analysis |

### Category-Specific Metrics

| Category | Key Metric | Target |
|----------|-----------|--------|
| Primary Care | Weekly care hours logged | 10+ hours/user |
| Manufacturing | Product claims/month | 8+ claims/user |
| Service Economy | Training sessions delivered | 4+ sessions/user/month |
| Experience Economy | Emergency response activation | 100% within 24h of crisis |

---

## Appendix: Platform Feature Reference

### Work Type Multipliers

```typescript
enum WorkType {
  Basic = 0,      // 1.0x - Basic care, agriculture, cooking
  Skilled = 1,    // 1.3x - Skilled care, heavy labor, manufacturing
  Training = 2,   // 1.5x - Teaching, skills transfer, hospitality
  Emergency = 3,  // 2.0x - Emergency care, crisis response
}
```

### Claim Status Flow

```typescript
enum ClaimStatus {
  Pending = 0,    // Waiting for verification
  Approved = 1,   // Approved and tokens minted
  Rejected = 2,   // Rejected by verifiers
  Expired = 3,    // Verification window expired
}
```

### Proposal Types

```typescript
enum ProposalType {
  RateChange = 0,        // Change trust demurrage rate
  TrustParameters = 1,   // Adjust trust parameters
  ProtocolUpgrade = 2,   // Protocol-level upgrade
  Emergency = 3,         // Emergency measure (crisis exception)
}
```

### Key Contract Methods

| Method | Purpose | User Action |
|--------|---------|-------------|
| `submit_work_claim()` | Submit work for verification | Earn KCHNG |
| `approve_work_claim()` | Verifier approves work | Verify peers |
| `reject_work_claim()` | Verifier rejects work | Quality control |
| `transfer()` | Send KCHNG to another account | Spend/share |
| `join_trust()` | Join a community trust | Community membership |
| `create_proposal()` | Create governance proposal | Governance |
| `vote_on_proposal()` | Vote on proposals | Governance |
| `activate_grace_period()` | Oracle activates demurrage pause | Hardship protection (oracle-only) |
| `register_verifier()` | Become a work verifier | Verification role |

---

**Document Status**: Complete
**Next Review**: Quarterly, or upon significant platform updates
**Owner**: Marketing / Brand Team
