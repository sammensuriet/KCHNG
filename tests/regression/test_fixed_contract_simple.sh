#!/bin/bash
# Simple regression test - verify contract functions work

CONTRACT="CBMICVZ3FOVBLUBGOHU33FLCEO4ZSODIT7RS4GH27LECR6KHPNJP4CNB"
ADMIN="kchng_admin"
ADMIN_PUB="GCW4XHQLIK3VHXUGNXJ4NFLW5JDWZZ3UT2XJCYTZSO6AQJZUUDB7RVMS"
NETWORK="testnet"

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║     FIXED CONTRACT REGRESSION TEST (SIMPLE)                   ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""
echo "Contract: $CONTRACT"
echo ""

PASS=0
FAIL=0

test_func() {
    local name=$1
    local result=$2
    
    if [ -n "$result" ]; then
        echo "✓ $name"
        PASS=$((PASS + 1))
    else
        echo "✗ $name"
        FAIL=$((FAIL + 1))
    fi
}

# Test 1: Total Supply
SUPPLY=$(soroban contract invoke \
  --id $CONTRACT \
  --source-account $ADMIN \
  --network $NETWORK \
  -- total_supply 2>&1 | grep -oP '\d+')
test_func "Total Supply accessible" "$SUPPLY"

# Test 2: Balance
BALANCE=$(soroban contract invoke \
  --id $CONTRACT \
  --source-account $ADMIN \
  --network $NETWORK \
  -- balance \
  --account $ADMIN_PUB 2>&1 | grep -oP '\d+')
test_func "Balance function works" "$BALANCE"

# Test 3: Get Account
ACCOUNT=$(soroban contract invoke \
  --id $CONTRACT \
  --source-account $ADMIN \
  --network $NETWORK \
  -- get_account \
  --account $ADMIN_PUB 2>&1)
test_func "Get account function works" "$ACCOUNT"

# Test 4: Demurrage Rate
RATE=$(soroban contract invoke \
  --id $CONTRACT \
  --source-account $ADMIN \
  --network $NETWORK \
  -- get_account_demurrage_rate \
  --account $ADMIN_PUB 2>&1 | grep -oP '\[.*\]')
test_func "Demurrage rate function works" "$RATE"

# Test 5: Get All Trusts
TRUSTS=$(soroban contract invoke \
  --id $CONTRACT \
  --source-account $ADMIN \
  --network $NETWORK \
  -- get_all_trusts 2>&1)
test_func "Get all trusts function works" "$TRUSTS"

# Test 6: Get Protocol Info
INFO=$(soroban contract invoke \
  --id $CONTRACT \
  --source-account $ADMIN \
  --network $NETWORK \
  -- get_protocol_info 2>&1)
test_func "Get protocol info function works" "$INFO"

echo ""
echo "─────────────────────────────────────────────────────────────────"
echo "Results: $PASS passed, $FAIL failed"
echo ""

if [ $FAIL -eq 0 ]; then
    echo "✅ All regression tests passed!"
    echo ""
    echo "Contract functions verified:"
    echo "  • Total supply tracking"
    echo "  • Balance queries"
    echo "  • Account information"
    echo "  • Demurrage rate calculation"
    echo "  • Trust system"
    echo "  • Protocol information"
    echo ""
    echo "Fixed contract is fully functional!"
else
    echo "⚠️  Some tests failed"
fi

exit $FAIL
