#!/bin/bash
# KCHNG Demurrage Time Capsule Test - FIXED CONTRACT
#
# Purpose: Create test data today to verify demurrage in 30+ days
# Setup: 2026-01-02
# Verification: 2026-02-02 (or later)
#
# This will definitively prove whether the FIXED contract applies demurrage

CONTRACT_ID="CBMICVZ3FOVBLUBGOHU33FLCEO4ZSODIT7RS4GH27LECR6KHPNJP4CNB"
NETWORK="testnet"
ADMIN_KEY="kchng_admin"
ADMIN_PUBKEY="GCW4XHQLIK3VHXUGNXJ4NFLW5JDWZZ3UT2XJCYTZSO6AQJZUUDB7RVMS"

# Time capsule data file
TIME_CAPSULE_FILE="$(git rev-parse --show-toplevel)/tests/regression/time_capsule_fixed_data.json"

echo "╔══════════════════════════════════════════════════════════════════════╗"
echo "║     KCHNG DEMURRAGE TIME CAPSULE TEST - FIXED CONTRACT                 ║"
echo "║                                                                     ║"
echo "║  Setup: $(date '+%Y-%m-%d %H:%M:%S')                                         ║"
echo "║  Verify: After 2026-02-02                                            ║"
echo "║  Contract: $CONTRACT_ID ║"
echo "╚══════════════════════════════════════════════════════════════════════╝"
echo ""

# ============================================================================
# STEP 1: Create Test Account
# ============================================================================

echo "Step 1: Create time capsule test account"
echo "─────────────────────────────────────────────────────────────────"

TEST_ACCOUNT="fixed_capsule_account2"
TEST_EVIDENCE="fixed_capsule_$(date +%s | xxd -p -c 16)"

# Generate test account
echo "Creating test account: $TEST_ACCOUNT"
soroban keys generate $TEST_ACCOUNT > /dev/null 2>&1
TEST_PUBKEY=$(soroban keys public-key $TEST_ACCOUNT 2>&1)

echo "OK - Account created: ${TEST_PUBKEY:0:20}..."
echo ""

# ============================================================================
# STEP 2: Fund Account and Transfer Tokens
# ============================================================================

echo "Step 2: Fund account via Friendbot and transfer tokens"
echo "─────────────────────────────────────────────────────────────────"

echo "Funding account via Friendbot..."
curl -s -X POST "https://friendbot.stellar.org/?addr=$TEST_PUBKEY" > /dev/null
echo "OK - Funded"
echo ""

sleep 2

# Check initial balance
INITIAL_BALANCE=$(soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ADMIN_KEY \
  --network $NETWORK \
  -- balance \
  --account $TEST_PUBKEY 2>&1 | grep -oP '\d+' || echo "0")

echo "Initial balance: $INITIAL_BALANCE KCHNG"
echo ""

# Transfer tokens from admin
echo "Transferring 1000 KCHNG from admin..."
soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ADMIN_KEY \
  --network $NETWORK \
  -- transfer \
  --from $ADMIN_PUBKEY \
  --to $TEST_PUBKEY \
  --amount 1000 2>&1 | grep -q "Signing transaction"

if [ $? -eq 0 ]; then
    echo "OK - Transferred"
else
    echo "✗ Transfer failed"
    exit 1
fi

echo ""

sleep 3

# Verify transfer
FINAL_BALANCE=$(soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ADMIN_KEY \
  --network $NETWORK \
  -- balance \
  --account $TEST_PUBKEY 2>&1 | grep -oP '\d+')

echo "Balance after transfer: $FINAL_BALANCE KCHNG"
echo ""

# ============================================================================
# STEP 3: Record Initial State
# ============================================================================

echo "Step 3: Record initial state for future verification"
echo "─────────────────────────────────────────────────────────────────"

