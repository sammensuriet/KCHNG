# KCHNG Merchant Onboarding Guide

**For cafés, restaurants, and meal providers joining the KCHNG ecosystem.**

---

## Overview

KCHNG is a community currency where **1000 KCHNG = 1 community meal**. As a merchant, you accept KCHNG from community members in exchange for meals, and participate in a circular economy backed by external subsidies that cover your real-world costs.

### Who This Is For

This guide is designed for:
- Community cafés in cooperative spaces
- Worker cooperatives
- Social enterprises prioritizing community impact
- Restaurants in eco-villages or intentional communities

---

## Economic Model

### The Core Equation

```
30 minutes verified work = 1000 KCHNG = 1 community meal
```

### How Subsidies Work

KCHNG is a **community contribution accounting system**. External subsidies (grants, DAO treasury, municipal support) bridge the gap between KCHNG and real-world costs:

```
┌─────────────────────────────────────────────────────┐
│ Worker earns 1000 KCHNG for 30 min community work   │
│                    ↓                                │
│ Worker spends 1000 KCHNG at your café               │
│                    ↓                                │
│ You provide a meal                                  │
│                    ↓                                │
│ You report activity → Receive fiat subsidy          │
│                    ↓                                │
│ Use fiat for rent, utilities, external suppliers    │
└─────────────────────────────────────────────────────┘
```

### Your Role

| You Provide | You Receive |
|-------------|-------------|
| Meals to community members | KCHNG (community credit) |
| KCHNG acceptance | Fiat subsidies for real costs |
| Community participation | Access to KCHNG ecosystem |

---

## Step-by-Step Onboarding

### Step 1: Create a Stellar Wallet

You need a Stellar wallet to hold and transact KCHNG.

