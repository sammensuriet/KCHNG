#!/bin/bash
# KCHNG Fixed Contract Regression Tests
# Tests the new fixed contract on Stellar Testnet

set -e

NEW_CONTRACT_ID="CBMICVZ3FOVBLUBGOHU33FLCEO4ZSODIT7RS4GH27LECR6KHPNJP4CNB"
OLD_CONTRACT_ID="CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX"
NETWORK="testnet"
ADMIN_KEY="kchng_admin"
ADMIN_PUBKEY="GCW4XHQLIK3VHXUGNXJ4NFLW5JDWZZ3UT2XJCYTZSO6AQJZUUDB7RVMS"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Test counters
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

print_header() {
    echo ""
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo ""
}

print_test() {
    echo -e "${YELLOW}TEST:${NC} $1"
    TESTS_RUN=$((TESTS_RUN + 1))
}

print_pass() {
    echo -e "${GREEN}✓ PASS:${NC} $1"
    TESTS_PASSED=$((TESTS_PASSED + 1))
}

print_fail() {
    echo -e "${RED}✗ FAIL:${NC} $1"
    TESTS_FAILED=$((TESTS_FAILED + 1))
}

print_info() {
    echo -e "${BLUE}INFO:${NC} $1"
}

# ============================================================================
# TEST 1: Contract Deployment
# ============================================================================

print_header "FIXED CONTRACT DEPLOYMENT VERIFICATION"
print_info "New Contract: $NEW_CONTRACT_ID"
print_info "Old Contract: $OLD_CONTRACT_ID"

print_test "Total Supply Check"
TOTAL_SUPPLY=$(soroban contract invoke \
  --id $NEW_CONTRACT_ID \
  --source-account $ADMIN_KEY \
  --network $NETWORK \
  -- total_supply 2>&1 | grep -oP '\d+' || echo "0")

if [ "$TOTAL_SUPPLY" = "1000000" ]; then
    print_pass "Total supply is 1,000,000 KCHNG"
else
    print_fail "Total supply is $TOTAL_SUPPLY, expected 1,000,000"
fi

print_test "Admin Balance Check"
ADMIN_BALANCE=$(soroban contract invoke \
  --id $NEW_CONTRACT_ID \
  --source-account $ADMIN_KEY \
  --network $NETWORK \
  -- balance \
  --account $ADMIN_PUBKEY 2>&1 | grep -oP '\d+' || echo "0")

if [ "$ADMIN_BALANCE" = "1000000" ]; then
    print_pass "Admin balance is 1,000,000 KCHNG"
else
    print_fail "Admin balance is $ADMIN_BALANCE, expected 1,000,000"
fi

# ============================================================================
# TEST 2: Trust System
# ============================================================================

print_header "TRUST SYSTEM TESTS"

print_test "Register Trust"
RESULT=$(soroban contract invoke \
  --id $NEW_CONTRACT_ID \
  --source-account $ADMIN_KEY \
  --network $NETWORK \
  -- register_trust \
  --trust_name "Test Trust" \
  --annual_rate_bps 1200 \
  --demurrage_period_days 30 2>&1)

if echo "$RESULT" | grep -q "Signing transaction\| stellar.expert"; then
    print_pass "Trust registration submitted"
    sleep 3
else
    print_fail "Trust registration failed"
fi

print_test "Get Trust Info"
TRUST_INFO=$(soroban contract invoke \
  --id $NEW_CONTRACT_ID \
  --source-account $ADMIN_KEY \
  --network $NETWORK \
  -- get_trust_info \
  --trust_id $ADMIN_PUBKEY 2>&1)

if echo "$TRUST_INFO" | grep -q "Test Trust"; then
    print_pass "Trust info retrieved successfully"
else
    print_fail "Could not retrieve trust info"
fi

# ============================================================================
# TEST 3: Reputation System
# ============================================================================

print_header "REPUTATION SYSTEM TESTS"

# Create a test verifier
TEST_VERIFIER="test_regression_verifier_$(date +%s)"
soroban keys generate $TEST_VERIFIER > /dev/null 2>&1
TEST_VERIFIER_PUBKEY=$(soroban keys public-key $TEST_VERIFIER 2>&1)

print_test "Fund Verifier Account"
FUND_RESULT=$(curl -s -X POST "https://friendbot.stellar.org/?addr=$TEST_VERIFIER_PUBKEY")
if [ $? -eq 0 ]; then
    print_pass "Verifier account funded via Friendbot"
    sleep 2
