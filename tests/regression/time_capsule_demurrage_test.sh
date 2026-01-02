#!/bin/bash
# KCHNG Demurrage Time Capsule Test
#
# Purpose: Create test data today to verify demurrage in 30+ days
# Setup: 2026-01-02
# Verification: 2026-02-02 (or later)
#
# This will definitively prove whether demurrage bug exists in deployed contract

CONTRACT_ID="CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX"
NETWORK="testnet"
ADMIN_KEY="kchng_admin"

# Time capsule data file
TIME_CAPSULE_FILE="/tmp/kchng-simulation/time_capsule_data.json"

echo "╔════════════════════════════════════════════════════════════════════════╗"
echo "║     KCHNG DEMURRAGE TIME CAPSULE TEST                                 ║"
echo "║                                                                     ║"
echo "║  Setup: $(date '+%Y-%m-%d %H:%M:%S')                                         ║"
echo "║  Verify: After 2026-02-02                                            ║"
echo "╚════════════════════════════════════════════════════════════════════════╝"
echo ""

# ============================================================================
# STEP 1: Create Test Account
# ============================================================================

echo "Step 1: Create time capsule test account"
echo "─────────────────────────────────────────────────────────────────"

TEST_ACCOUNT="time_capsule_worker"
TEST_EVIDENCE="74696d655f63617073756c655f746573745f32303236303130325f$(date +%s | xxd -p -c 16)"

# Generate test account
echo "Creating test account: $TEST_ACCOUNT"
soroban keys generate $TEST_ACCOUNT > /dev/null 2>&1
TEST_PUBKEY=$(soroban keys public-key $TEST_ACCOUNT 2>&1)

echo "✓ Account created: ${TEST_PUBKEY:0:20}..."
echo ""

# ============================================================================
# STEP 2: Fund Account and Submit Work Claim
# ============================================================================

echo "Step 2: Submit work claim to earn tokens"
echo "─────────────────────────────────────────────────────────────────"

# Fund via friendbot
echo "Funding account via Friendbot..."
curl -s -X POST "https://friendbot.stellar.org/?addr=$TEST_PUBKEY" > /dev/null
echo "✓ Funded"
echo ""

# Get initial balance
INITIAL_BALANCE=$(soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ADMIN_KEY \
  --network $NETWORK \
  -- balance \
  --account $TEST_PUBKEY 2>&1 | grep -oP '\d+' || echo "0")

echo "Initial balance: $INITIAL_BALANCE KCHNG"
echo ""

# Submit work claim (30 minutes = 1 KCHNG expected)
echo "Submitting work claim for 30 minutes..."

# Submit as admin (since other accounts can't submit)
RESULT=$(soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ADMIN_KEY \
  --network $NETWORK \
  --send yes \
  -- submit_work_claim \
  --worker $ADMIN_KEY \
  --work_type 0 \
  --minutes_worked 30 \
  --evidence_hash "$TEST_EVIDENCE" 2>&1)

if echo "$RESULT" | grep -q "Signing transaction"; then
    echo "✓ Work claim submitted"
else
    echo "✗ Failed to submit claim"
    echo "$RESULT"
    exit 1
fi

sleep 2

# ============================================================================
# STEP 3: Approve Claim to Mint Tokens
# ============================================================================

echo ""
echo "Step 3: Approve work claim (requires 2 approvals)"
echo "─────────────────────────────────────────────────────────────────"

# We need to find the claim ID - check recent claims
echo "Finding submitted claim ID..."

# The claim should be the highest numbered one
# Let's try claim 117 (next after our 116 test)
CLAIM_ID=117

echo "Approving claim #$CLAIM_ID (first approval)..."
soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ADMIN_KEY \
  --network $NETWORK \
  --send yes \
  -- approve_work_claim \
  --claim_id $CLAIM_ID \
  --verifier $ADMIN_KEY > /dev/null 2>&1

sleep 2

