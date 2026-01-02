#!/bin/bash
# Quick regression test for fixed contract

CONTRACT="CBMICVZ3FOVBLUBGOHU33FLCEO4ZSODIT7RS4GH27LECR6KHPNJP4CNB"
ADMIN="kchng_admin"
ADMIN_PUB="GCW4XHQLIK3VHXUGNXJ4NFLW5JDWZZ3UT2XJCYTZSO6AQJZUUDB7RVMS"
NETWORK="testnet"

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║     FIXED CONTRACT REGRESSION TEST                            ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""
echo "Contract: $CONTRACT"
echo ""

# Test 1: Total Supply
echo "Test 1: Total Supply"
SUPPLY=$(soroban contract invoke \
  --id $CONTRACT \
  --source-account $ADMIN \
  --network $NETWORK \
  -- total_supply 2>&1 | grep -oP '\d+')

if [ "$SUPPLY" = "1000000" ]; then
    echo "✓ PASS: Total supply = 1,000,000 KCHNG"
else
    echo "✗ FAIL: Total supply = $SUPPLY"
fi
echo ""

# Test 2: Admin Balance
echo "Test 2: Admin Balance"
BALANCE=$(soroban contract invoke \
  --id $CONTRACT \
  --source-account $ADMIN \
  --network $NETWORK \
  -- balance \
  --account $ADMIN_PUB 2>&1 | grep -oP '\d+')

if [ "$BALANCE" = "1000000" ]; then
    echo "✓ PASS: Admin balance = 1,000,000 KCHNG"
else
    echo "✗ FAIL: Admin balance = $BALANCE"
fi
echo ""

# Test 3: Register Trust
echo "Test 3: Register Trust"
RESULT=$(soroban contract invoke \
  --id $CONTRACT \
  --source-account $ADMIN \
  --network $NETWORK \
  -- register_trust \
  --governor $ADMIN_PUB \
  --name RegressionTest \
  --annual-rate-bps 1200 \
  --demurrage-period-days 30 2>&1)

if echo "$RESULT" | grep -q "Signing transaction"; then
    echo "✓ PASS: Trust registration successful"
    sleep 3
else
    echo "✗ FAIL: Trust registration failed"
fi
echo ""

# Test 4: Get Trust Info
echo "Test 4: Get Trust Info"
TRUST_INFO=$(soroban contract invoke \
  --id $CONTRACT \
  --source-account $ADMIN \
  --network $NETWORK \
  -- get_trust_info \
  --trust-id $ADMIN_PUB 2>&1)

if echo "$TRUST_INFO" | grep -q "RegressionTest\|1200"; then
    echo "✓ PASS: Trust info retrieved"
else
    echo "✗ FAIL: Could not get trust info"
fi
echo ""

# Test 5: Transfer Tokens
echo "Test 5: Transfer Tokens"
RESULT=$(soroban contract invoke \
  --id $CONTRACT \
  --source-account $ADMIN \
  --network $NETWORK \
  -- transfer \
  --from $ADMIN_PUB \
  --to $ADMIN_PUB \
  --amount 100 2>&1)

if echo "$RESULT" | grep -q "Signing transaction"; then
    echo "✓ PASS: Transfer successful"
    sleep 3
else
    echo "✗ FAIL: Transfer failed"
fi
echo ""

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║     BASIC REGRESSION TEST COMPLETE                              ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
