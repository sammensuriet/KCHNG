#!/bin/bash
# Verify Time Capsule Test Results
# Run this 30+ days after setup
# Tests TWO accounts for better coverage

echo "╔════════════════════════════════════════════════════════════════════════╗"
echo "║     KCHNG DEMURRAGE TIME CAPSULE VERIFICATION                          ║"
echo "╚════════════════════════════════════════════════════════════════════════╝"
echo ""

CONTRACT_ID="CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX"
NETWORK="testnet"
TIME_CAPSULE="/home/pokho/dev/KCHNG/tests/regression/time_capsule_data.json"

echo "Verification date: $(date -Iseconds)"
echo ""

# Load time capsule data
if [ ! -f "$TIME_CAPSULE" ]; then
    echo "❌ ERROR: Time capsule data not found!"
    echo "   Expected: $TIME_CAPSULE"
    exit 1
fi

echo "Loading time capsule data..."
echo ""

# Get current time
CURRENT_TIME=$(date +%s)

# Test each account
echo "╔════════════════════════════════════════════════════════════════════════╗"
echo "║                      ACCOUNT VERIFICATION                             ║"
echo "╚════════════════════════════════════════════════════════════════════════╝"
echo ""

# Account 1: Admin
echo "Account 1: Admin (kchng_admin)"
echo "─────────────────────────────────────────────────────────────────"

ACCOUNT1_KEY="kchng_admin"
ACCOUNT1_PUBKEY="GCW4XHQLIK3VHXUGNXJ4NFLW5JDWZZ3UT2XJCYTZSO6AQJZUUDB7RVMS"
INITIAL_BALANCE_1=699999
EXPECTED_WORKING_1=693000
EXPECTED_BROKEN_1=699999

CURRENT_BALANCE_1=$(soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ACCOUNT1_KEY \
  --network $NETWORK \
  -- balance \
  --account $ACCOUNT1_KEY 2>&1 | grep -oP '\d+')

CURRENT_ACTIVITY_1=$(soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ACCOUNT1_KEY \
  --network $NETWORK \
  -- get_account \
  --account $ACCOUNT1_KEY 2>&1 | grep -oP '"last_activity":\s*\K[0-9]+')

ELAPSED_1=$(( (CURRENT_TIME - CURRENT_ACTIVITY_1) / 86400 ))

echo "  Initial balance: $INITIAL_BALANCE_1 KCHNG"
echo "  Current balance: $CURRENT_BALANCE_1 KCHNG"
echo "  Last activity: $CURRENT_ACTIVITY_1"
echo "  Days elapsed: $ELAPSED_1"
echo ""

if [ $ELAPSED_1 -ge 30 ]; then
    LOSS_1=$((INITIAL_BALANCE_1 - CURRENT_BALANCE_1))
    echo "  Balance change: $LOSS_1 KCHNG"
    echo ""

    if [ $CURRENT_BALANCE_1 -eq $INITIAL_BALANCE_1 ]; then
        echo "  🔴 RESULT: No demurrage applied (BUG CONFIRMED for account 1)"
    elif [ $CURRENT_BALANCE_1 -le $EXPECTED_WORKING_1 ]; then
        echo "  ✅ RESULT: Demurrage applied correctly (WORKING for account 1)"
    else
        echo "  ⚠️  RESULT: Unexpected balance change"
    fi
else
    echo "  ⏳ Too early: Need $((30 - ELAPSED_1)) more days"
fi

echo ""
echo ""

# Account 2: Time Capsule
echo "Account 2: Time Capsule (time_capsule_account2)"
echo "─────────────────────────────────────────────────────────────────"

ACCOUNT2_KEY="time_capsule_account2"
ACCOUNT2_PUBKEY="GBOAA6SIHR6E3LOP4I522OWC3NHGSRS5IUCYIESJ2GO35LFAMR4LUUSV"
INITIAL_BALANCE_2=1000
EXPECTED_WORKING_2=990
EXPECTED_BROKEN_2=1000

CURRENT_BALANCE_2=$(soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ACCOUNT1_KEY \
  --network $NETWORK \
  -- balance \
  --account $ACCOUNT2_KEY 2>&1 | grep -oP '\d+')

CURRENT_ACTIVITY_2=$(soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ACCOUNT1_KEY \
  --network $NETWORK \
  -- get_account \
  --account $ACCOUNT2_KEY 2>&1 | grep -oP '"last_activity":\s*\K[0-9]+')

ELAPSED_2=$(( (CURRENT_TIME - CURRENT_ACTIVITY_2) / 86400 ))

echo "  Initial balance: $INITIAL_BALANCE_2 KCHNG"
echo "  Current balance: $CURRENT_BALANCE_2 KCHNG"
echo "  Last activity: $CURRENT_ACTIVITY_2"
echo "  Days elapsed: $ELAPSED_2"
echo ""

