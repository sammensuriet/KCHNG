#!/bin/bash
# Verify Time Capsule Test Results - FIXED CONTRACT
# Run this 30+ days after setup

echo "╔══════════════════════════════════════════════════════════════════════╗"
echo "║     KCHNG DEMURRAGE TIME CAPSULE VERIFICATION - FIXED CONTRACT          ║"
echo "╚══════════════════════════════════════════════════════════════════════╝"
echo ""

CONTRACT_ID="CBMICVZ3FOVBLUBGOHU33FLCEO4ZSODIT7RS4GH27LECR6KHPNJP4CNB"
NETWORK="testnet"
ADMIN_KEY="kchng_admin"
ADMIN_PUBKEY="GCW4XHQLIK3VHXUGNXJ4NFLW5JDWZZ3UT2XJCYTZSO6AQJZUUDB7RVMS"
TIME_CAPSULE="$(git rev-parse --show-toplevel)/tests/regression/time_capsule_fixed_data.json"

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

# Test each account
echo "╔══════════════════════════════════════════════════════════════════════╗"
echo "║                      ACCOUNT VERIFICATION                             ║"
echo "╚══════════════════════════════════════════════════════════════════════╝"
echo ""

# Account 1: Admin
echo "Account 1: Admin"
echo "─────────────────────────────────────────────────────────────────"

ACCOUNT1_KEY="kchng_admin"
ACCOUNT1_PUBKEY="GCW4XHQLIK3VHXUGNXJ4NFLW5JDWZZ3UT2XJCYTZSO6AQJZUUDB7RVMS"
INITIAL_BALANCE_1=500000
CURRENT_BALANCE_1=$(soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ACCOUNT1_KEY \
  --network $NETWORK \
  -- balance \
  --account $ACCOUNT1_PUBKEY 2>&1 | grep -oP '[0-9]+' | head -1)

CURRENT_ACTIVITY_1=$(soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ACCOUNT1_KEY \
  --network $NETWORK \
  -- get_account \
  --account $ACCOUNT1_PUBKEY 2>&1 | grep -oP '"last_activity":\s*\K[0-9]+')

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
    echo "  Admin account balance may have changed for various reasons"
else
    echo "  Too early: Need $((30 - ELAPSED_1)) more days"
fi

echo ""
echo ""

# Account 2: Time Capsule
echo "Account 2: Time Capsule"
echo "─────────────────────────────────────────────────────────────────"

TEST_ACCOUNT="fixed_capsule_account2"
TEST_PUBKEY_PLACEHOLDER="PLACEHOLDER"
INITIAL_BALANCE_2=1000

# Load the actual public key from time capsule data
TEST_PUBKEY=$(grep -oP '"public_key":\s*"\K[^"]+' "$TIME_CAPSULE" | tail -1)

CURRENT_BALANCE_2=$(soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ADMIN_KEY \
  --network $NETWORK \
  -- balance \
  --account $TEST_PUBKEY 2>&1 | grep -oP '[0-9]+' | head -1)

CURRENT_ACTIVITY_2=$(soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ADMIN_KEY \
  --network $NETWORK \
  -- get_account \
  --account $TEST_PUBKEY 2>&1 | grep -oP '"last_activity":\s*\K[0-9]+')

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
        echo "  RESULT: No demurrage applied (FIX FAILED!)"
        echo ""
        echo "  This would mean the integer division bug is STILL PRESENT"
        echo "  even in the fixed contract!"
        echo ""
        echo "  CRITICAL: Investigation needed!"
    elif [ $CURRENT_BALANCE_2 -lt 980 ] && [ $CURRENT_BALANCE_2 -ge 900 ]; then
        echo "  RESULT: Demurrage applied correctly (FIX WORKS!)"
        echo ""
        echo "  Balance decreased from $INITIAL_BALANCE_2 to $CURRENT_BALANCE_2"
        echo "  This proves the fixed contract applies demurrage correctly!"
        echo ""
        echo "  The integer division bug fix is WORKING on-chain!"
    else
        echo "  RESULT: Unexpected balance"
        echo ""
        echo "  Balance: $CURRENT_BALANCE_2 (expected about 990 if working)"
        echo "  May indicate partial demurrage or other activity"
    fi
else
    echo "  Too early: Need $((30 - ELAPSED_2)) more days"
fi

echo ""
echo ""
echo "╔══════════════════════════════════════════════════════════════════════╗"
echo "║                          COMPARISON                                  ║"
echo "╚══════════════════════════════════════════════════════════════════════╝"
echo ""
echo "Old Contract (Buggy):"
echo "  Address: CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX"
echo "  Verification: Run bash $(git rev-parse --show-toplevel)/tests/regression/verify_time_capsule.sh"
echo ""
echo "New Contract (Fixed):"
echo "  Address: $CONTRACT_ID"
echo "  This contract"
echo ""
echo "Expected Results:"
echo "  Old Contract: No demurrage (balance unchanged)"
echo "  New Contract: about 1pct demurrage (balance decreased)"
echo ""

echo "Verification complete: $(date -Iseconds)"
