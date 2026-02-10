#!/bin/bash
# Verify Time Capsule Test Results - 7-Day Period Contract
# Run this 7+ days after setup

echo "╔══════════════════════════════════════════════════════════════════════╗"
echo "║     KCHNG DEMURRAGE TIME CAPSULE VERIFICATION - 7-DAY PERIOD         ║"
echo "╚══════════════════════════════════════════════════════════════════════╝"
echo ""

CONTRACT_ID="CAZNVOPPOMRYC5SUN2O4U4T4B4YWTGBMCP7PN2R4XPZQYWAQ6NRNBX6Z"
NETWORK="testnet"
ADMIN_KEY="kchng_admin"
TIME_CAPSULE="/home/pokho/dev/KCHNG/tests/regression/time_capsule_7day_data.json"

echo "Verification date: $(date -Iseconds)"
echo ""

# Load time capsule data
if [ ! -f "$TIME_CAPSULE" ]; then
    echo "ERROR: Time capsule data not found!"
    echo "   Expected: $TIME_CAPSULE"
    exit 1
fi

echo "Loading time capsule data..."
echo ""

# Get current time
CURRENT_TIME=$(date +%s)

# Get test account data from capsule file
TEST_PUBKEY=$(grep -oP '"public_key":\s*"\K[^"]+' "$TIME_CAPSULE")
INITIAL_BALANCE=$(grep -oP '"initial_balance": \s*\K[0-9]+' "$TIME_CAPSULE")
SETUP_TIME=$(grep -oP '"last_activity": \s*\K[0-9]+' "$TIME_CAPSULE")

echo "╔══════════════════════════════════════════════════════════════════════╗"
echo "║                      ACCOUNT VERIFICATION                             ║"
echo "╚══════════════════════════════════════════════════════════════════════╝"
echo ""

echo "Time Capsule Account"
echo "─────────────────────────────────────────────────────────────────"
echo "  Public Key: $TEST_PUBKEY"
echo "  Initial balance: $INITIAL_BALANCE KCHNG"
echo ""

# Get current balance
CURRENT_BALANCE=$(soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ADMIN_KEY \
  --network $NETWORK \
  -- balance \
  --account $TEST_PUBKEY 2>&1 | grep -oP '[0-9]+' | head -1)

# Get last activity
CURRENT_ACTIVITY=$(soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ADMIN_KEY \
  --network $NETWORK \
  -- get_account \
  --account $TEST_PUBKEY 2>&1 | grep -oP '"last_activity":\s*\K[0-9]+')

# Calculate elapsed time
ELAPSED_SECONDS=$((CURRENT_TIME - CURRENT_ACTIVITY))
ELAPSED_DAYS=$((ELAPSED_SECONDS / 86400))

echo "  Current balance: $CURRENT_BALANCE KCHNG"
echo "  Last activity: $CURRENT_ACTIVITY ($(date -d @$CURRENT_ACTIVITY -Iseconds))"
echo "  Days elapsed: $ELAPSED_DAYS"
echo ""

# Calculate expected values
EXPECTED_BURN=230
EXPECTED_BALANCE=9770

echo "╔══════════════════════════════════════════════════════════════════════╗"
echo "║                          RESULTS                                    ║"
echo "╚══════════════════════════════════════════════════════════════════════╝"
echo ""

# Calculate actual loss
LOSS=$((INITIAL_BALANCE - CURRENT_BALANCE))

echo "Expected (if working):"
echo "  Balance after $ELAPSED_DAYS days: ~$EXPECTED_BALANCE KCHNG"
echo "  KCHNG burned: ~$EXPECTED_BURN (2.3%)"
echo ""

echo "Actual Results:"
echo "  Balance: $CURRENT_BALANCE KCHNG"
echo "  KCHNG change: $LOSS"
echo ""

if [ $ELAPSED_DAYS -lt 7 ]; then
    echo "⏳ TOO EARLY: Need $((7 - ELAPSED_DAYS)) more days"
    echo ""
    echo "Test will be conclusive after 7+ days of inactivity"
else
    echo "✅ SUFFICIENT TIME PASSED: $ELAPSED_DAYS days"
    echo ""

    # Check if demurrage was applied
    if [ $CURRENT_BALANCE -eq $INITIAL_BALANCE ]; then
        echo "🔴 RESULT: NO DEMURRAGE APPLIED (BUG STILL PRESENT!)"
        echo ""
        echo "The contract still has the integer division bug."
        echo "Balance unchanged after $ELAPSED_DAYS days."
        echo ""
        echo "CRITICAL: The fix did not work - investigate further!"
    elif [ $CURRENT_BALANCE -ge $((EXPECTED_BALANCE - 50)) ] && [ $CURRENT_BALANCE -le $((EXPECTED_BALANCE + 50)) ]; then
        echo "✅ RESULT: DEMURRAGE APPLIED CORRECTLY (FIX WORKS!)"
        echo ""
        echo "Balance decreased from $INITIAL_BALANCE to $CURRENT_BALANCE"
        echo "KCHNG burned: $LOSS"
        echo ""
        echo "The integer division bug fix is WORKING on-chain!"
        echo ""
        echo "Demurrage is functioning as designed."
    else
        echo "⚠️  RESULT: UNEXPECTED BALANCE"
        echo ""
        echo "Balance: $CURRENT_BALANCE (expected ~$EXPECTED_BALANCE)"
        echo "Loss: $LOSS KCHNG"
        echo ""
        echo "This could indicate:"
        echo "  - Partial demurrage calculation"
        echo "  - Different rate than expected"
        echo "  - Account activity during test period"
    fi
fi

echo ""
echo "╔══════════════════════════════════════════════════════════════════════╗"
echo "║                          COMPARISON                                  ║"
echo "╚══════════════════════════════════════════════════════════════════════╝"
echo ""
echo "Old Contracts (Buggy):"
echo "  CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX"
echo "  CBMICVZ3FOVBLUBGOHU33FLCEO4ZSODIT7RS4GH27LECR6KHPNJP4CNB"
echo "  Result: No demurrage after 30+ days"
echo ""
echo "New Contract (7-Day Period):"
echo "  $CONTRACT_ID"
echo "  Result: Testing..."
echo ""

echo "Verification complete: $(date -Iseconds)"