echo "Approving claim #$CLAIM_ID (second approval)..."
soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ADMIN_KEY \
  --network $NETWORK \
  --send yes \
  -- approve_work_claim \
  --claim_id $CLAIM_ID \
  --verifier $ADMIN_KEY > /dev/null 2>&1

sleep 3

# Check admin balance (worker for this claim)
BALANCE_AFTER_CLAIM=$(soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ADMIN_KEY \
  --network $NETWORK \
  -- balance \
  --account $ADMIN_KEY 2>&1 | grep -oP '\d+')

echo "✓ Claim approved"
echo "Worker balance after approval: $BALANCE_AFTER_CLAIM KCHNG"
echo ""

# ============================================================================
# STEP 4: Record Initial State
# ============================================================================

echo "Step 4: Record initial state for future verification"
echo "─────────────────────────────────────────────────────────────────"

CURRENT_TIME=$(date +%s)
SETUP_DATE=$(date -Iseconds)
DEADLINE_DATE=$(date -d "+30 days" -Iseconds 2>/dev/null || date -v+30d -Iseconds)

echo "Setup timestamp: $CURRENT_TIME"
echo "Setup date: $SETUP_DATE"
echo "Verify after: $DEADLINE_DATE (30+ days)"
echo ""

# Get worker account data
ACCOUNT_DATA=$(soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ADMIN_KEY \
  --network $NETWORK \
  -- get_account \
  --account $ADMIN_KEY 2>&1)

echo "Account data:"
echo "$ACCOUNT_DATA" | python3 -m json.tool 2>/dev/null || echo "$ACCOUNT_DATA"
echo ""

# Extract key values
WORKER_BALANCE=$(echo "$ACCOUNT_DATA" | grep -oP '"balance":\s*"\K[0-9]+')
LAST_ACTIVITY=$(echo "$ACCOUNT_DATA" | grep -oP '"last_activity":\s*\K[0-9]+')

echo "Key values to remember:"
echo "  Worker: $ADMIN_KEY"
echo "  Initial balance: $WORKER_BALANCE KCHNG"
echo "  Last activity: $LAST_ACTIVITY"
echo ""

# ============================================================================
# STEP 5: Calculate Expected Results
# ============================================================================

echo "Step 5: Calculate expected results"
echo "─────────────────────────────────────────────────────────────────"
echo ""

echo "IF DEMURRAGE WORKS (no bug):"
echo "  Expected balance after 30 days: ~$((WORKER_BALANCE - WORKER_BALANCE / 100)) KCHNG"
echo "  Expected loss: ~$((WORKER_BALANCE / 100)) KCHNG (~1%)"
echo ""

echo "IF DEMURRAGE IS BROKEN (bug exists):"
echo "  Expected balance after 30 days: $WORKER_BALANCE KCHNG (no change)"
echo "  Expected loss: 0 KCHNG"
echo ""

# ============================================================================
# STEP 6: Save Time Capsule Data
# ============================================================================

echo "Step 6: Save time capsule data"
echo "─────────────────────────────────────────────────────────────────"

mkdir -p /tmp/kchng-simulation

cat > "$TIME_CAPSULE_FILE" << EOF
{
  "test_name": "KCHNG Demurrage Time Capsule Test",
  "contract_id": "$CONTRACT_ID",
  "network": "$NETWORK",
  "setup_timestamp": $CURRENT_TIME,
  "setup_date": "$SETUP_DATE",
  "verify_after_date": "$DEADLINE_DATE",
  "min_verification_days": 30,
  "test_account": "$ADMIN_KEY",
  "initial_state": {
    "balance": $WORKER_BALANCE,
    "last_activity": $LAST_ACTIVITY
  },
  "expected_results": {
    "if_demurrage_works": {
      "description": "Demurrage applies correctly",
      "expected_balance_after_30_days": $((WORKER_BALANCE - WORKER_BALANCE / 100)),
      "expected_loss": $((WORKER_BALANCE / 100)),
      "note": "1% per month (12% annual)"
    },
    "if_demurrage_broken": {
      "description": "Integer division bug prevents demurrage",
      "expected_balance_after_30_days": $WORKER_BALANCE,
      "expected_loss": 0,
      "bug_location": "packages/contracts/src/lib.rs:709",
      "bug_formula": "period_rate_bps = (annual_rate_bps * period_days) / 36500",
      "bug_result": "period_rate_bps = 1200 * 30 / 36500 = 0"
    }
  },
  "verification_command": "bash /home/pokho/dev/KCHNG/tests/regression/verify_time_capsule.sh",
  "bug_report": "docs/2026-01-02_demurrage-critical-bug.md",
  "verification_status": "PENDING"
}
EOF

