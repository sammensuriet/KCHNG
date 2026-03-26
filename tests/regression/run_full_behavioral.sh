#!/bin/bash
# Run full behavioral tests with timeout and better error handling

echo "Running full behavioral tests..."
echo "This will take 10-20 minutes"
echo ""

# Run with timeout to prevent hanging
timeout 300 bash $(git rev-parse --show-toplevel)/tests/regression/full_behavioral_tests.sh 2>&1 | tee /tmp/behavioral_test_output.txt

EXIT_CODE=$?

echo ""
echo "============================================"
echo "Test completed with exit code: $EXIT_CODE"
echo "============================================"
echo ""

# Show results
grep -E "PASS|FAIL|SKIP|Tests Run|Tests Passed|Tests Failed" /tmp/behavioral_test_output.txt | tail -20

exit $EXIT_CODE