CURRENT_TIME=$(date +%s)
SETUP_DATE=$(date -Iseconds)
DEADLINE_DATE=$(date -d "+30 days" -Iseconds 2>/dev/null || date -v+30d -Iseconds)

echo "Setup timestamp: $CURRENT_TIME"
echo "Setup date: $SETUP_DATE"
echo "Verify after: $DEADLINE_DATE - 30+ days"
echo ""

# Get worker account data
ACCOUNT_DATA=$(soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ADMIN_KEY \
  --network $NETWORK \
  -- get_account \
  --account $TEST_PUBKEY 2>&1)

echo "Account data:"
echo "$ACCOUNT_DATA" | python3 -m json.tool 2>/dev/null || echo "$ACCOUNT_DATA"
echo ""

# Extract key values
WORKER_BALANCE=$(echo "$ACCOUNT_DATA" | grep -oP '"balance":\s*"\K[0-9]+')
LAST_ACTIVITY=$(echo "$ACCOUNT_DATA" | grep -oP '"last_activity":\s*\K[0-9]+')

echo "Key values to remember:"
echo "  Account: $TEST_ACCOUNT"
echo "  Public Key: $TEST_PUBKEY"
echo "  Initial balance: 1000 KCHNG"
echo "  Last activity: $LAST_ACTIVITY"
echo ""

# ============================================================================
# STEP 4: Calculate Expected Results
# ============================================================================

echo "Step 4: Calculate expected results"
echo "─────────────────────────────────────────────────────────────────"
echo ""

echo "FIXED CONTRACT EXPECTATIONS:"
echo ""
echo "IF DEMURRAGE WORKS - FIXED CONTRACT:"
echo "  Initial balance: 1000 KCHNG"
echo "  Expected balance after 30 days: about 990 KCHNG"
echo "  Expected loss: about 10 KCHNG - about 1pct"
echo "  This confirms the integer division bug is FIXED"
echo ""

echo "IF DEMURRAGE STILL BROKEN:"
echo "  Initial balance: 1000 KCHNG"
echo "  Expected balance after 30 days: 1000 KCHNG - no change"
echo "  Expected loss: 0 KCHNG"
echo "  This would mean the fix did not work"
echo ""

# ============================================================================
# STEP 5: Save Time Capsule Data
# ============================================================================

echo "Step 5: Save time capsule data"
echo "─────────────────────────────────────────────────────────────────"

mkdir -p $(git rev-parse --show-toplevel)/tests/regression

# Export variables for python script
export CONTRACT_ID NETWORK CURRENT_TIME SETUP_DATE DEADLINE_DATE
export ADMIN_KEY ADMIN_PUBKEY TEST_ACCOUNT TEST_PUBKEY TIME_CAPSULE_FILE

# Create JSON file using python to avoid shell escaping issues
python3 << 'ENDPYTHON'
import json, os

# Get environment variables
contract_id = os.environ.get('CONTRACT_ID', 'unknown')
network = os.environ.get('NETWORK', 'unknown')
current_time = int(os.environ.get('CURRENT_TIME', '0'))
setup_date = os.environ.get('SETUP_DATE', 'unknown')
deadline_date = os.environ.get('DEADLINE_DATE', 'unknown')
admin_key = os.environ.get('ADMIN_KEY', 'unknown')
admin_pubkey = os.environ.get('ADMIN_PUBKEY', 'unknown')
test_account = os.environ.get('TEST_ACCOUNT', 'unknown')
test_pubkey = os.environ.get('TEST_PUBKEY', 'unknown')
time_capsule_file = os.environ.get('TIME_CAPSULE_FILE', '/tmp/time_capsule.json')