**Option A: Freighter Wallet (Recommended for beginners)**
1. Install [Freighter](https://www.freighter.app/) browser extension
2. Create a new wallet and secure your seed phrase
3. Switch to the network your Trust uses (mainnet or testnet)

**Option B: Stellar CLI (For advanced users)**
```bash
# Install Stellar CLI
cargo install stellar-cli

# Create a new keypair
stellar keys generate --out-file merchant.json

# View your public key (your wallet address)
stellar keys show merchant
```

### Step 2: Get Initial KCHNG

You need a small amount of KCHNG to join a Trust (for transaction fees and initial activity).

**Options:**
- Receive from your Trust governor
- Receive from an existing community member
- Request from the KCHNG team

### Step 3: Join a Trust

A Trust is a community organization that sets its own demurrage rate and governs membership.

**Via CLI:**
```bash
# Join your community's trust
stellar contract invoke \
  --id CDMKVYIU6KAATZXLLFT6KTJCKXCWP3HPYNMA3HMEUUWYVCM5DJVZ5AQS \
  --source merchant \
  --network mainnet \
  -- join_trust \
  --member YOUR_ADDRESS \
  --trust_id TRUST_GOVERNOR_ADDRESS
```

**Via Frontend (when available):**
1. Connect your wallet
2. Navigate to "Join Trust"
3. Select your community trust
4. Confirm the transaction

### Step 4: Display Your Participation

Let customers know you accept KCHNG:

- Display "We accept KCHNG" signage
- Show your wallet address as a QR code
- List your café in community directories

### Step 5: Accept Payments

When a customer wants to pay with KCHNG:

**CLI Example:**
```bash
# Customer transfers 1000 KCHNG to your address
stellar contract invoke \
  --id CDMKVYIU6KAATZXLLFT6KTJCKXCWP3HPYNMA3HMEUUWYVCM5DJVZ5AQS \
  --source customer_wallet \
  --network mainnet \
  -- transfer \
  --from CUSTOMER_ADDRESS \
  --to YOUR_MERCHANT_ADDRESS \
  --amount 1000
```

**Frontend (when available):**
1. Customer scans your QR code
2. Customer enters amount (1000 KCHNG per meal)
3. Customer confirms transfer
4. You receive KCHNG instantly

---

## Pricing Guide

### Standard Pricing

| Item | KCHNG Price | Notes |
|------|-------------|-------|
| Basic meal | 1000 KCHNG | Standard community meal |
| Larger portion | 1500 KCHNG | 1.5× standard |
| Small plate / snack | 500 KCHNG | 0.5× standard |
| Beverage only | 200-300 KCHNG | ~1/4 to 1/3 standard |

### Pricing Principles

1. **1000 KCHNG represents real meal value** - not a discount or premium
2. **Keep prices stable** - community members rely on predictable pricing
3. **Be transparent** - display KCHNG prices alongside any fiat prices

---

## Managing Your KCHNG

### Receiving KCHNG

KCHNG received goes directly to your wallet balance. No action required.

### Spending KCHNG

Use your KCHNG within the community:
- Pay community suppliers who accept KCHNG
- Purchase from other KCHNG merchants
- Pay for community services (care, training, etc.)

### Understanding Demurrage

KCHNG has a **demurrage (decay) rate** of ~12% annually. This incentivizes circulation.

**What this means for you:**
- Don't hoard KCHNG - spend or circulate it
- Balance will slowly decrease if inactive
- Each transaction resets your activity timer

**Practical tip:** Aim to circulate KCHNG within 28 days to avoid demurrage.

### Transfer Cooldown

There is a **24-hour cooldown** between transfers from the same account. Plan larger transactions accordingly.

---

## Subsidy Reporting

### Tracking Activity

Keep records of:
- Meals served for KCHNG (count and dates)
- KCHNG received (transaction hashes)
- Any issues or disputes

### Claiming Subsidies

Subsidy distribution varies by Trust. Common methods:

| Model | How It Works |
|-------|--------------|
| **Monthly reporting** | Submit meal count, receive fiat transfer |
| **Per-meal matching** | $X fiat per 1000 KCHNG earned |
| **Quarterly grants** | Lump sum based on participation |

Contact your Trust governor for specifics.

---

## Best Practices

### For Smooth Operations

1. **Test with small amounts first** - Get comfortable with the flow
2. **Have a backup payment method** - KCHNG is new; some customers may need alternatives
3. **Train your staff** - Ensure everyone understands how to accept KCHNG
4. **Stay active** - Regular transactions prevent demurrage on your balance

### For Community Building

1. **Promote KCHNG to customers** - Help grow the ecosystem
2. **Connect with other merchants** - Build supplier relationships
3. **Participate in governance** - Vote on Trust proposals
4. **Share feedback** - Help improve the system

---

## Troubleshooting

### Common Issues

| Issue | Solution |
|-------|----------|
| **Transfer failed** | Check 24-hour cooldown hasn't been exceeded |
| **Balance lower than expected** | Demurrage may have been applied; make a transaction to reset |
| **Can't join Trust** | Contact Trust governor; you may need an invitation |
| **Customer can't pay** | Verify they have sufficient balance (after demurrage) |

### Getting Help

- **Trust Governor**: Your primary contact for Trust-specific issues
- **Community Forum**: Connect with other merchants and users
- **Technical Issues**: Submit issues at [github.com/kachi-ng/KCHNG](https://github.com/kachi-ng/KCHNG)

---

## Contract Reference

### Key Functions for Merchants

| Function | Purpose |
|----------|---------|
| `join_trust(member, trust_id)` | Join a community trust |
| `balance(account)` | Check your KCHNG balance |
| `transfer(from, to, amount)` | Send KCHNG (requires auth) |
| `get_account_trust(account)` | Verify your trust membership |
| `get_account(account)` | View full account details |

### Contract Addresses

| Network | Contract ID |
|---------|-------------|
| **Mainnet** | `CDMKVYIU6KAATZXLLFT6KTJCKXCWP3HPYNMA3HMEUUWYVCM5DJVZ5AQS` |
| **Testnet** | `CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX` |

---

## Glossary

| Term | Definition |
|------|------------|
| **Trust** | A community organization with its own demurrage rate and governance |
| **Governor** | The address that manages a Trust |
| **Demurrage** | A decay rate on inactive balances, incentivizing circulation |
| **KCHNG** | The community currency token (1000 KCHNG = 1 meal) |
| **Subsidy** | External fiat funding that bridges KCHNG to real-world costs |

---

## Next Steps

1. ✅ Create your Stellar wallet
2. ✅ Get initial KCHNG from your community
3. ✅ Join your community Trust
4. ✅ Set up payment acceptance (QR code, signage)
5. ✅ Serve your first KCHNG customer!
6. ✅ Report activity for subsidy claims

---

**Welcome to the KCHNG community!**

Questions? Contact your Trust governor or visit [kachi.ng](https://kachi.ng).
