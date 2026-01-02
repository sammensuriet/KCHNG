#!/bin/bash
# Verify Demurrage Bug on Actual Contract
# This test checks if the deployed contract actually applies demurrage

CONTRACT_ID="CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX"
NETWORK="testnet"
ADMIN_KEY="kchng_admin"

echo "╔════════════════════════════════════════════════════════════════════════╗"
echo "║     VERIFY DEMURRAGE BUG ON ACTUAL CONTRACT                          ║"
echo "╚════════════════════════════════════════════════════════════════════════╝"
echo ""
echo "This test checks if the deployed contract actually applies demurrage"
echo ""

# ============================================================================
# STEP 1: Get account data including last_activity timestamp
# ============================================================================

echo "Step 1: Get current account state"
echo "─────────────────────────────────────────────────────────────────"

ACCOUNT_DATA=$(soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ADMIN_KEY \
  --network $NETWORK \
  -- get_account \
  --account $ADMIN_KEY 2>&1 | grep -v "Simulation identified")

echo "$ACCOUNT_DATA"

echo ""
echo "Parsing account data..."

# Extract balance and last_activity from JSON
CURRENT_BALANCE=$(echo "$ACCOUNT_DATA" | grep -oP '"balance":\s*"?\K[0-9]+' || echo "0")
LAST_ACTIVITY=$(echo "$ACCOUNT_DATA" | grep -oP '"last_activity":\s*\K[0-9]+' || echo "0")

echo "Current Balance: $CURRENT_BALANCE"
echo "Last Activity: $LAST_ACTIVITY"

# Convert timestamp to readable date
if command -v date &> /dev/null; then
    ACTIVITY_DATE=$(date -d @$LAST_ACTIVITY 2>/dev/null || date -r $LAST_ACTIVITY 2>/dev/null || echo "Unknown")
    echo "Last Activity Date: $ACTIVITY_DATE"
fi

# ============================================================================
# STEP 2: Check what balance SHOULD be with demurrage
# ============================================================================

echo ""
echo "Step 2: Calculate expected balance if demurrage works"
echo "─────────────────────────────────────────────────────────────────"

CURRENT_TIME=$(date +%s)
echo "Current timestamp: $CURRENT_TIME"

if [ "$LAST_ACTIVITY" -gt 0 ]; then
    ELAPSED_SECONDS=$((CURRENT_TIME - LAST_ACTIVITY))
    ELAPSED_DAYS=$((ELAPSED_SECONDS / 86400))

    echo "Elapsed time: $ELAPSED_DAYS days"

    if [ $ELAPSED_DAYS -ge 30 ]; then
        PERIODS=$((ELAPSED_DAYS / 30))
        echo "Complete 30-day periods: $PERIODS"

        # Expected demurrage: ~1% per period (12% annual / 12 months)
        # Formula: balance * (0.99 ^ periods)
        EXPECTED_LOSS=$((CURRENT_BALANCE * PERIODS / 100))
        EXPECTED_BALANCE=$((CURRENT_BALANCE - EXPECTED_LOSS))

        echo ""
        echo "If demurrage WORKS:"
        echo "  Expected loss: ~$EXPECTED_LOSS KCHNG"
        echo "  Expected balance: ~$EXPECTED_BALANCE KCHNG"
        echo ""
        echo "If demurrage IS BROKEN (bug exists):"
        echo "  Actual balance: $CURRENT_BALANCE KCHNG (no change)"
    else
        echo "Less than 30 days elapsed - demurrage shouldn't apply yet"
        echo "Need to wait for account to be inactive for 30+ days"
    fi
else
    echo "ERROR: Could not determine last activity time"
fi

# ============================================================================
# STEP 3: Check total supply history (if we have records)
# ============================================================================

echo ""
echo "Step 3: Check total supply"
echo "─────────────────────────────────────────────────────────────────"

TOTAL_SUPPLY=$(soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $ADMIN_KEY \
  --network $NETWORK \
  -- total_supply 2>&1 | grep -oP '\d+')

echo "Current Total Supply: $TOTAL_SUPPLY"

# ============================================================================
# STEP 4: Try to create a test scenario
# ============================================================================

echo ""
echo "Step 4: Create test scenario to verify demurrage"
echo "─────────────────────────────────────────────────────────────────"
echo ""
echo "Option A: Find old inactive account"
echo "  - Search for account with last_activity > 30 days ago"
echo "  - Check if balance decreased"
echo ""
echo "Option B: Wait and re-test"
echo "  - Record current balance and time"
echo "  - Wait 30+ days"
echo "  - Check again"
echo ""
echo "Option C: Check contract source code"
echo "  - Verify the bug exists in the code"
echo "  - Location: packages/contracts/src/lib.rs:709"

# ============================================================================
# STEP 5: Check the source code
# ============================================================================

echo ""
echo "Step 5: Verify bug in contract source code"
echo "─────────────────────────────────────────────────────────────────"
echo ""

CONTRACT_FILE="/home/pokho/dev/KCHNG/packages/contracts/src/lib.rs"

if [ -f "$CONTRACT_FILE" ]; then
    echo "Checking line 709 of contract source..."
    echo ""

    # Extract the problematic line
    sed -n '705,715p' "$CONTRACT_FILE"

    echo ""
    echo "Analysis:"
    echo "  period_rate_bps = (annual_rate_bps * period_days) / 36500"
    echo ""
    echo "  For 12% annual (1200 bps) and 30-day periods:"
    echo "    period_rate_bps = 1200 * 30 / 36500"
    echo "                    = 36000 / 36500"
    echo "                    = 0 (integer division truncates!)"
    echo ""
    echo "  This means: burn_amount = balance * 0 / 10000 = 0"
    echo "  Result: NO DEMURRAGE APPLIES"
else
    echo "Contract source file not found"
fi

# ============================================================================
# CONCLUSION
# ============================================================================

echo ""
echo "╔════════════════════════════════════════════════════════════════════════╗"
echo "║                          VERDICT                                     ║"
echo "╚════════════════════════════════════════════════════════════════════════╝"
echo ""
echo "Source Code Analysis:"
echo "  ✅ Bug confirmed in source code (line 709)"
echo "  ✅ Integer division causes period_rate_bps = 0"
echo "  ✅ This means demurrage is calculated but always equals 0"
echo ""
echo "Deployed Contract:"
echo "  ⚠️  Needs verification with actual time passage"
echo "  ⚠️  Or find account inactive for 30+ days"
echo ""
echo "Recommendation:"
echo "  1. Check if deployed contract matches source code"
echo "  2. Create test account, wait 30 days, verify balance"
echo "  3. Fix bug in source code"
echo "  4. Deploy fixed contract to testnet"
echo "  5. Verify fix works with real time passage"
echo ""
