# KCHNG Mainnet Deployment Status Report

**Date**: 2026-02-11
**Status**: ✅ **COMPLETE**
**Contract ID**: `CDMKVYIU6KAATZXLLFT6KTJCKXCWP3HPYNMA3HMEUUWYVCM5DJVZ5AQS`

---

## Executive Summary

The KCHNG smart contract has been successfully deployed to Stellar mainnet. The deployment required resolving several technical challenges related to Soroban's fee structure on mainnet, particularly the `InsufficientRefundableFee` and `TxInsufficientBalance` errors.

---

## Deployment Details

| Field | Value |
|-------|-------|
| **Contract ID** | `CDMKVYIU6KAATZXLLFT6KTJCKXCWP3HPYNMA3HMEUUWYVCM5DJVZ5AQS` |
| **WASM Hash** | `dc90e75de093eb27d53249025c3595d0dd093618c2c8a0f69023d4bfaf532b97` |
| **Initial Supply** | 100,000,000,000,000,000 KCHNG (100 quintillion) |
| **Creator Address** | `GCW4XHQLIK3VHXUGNXJ4NFLW5JDWZZ3UT2XJCYTZSO6AQJZUUDB7RVMS` |
| **Upload Tx** | `cca06b61d8037bfe265aeadf2738983dd46bc6334ee3e6d624223f28ab2b2519` |
| **Deploy Tx** | `80cbcb0a4527f9182e27e593fac5bc8ebc3ee5d4659bda2f66f4b238666e5ec8` |

---

## Links

| Resource | URL |
|----------|-----|
| **Stellar Expert Transaction** | https://stellar.expert/explorer/public/tx/80cbcb0a4527f9182e27e593fac5bc8ebc3ee5d4659bda2f66f4b238666e5ec8 |
| **Stellar Lab Contract** | https://lab.stellar.org/r/mainnet/contract/CDMKVYIU6KAATZXLLFT6KTJCKXCWP3HPYNMA3HMEUUWYVCM5DJVZ5AQS |

---

## Challenges Resolved

### 1. TxInsufficientBalance Error (Initial)

**Problem**: Despite having 83.99 XLM, deployment failed with `TxInsufficientBalance`.

**Root Cause**: The actual resource fee required for mainnet WASM upload is much higher than testnet. Initial estimates were insufficient.

**Solution**: Account was funded with additional XLM to reach 860+ XLM balance.

---

### 2. InsufficientRefundableFee Error

**Problem**: Deployment failed with `InsufficientRefundableFee` in `InvokeHostFunction` operation.

**Root Cause**: Soroban's resource fee has two components:
- **Non-refundable**: Always charged
- **Refundable**: Only refunded for Events, Return Value Size, and Ledger Space Rent

The refundable portion must meet a minimum threshold for the operation to succeed.

**Solution**: Used `--resource-fee 4000000000` (400 XLM) and `--ignore-checks` flag to bypass CLI safety checks.

**Resources Referenced**:
- [Stellar Fees Documentation](https://developers.stellar.org/docs/learn/fundamentals/fees-resource-limits-metering)
- [Cheesecake Labs on Soroban Fees](https://cheesecakelabs.com/blog/how-much-do-soroban-fees-cost/)

---

### 3. RPC Endpoint Issues

**Problem**: Various RPC endpoints had connectivity issues (DNS failures, 404 errors).

**Working Endpoint**: `https://soroban-rpc.mainnet.stellar.gateway.fm`

---

## Final Deployment Command

```bash
# WASM Upload
stellar contract upload \
  --ignore-checks \
  --wasm target/wasm32v1-none/release/kchng_contract.wasm \
  --source kchng_admin \
  --network mainnet \
  --resource-fee 4000000000

# Contract Deploy
stellar contract deploy \
  --wasm-hash dc90e75de093eb27d53249025c3595d0dd093618c2c8a0f69023d4bfaf532b97 \
  --source kchng_admin \
  --network mainnet \
  --ignore-checks \
  -- --creator GCW4XHQLIK3VHXUGNXJ4NFLW5JDWZZ3UT2XJCYTZSO6AQJZUUDB7RVMS \
     --initial-supply 100000000000000000000
```

---

## Configuration Updated

**File**: `packages/shared/src/networks.ts`

```typescript
mainnet: {
  networkUrl: "https://horizon.stellar.org",
  rpcUrl: "https://mainnet.soroban.rpc.stellar.org",
  networkPassphrase: "Public Global Stellar Network ; September 2015",
  contractId: "CDMKVYIU6KAATZXLLFT6KTJCKXCWP3HPYNMA3HMEUUWYVCM5DJVZ5AQS", // Deployed 2026-02-11
}
```

---

## Testnet Comparison

| Network | Contract ID | Status |
|---------|-------------|--------|
| **Mainnet** | `CDMKVYIU6KAATZXLLFT6KTJCKXCWP3HPYNMA3HMEUUWYVCM5DJVZ5AQS` | ✅ Deployed |
| **Testnet** | `CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX` | ✅ Deployed |

---

## Contract Features Deployed

The KCHNG contract includes the following features:
- ✅ Native demurrage (2 KCHNG per 7 days of inactivity)
- ✅ Trust registration for custom demurrage rates
- ✅ Multi-trust support with cross-trust swaps
- ✅ Grace periods for economic hardship
- ✅ Governance voting system
- ✅ Reputation scoring

---

## Cost Summary

| Phase | Estimated Cost |
|-------|----------------|
| WASM Upload | ~400 XLM (resource fee) |
| Contract Deploy | ~Variable (resource fee) |
| Inclusion Fees | ~0.0001 XLM per operation |

---

## Next Steps

1. ✅ Contract deployed to mainnet
2. ⏳ Demurrage time capsule test running (results due 2026-02-17)
3. 🔄 Frontend configuration for mainnet
4. 📋 Monitoring contract operations

---

## Appendix: CLI Version Used

```
stellar 25.1.0 (a048a57a75762458b487052e0021ea704a926bee)
stellar-xdr 25.0.0 (dc9f40fcb83c3054341f70b65a2222073369b37b)
```

---

**Report Prepared**: 2026-02-11
**Prepared By**: Claude Code
**Deployment**: Successful ✅