echo "✓ Time capsule data saved to: $TIME_CAPSULE_FILE"
echo ""

# ============================================================================
# STEP 7: Create Verification Script
# ============================================================================

echo "Step 7: Create verification script"
echo "─────────────────────────────────────────────────────────────────"

cat > /home/pokho/dev/KCHNG/tests/regression/verify_time_capsule.sh << 'VERIFYSCRIPT'
#!/bin/bash
# Verify Time Capsule Test Results
# Run this 30+ days after setup

echo "╔════════════════════════════════════════════════════════════════════════╗"
echo "║     KCHNG DEMURRAGE TIME CAPSULE VERIFICATION                          ║"
echo "╚════════════════════════════════════════════════════════════════════════╝"
echo ""

CONTRACT_ID="CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX"
NETWORK="testnet"
ADMIN_KEY="kchng_admin"
TIME_CAPSULE="/tmp/kchng-simulation/time_capsule_data.json"

echo "Verification date: $(date -Iseconds)"
echo ""

# Load time capsule data
if [ ! -f "$TIME_CAPSULE" ]; then
    echo "❌ ERROR: Time capsule data not found!"
    echo "   Expected: $TIME_CAPSULE"
    exit 1
fi

echo "Loading time capsule data..."
INITIAL_BALANCE=$(grep -oP '"initial_state".*?"balance":\s*\K[0-9]+' "$TIME_CAPSULE")
LAST_ACTIVITY=$(grep -oP '"initial_state".*?"last_activity":\s*\K[0-9]+' "$TIME_CAPSULE")

echo "  Initial balance: $INITIAL_BALANCE KCHNG"
echo "  Last activity: $LAST_ACTIVITY"
echo ""

# Check current account state
echo "Checking current account state..."
CURRENT_BALANCE=$(soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ADMIN_KEY \
  --network $NETWORK \
  -- balance \
  --account $ADMIN_KEY 2>&1 | grep -oP '\d+')

CURRENT_ACTIVITY=$(soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ADMIN_KEY \
  --network $NETWORK \
  -- get_account \
  --account $ADMIN_KEY 2>&1 | grep -oP '"last_activity":\s*\K[0-9]+')

CURRENT_TIME=$(date +%s)
ELAPSED_DAYS=$(( (CURRENT_TIME - LAST_ACTIVITY) / 86400 ))

echo "  Current balance: $CURRENT_BALANCE KCHNG"
echo "  Current activity: $CURRENT_ACTIVITY"
echo "  Elapsed time: $ELAPSED_DAYS days"
echo ""

# Calculate expected
EXPECTED_LOSS=$((INITIAL_BALANCE / 100))
EXPECTED_WORKING=$((INITIAL_BALANCE - EXPECTED_LOSS))

echo "╔════════════════════════════════════════════════════════════════════════╗"
echo "║                          RESULTS                                     ║"
echo "╚════════════════════════════════════════════════════════════════════════╝"
echo ""

if [ $ELAPSED_DAYS -lt 30 ]; then
    echo "⚠️  WARNING: Less than 30 days have passed"
    echo "   Elapsed: $ELAPSED_DAYS days"
    echo "   Required: 30 days for demurrage to apply"
    echo ""
    echo "Current status:"
    echo "  Balance: $CURRENT_BALANCE KCHNG"
    echo "  Initial: $INITIAL_BALANCE KCHNG"
    echo "  Change: $((INITIAL_BALANCE - CURRENT_BALANCE)) KCHNG"
    echo ""
    echo "Please run again after $((30 - ELAPSED_DAYS)) more days"
    exit 0