if [ $ELAPSED_2 -ge 30 ]; then
    LOSS_2=$((INITIAL_BALANCE_2 - CURRENT_BALANCE_2))
    echo "  Balance change: $LOSS_2 KCHNG"
    echo ""

    if [ $CURRENT_BALANCE_2 -eq $INITIAL_BALANCE_2 ]; then
        echo "  🔴 RESULT: No demurrage applied (BUG CONFIRMED for account 2)"
    elif [ $CURRENT_BALANCE_2 -le $EXPECTED_WORKING_2 ]; then
        echo "  ✅ RESULT: Demurrage applied correctly (WORKING for account 2)"
    else
        echo "  ⚠️  RESULT: Unexpected balance change"
    fi
else
    echo "  ⏳ Too early: Need $((30 - ELAPSED_2)) more days"
fi

echo ""
echo ""
echo "╔════════════════════════════════════════════════════════════════════════╗"
echo "║                          FINAL VERDICT                               ║"
echo "╚════════════════════════════════════════════════════════════════════════╝"
echo ""

# Check if both accounts are old enough
if [ $ELAPSED_1 -lt 30 ] || [ $ELAPSED_2 -lt 30 ]; then
    echo "⏳ TEST IN PROGRESS"
    echo ""
    echo "Time elapsed:"
    echo "  Account 1: $ELAPSED_1 days"
    echo "  Account 2: $ELAPSED_2 days"
    echo ""
    echo "Minimum required: 30 days"
    echo "Please run again after both accounts reach 30 days."
    echo ""
    exit 0
fi

# Both accounts are old enough - make determination
TOTAL_LOSS=$((INITIAL_BALANCE_1 + INITIAL_BALANCE_2 - CURRENT_BALANCE_1 - CURRENT_BALANCE_2))
EXPECTED_TOTAL_LOSS=$((INITIAL_BALANCE_1 / 100 + INITIAL_BALANCE_2 / 100))

echo "Combined Results:"
echo "  Total initial: $((INITIAL_BALANCE_1 + INITIAL_BALANCE_2)) KCHNG"
echo "  Total current: $((CURRENT_BALANCE_1 + CURRENT_BALANCE_2)) KCHNG"
echo "  Total loss: $TOTAL_LOSS KCHNG"
echo "  Expected loss (if working): ~$EXPECTED_TOTAL_LOSS KCHNG"
echo ""

if [ $TOTAL_LOSS -eq 0 ]; then
    echo "╔════════════════════════════════════════════════════════════════════════╗"
    echo "║                  🔴🔴 BUG CONFIRMED - BOTH ACCOUNTS                     ║"
    echo "╚════════════════════════════════════════════════════════════════════════╝"
    echo ""
    echo "RESULT: Neither account lost tokens after $ELAPSED_1 and $ELAPSED_2 days"
    echo ""
    echo "This confirms the integer division bug in the deployed contract:"
    echo "  period_rate_bps = (1200 * 30) / 36500 = 0"
    echo "  burn_amount = balance * 0 / 10000 = 0"
    echo ""
    echo "Both accounts should have lost ~1% but lost nothing."
    echo ""
    echo "ACTION REQUIRED:"
    echo "  1. Fix line 709 in packages/contracts/src/lib.rs"
    echo "  2. Redeploy contract to testnet"
    echo "  3. Create new time capsule test"
    echo "  4. Verify fix works"
    echo ""
    echo "See: docs/2026-01-02_demurrage-critical-bug.md"

elif [ $TOTAL_LOSS -gt 0 ] && [ $TOTAL_LOSS -lt $((EXPECTED_TOTAL_LOSS * 2)) ]; then
    echo "╔════════════════════════════════════════════════════════════════════════╗"
    echo "║                  ✅✅ DEMURRAGE WORKING - BOTH ACCOUNTS                   ║"
    echo "╚════════════════════════════════════════════════════════════════════════╝"
    echo ""
    echo "RESULT: Both accounts lost tokens as expected!"
    echo ""
    echo "Expected loss: ~$EXPECTED_TOTAL_LOSS KCHNG"
    echo "Actual loss:   $TOTAL_LOSS KCHNG"
    echo ""
    echo "The contract is working correctly."
    echo "Demurrage is being applied (~1% per month)."

else
    echo "⚠️  UNEXPECTED RESULT"
    echo ""
    echo "Combined loss is outside expected range:"
    echo "  Expected: ~$EXPECTED_TOTAL_LOSS KCHNG"
    echo "  Actual:   $TOTAL_LOSS KCHNG"
    echo ""
    echo "This could indicate:"
    echo "  - Accounts were used (activity reset demurrage)"
    echo "  - Grace periods were activated"
    echo "  - Other transactions occurred"
    echo ""
    echo "Recommendation: Manual review needed"
fi

echo ""
echo "Verification complete: $(date -Iseconds)"