data = {
    "test_name": "KCHNG Demurrage Time Capsule Test - Fixed Contract",
    "contract_id": contract_id,
    "network": network,
    "setup_timestamp": current_time,
    "setup_date": setup_date,
    "verify_after_date": deadline_date,
    "min_verification_days": 30,
    "test_description": "Verify FIXED contract applies demurrage correctly",
    "accounts": [
        {
            "name": "account_1_admin",
            "identity": admin_key,
            "public_key": admin_pubkey,
            "initial_balance": 500000,
            "last_activity": 1767326919,
            "description": "Admin account with existing balance",
            "note": "Admin balance is from earlier testing"
        },
        {
            "name": "account_2_time_capsule",
            "identity": test_account,
            "public_key": test_pubkey,
            "initial_balance": 1000,
            "last_activity": current_time,
            "description": "Time capsule test account with transferred tokens",
            "note": "Should show about 1pct loss after 30 days if demurrage is FIXED"
        }
    ],
    "expected_results": {
        "if_demurrage_fixed": {
            "description": "Demurrage applies correctly - about 1pct per month",
            "account_1_expected_balance": "Variable",
            "account_2_expected_balance": 990,
            "account_2_expected_loss": 10,
            "account_2_note": "Should lose about 1pct after 30 days if fix worked"
        },
        "if_demurrage_broken": {
            "description": "Integer division bug still present",
            "account_1_expected_balance": "Variable",
            "account_2_expected_balance": 1000,
            "account_2_expected_loss": 0,
            "note": "This would mean the fix in the new contract did not work"
        }
    },
    "verification_command": "bash $(git rev-parse --show-toplevel)/tests/regression/verify_time_capsule_fixed.sh",
    "comparison_contract": {
        "address": "CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX",
        "description": "Old buggy contract for side-by-side comparison",
        "verification_date": "2026-02-02T07:57:02+01:00"
    },
    "bug_report": "docs/2026-01-02_demurrage-critical-bug.md",
    "fix_report": "docs/2026-01-02_demurrage-critical-bug.md",
    "verification_status": "PENDING",
    "notes": [
        "Fixed contract should show about 1pct demurrage after 30 days",
        "This will prove the integer division bug fix works on-chain",
        "Compare with old contract results from same verification date",
        "Both contracts tested on same day for accurate comparison"
    ]
}

# Write to file
with open(time_capsule_file, 'w') as f:
    json.dump(data, f, indent=2)
ENDPYTHON

echo "OK - Time capsule data saved to: $TIME_CAPSULE_FILE"
echo ""

# ============================================================================
# STEP 6: Verification Script
# ============================================================================

echo "Step 6: Verification script"
echo "─────────────────────────────────────────────────────────────────"
echo ""
echo "Verification script already created at:"
echo "  $(git rev-parse --show-toplevel)/tests/regression/verify_time_capsule_fixed.sh"
echo ""
echo "Make it executable:"
chmod +x $(git rev-parse --show-toplevel)/tests/regression/verify_time_capsule_fixed.sh
echo ""

# ============================================================================
# SUMMARY
# ============================================================================

echo "╔══════════════════════════════════════════════════════════════════════╗"
echo "║                      TIME CAPSULE READY                                ║"
echo "╚══════════════════════════════════════════════════════════════════════╝"
echo ""
echo "Setup complete: $SETUP_DATE"
echo "Verify after: $DEADLINE_DATE"
echo ""
echo "Test Accounts:"
echo "  Account 1 - admin: $ADMIN_PUBKEY"
echo "    Initial Balance: 500000 KCHNG - existing balance"
echo "  "
echo "  Account 2 - time capsule: $TEST_PUBKEY"
echo "    Initial Balance: 1000 KCHNG"
echo ""
echo "Expected Results on $DEADLINE_DATE:"
echo "  IF BUG STILL EXISTS: Account 2 still at 1000 KCHNG"
echo "  IF FIX WORKS: Account 2 at about 990 KCHNG - about 1pct loss"
echo ""
echo "To verify results, run:"
echo "  bash $(git rev-parse --show-toplevel)/tests/regression/verify_time_capsule_fixed.sh"
echo ""
echo "Data saved to: $TIME_CAPSULE_FILE"
echo ""