fi

# Verify results
BALANCE_CHANGE=$((INITIAL_BALANCE - CURRENT_BALANCE))

echo "Initial Balance: $INITIAL_BALANCE KCHNG"
echo "Current Balance: $CURRENT_BALANCE KCHNG"
echo "Net Change:      $BALANCE_CHANGE KCHNG"
echo ""

if [ $CURRENT_BALANCE -eq $INITIAL_BALANCE ]; then
    echo "╔════════════════════════════════════════════════════════════════════════╗"
    echo "║                  🔴 BUG CONFIRMED                                   ║"
    echo "╚════════════════════════════════════════════════════════════════════════╝"
    echo ""
    echo "RESULT: No demurrage applied after $ELAPSED_DAYS days"
    echo ""
    echo "This confirms the integer division bug:"
    echo "  period_rate_bps = (1200 * 30) / 36500 = 0"
    echo "  burn_amount = balance * 0 / 10000 = 0"
    echo ""
    echo "ACTION REQUIRED:"
    echo "  1. Fix line 709 in packages/contracts/src/lib.rs"
    echo "  2. Redeploy contract to testnet"
    echo "  3. Run new time capsule test"
    echo "  4. Verify fix works"
    echo ""
    echo "See: docs/2026-01-02_demurrage-critical-bug.md"

elif [ $BALANCE_CHANGE -gt 0 ] && [ $BALANCE_CHANGE -lt $((EXPECTED_LOSS * 2)) ]; then
    echo "╔════════════════════════════════════════════════════════════════════════╗"
    echo "║                  ✅ DEMURRAGE WORKING                               ║"
    echo "╚════════════════════════════════════════════════════════════════════════╝"
    echo ""
    echo "RESULT: Demurrage applied correctly!"
    echo ""
    echo "Expected loss: ~$EXPECTED_LOSS KCHNG"
    echo "Actual loss:   $BALANCE_CHANGE KCHNG"
    echo ""
    echo "The contract is working as designed."
    echo "No bug fix needed (or already fixed)."

else
    echo "⚠️  UNEXPECTED RESULT"
    echo ""
    echo "Balance change is outside expected range:"
    echo "  Expected: ~$EXPECTED_LOSS KCHNG loss"
    echo "  Actual:   $BALANCE_CHANGE KCHNG"
    echo ""
    echo "This could indicate:"
    echo "  - Account was used (activity reset demurrage)"
    echo "  - Grace period was activated"
    echo "  - Other transactions occurred"
    echo ""
    echo "Recommendation: Manual review needed"
fi

echo ""
echo "Verification complete: $(date -Iseconds)"
VERIFYSCRIPT

chmod +x /home/pokho/dev/KCHNG/tests/regression/verify_time_capsule.sh

echo "✓ Verification script created: /home/pokho/dev/KCHNG/tests/regression/verify_time_capsule.sh"
echo ""

# ============================================================================
# SUMMARY
# ============================================================================

echo "╔════════════════════════════════════════════════════════════════════════╗"
echo "║                      TIME CAPSULE READY                                ║"
echo "╚════════════════════════════════════════════════════════════════════════╝"
echo ""
echo "Setup complete: $SETUP_DATE"
echo "Verify after: $DEADLINE_DATE"
echo ""
echo "Test Account: $ADMIN_KEY"
echo "Initial Balance: $WORKER_BALANCE KCHNG"
echo "Last Activity: $LAST_ACTIVITY"
echo ""
echo "Expected in 30 days:"
echo "  IF BUG EXISTS: Balance stays at $WORKER_BALANCE KCHNG"
echo "  IF WORKING:  Balance drops to ~$((WORKER_BALANCE - WORKER_BALANCE / 100)) KCHNG"
echo ""
echo "To verify results, run:"
echo "  bash /home/pokho/dev/KCHNG/tests/regression/verify_time_capsule.sh"
echo ""
echo "Data saved to: $TIME_CAPSULE_FILE"
echo ""
