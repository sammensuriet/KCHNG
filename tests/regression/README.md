# KCHNG Regression Tests

Automated tests to verify KCHNG contract functionality on Stellar Testnet.

## Purpose

These tests verify that core contract features work correctly after:
- Code changes
- Contract deployments
- Configuration updates
- Bug fixes

## Running Tests

### Quick Test
```bash
./tests/regression/test_contract_functionality.sh
```

### Full Test Suite
```bash
cd tests/regression
./test_contract_functionality.sh
```

## Test Coverage

### ✅ Working Features

1. **Contract Accessibility**
   - Contract responds to queries
   - Total supply readable

2. **Balance Queries**
   - Account balance retrieval
   - Demurrage-aware calculations

3. **Token Transfers**
   - Transfer between accounts
   - Atomic transactions
   - Balance updates

4. **Work Claim Submission**
   - Submit claim with evidence
   - Verifier assignment
   - Claim status tracking

5. **Worker Payments**
   - Tokens minted to worker after approval
   - Total supply increases correctly
   - 30 minutes = 1 KCHNG

6. **Claim Rejection**
   - Rejected claims don't mint tokens
   - Status changes correctly
   - No worker payment

7. **Demurrage Configuration**
   - 12% annual rate
   - 30-day periods
   - Per-trust customization

8. **Cross-Trust Exchange**
   - Rate calculation works
   - Adjusts for different demurrage rates

### ⚠️ Known Issues

9. **Reputation System**
   - Structurally defined but not functional
   - Score always 500, never changes
   - See: `docs/2026-01-02_reputation-system-gap-analysis.md`

## Test Scenarios

### Scenario 1: Worker Payment Flow
```
1. Worker submits claim (30 min work)
2. Verifier 1 approves
3. Verifier 2 approves
4. Worker balance increases by 1 KCHNG ✓
5. Total supply increases by 1 ✓
```

### Scenario 2: Token Consumption
```
1. Worker has 1 KCHNG
2. Transfers 1 KCHNG to meal provider
3. Worker balance decreases by 1 ✓
4. Provider balance increases by 1 ✓
```

### Scenario 3: Claim Rejection
```
1. Worker submits claim
2. Verifier 1 rejects
3. Verifier 2 rejects
4. Claim status = Rejected ✓
5. No tokens minted ✓
```

## Requirements

- Soroban CLI installed
- `kchng_admin` key configured
- Access to Stellar Testnet
- Contract deployed: `CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX`

## Test Data

### Accounts Used
- **Admin:** `kchng_admin` (GCW4XHQLIK3VHXUG...)
- **Test User:** `kchng_test_user` (for transfers)
- **Worker:** Various accounts for claims

### Trusts Used
- **Urban Elder Care:** GCW4XHQLIK3VHXUG...
- **Rural Health:** GAM6N54Y5SBFUDV2YRLZB45CBPMAEJO5KACRSAR35F37GYDGGUGNNDK2

## Interpreting Results

### PASS ✅
Feature works as designed. No action needed.

### FAIL ❌
Feature broken or not working:
1. Check contract logs
2. Verify account permissions
3. Check network connectivity
4. Review recent code changes

### KNOWN ISSUE ⚠️
Feature documented as not working:
- Reputation system
- See gap analysis document

## Regression Testing Best Practices

### When to Run
- Before deploying to mainnet
- After contract code changes
- After configuration updates
- After bug fixes
- Periodically (weekly/monthly)

### What to Check
1. All tests pass ✅
2. No unexpected failures
3. Known issues still known
4. New features tested

### Troubleshooting

**Resource Limit Exceeded:**
- Wait a few minutes between test runs
- Testnet has rate limits

**Access Control Errors:**
- Verify `kchng_admin` key has permissions
- Check account is registered in trust

**Transaction Failures:**
- Check testnet connectivity
- Verify contract ID is correct
- Ensure sufficient testnet XLM

## Adding New Tests

To add a new test scenario:

1. Create a new test function in the script
2. Follow the existing pattern:
   ```bash
   print_header "Test N: Description"
   TESTS_RUN=$((TESTS_RUN + 1))
   print_test "What we're testing"
   # ... test code ...
   # Use print_pass or print_fail
   ```

3. Update this README with new test coverage
4. Run tests to verify

## Continuous Integration

These tests can be integrated into CI/CD:

```yaml
# Example GitHub Actions
- name: Run KCHNG Regression Tests
  run: |
    cd <kchng-repo>
    ./tests/regression/test_contract_functionality.sh
```

## Related Documentation

- [Token Consumption Test Report](../../docs/2026-01-02_token-consumption-test-report.md)
- [Reputation System Gap Analysis](../../docs/2026-01-02_reputation-system-gap-analysis.md)
- [Real Testnet Simulation Report](../../docs/2026-01-02_real-testnet-simulation-report.md)

## Version History

- **2026-01-02:** Initial test suite created
  - 9 test scenarios
  - Worker payment verification
  - Token consumption tests
  - Reputation system check

## Maintainers

- Created during contract testing phase
- Update when contract features change
- Add tests for new functionality

---

**Last Updated:** 2026-01-02
**Contract Version:** Testnet Deployment