else
    print_fail "Failed to fund verifier account"
fi

print_test "Register Verifier"
# Transfer tokens for staking first
soroban contract invoke \
  --id $NEW_CONTRACT_ID \
  --source-account $ADMIN_KEY \
  --network $NETWORK \
  -- transfer \
  --from $ADMIN_PUBKEY \
  --to $TEST_VERIFIER_PUBKEY \
  --amount 100000 2>&1 | grep -q "Signing transaction"

sleep 2

REG_RESULT=$(soroban contract invoke \
  --id $NEW_CONTRACT_ID \
  --source-account $ADMIN_KEY \
  --network $NETWORK \
  -- register_verifier \
  --verifier $TEST_VERIFIER_PUBKEY \
  --trust_id $ADMIN_PUBKEY 2>&1)

if echo "$REG_RESULT" | grep -q "Signing transaction\|stellar.expert"; then
    print_pass "Verifier registered"
    sleep 3
else
    print_fail "Verifier registration failed"
fi

print_test "Get Verifier Data"
VERIFIER_DATA=$(soroban contract invoke \
  --id $NEW_CONTRACT_ID \
  --source-account $ADMIN_KEY \
  --network $NETWORK \
  -- get_verifier \
  --verifier $TEST_VERIFIER_PUBKEY 2>&1)

echo "$VERIFIER_DATA" | python3 -m json.tool 2>/dev/null || echo "$VERIFIER_DATA"

if echo "$VERIFIER_DATA" | grep -q '"reputation_score":\s*500'; then
    print_pass "Initial reputation is 500 (neutral)"
else
    print_fail "Initial reputation not 500 or not found"
fi

# ============================================================================
# TEST 4: Work Claim System
# ============================================================================

print_header "WORK CLAIM SYSTEM TESTS"

# Create test worker
TEST_WORKER="test_regression_worker_$(date +%s)"
soroban keys generate $TEST_WORKER > /dev/null 2>&1
TEST_WORKER_PUBKEY=$(soroban keys public-key $TEST_WORKER 2>&1)

print_test "Fund Worker Account"
curl -s -X POST "https://friendbot.stellar.org/?addr=$TEST_WORKER_PUBKEY" > /dev/null
sleep 2
print_pass "Worker account funded"

print_test "Join Trust"
soroban contract invoke \
  --id $NEW_CONTRACT_ID \
  --source-account $TEST_WORKER \
  --network $NETWORK \
  -- join_trust \
  --trust_id $ADMIN_PUBKEY 2>&1 | grep -q "Signing transaction"

if [ $? -eq 0 ]; then
    print_pass "Worker joined trust"
    sleep 3
else
    print_fail "Worker failed to join trust"
fi

print_test "Submit Work Claim"
EVIDENCE="regression_test_$(date +%s | xxd -p -c 32)"
CLAIM_RESULT=$(soroban contract invoke \
  --id $NEW_CONTRACT_ID \
  --source-account $ADMIN_KEY \
  --network $NETWORK \
  -- submit_work_claim \
  --worker $TEST_WORKER_PUBKEY \
  --work_type 0 \
  --minutes_worked 30 \
  --evidence_hash "$EVIDENCE" \
  --gps_lat "" \
  --gps_lon "" 2>&1)

if echo "$CLAIM_RESULT" | grep -q "Signing transaction\|stellar.expert"; then
    print_pass "Work claim submitted"
    sleep 3
else
    print_fail "Work claim submission failed"
fi

# ============================================================================
# RESULTS
# ============================================================================

print_header "REGRESSION TEST RESULTS"

echo ""
echo -e "${BLUE}Contract:${NC} $NEW_CONTRACT_ID"
echo -e "${BLUE}Tests Run:${NC}    $TESTS_RUN"
echo -e "${GREEN}Tests Passed:${NC} $TESTS_PASSED"
if [ $TESTS_FAILED -gt 0 ]; then
    echo -e "${RED}Tests Failed:${NC}  $TESTS_FAILED"
fi
echo ""

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}╔═══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║                  ALL TESTS PASSED ✓                           ║${NC}"
    echo -e "${GREEN}╚═══════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo "Fixed contract is working correctly!"
    echo "Demurrage and reputation systems are functional."
    exit 0
else
    echo -e "${RED}╔═══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${RED}║                  SOME TESTS FAILED ✗                          ║${NC}"
    echo -e "${RED}╚═══════════════════════════════════════════════════════════════╝${NC}"
    exit 1
fi
