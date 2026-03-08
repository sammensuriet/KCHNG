# KCHNG Token Contract Deployment Report

**Date**: 2025-12-31
**Network**: Stellar Testnet
**Status**: ✅ Successfully Deployed and Initialized

---

## Executive Summary

The KCHNG token contract has been successfully deployed to Stellar testnet with automatic initialization via the `__constructor` function. The contract is fully operational with 100 KCHNG initial supply minted to the admin account.

---

## Contract Details

### Contract Information

| Property | Value |
|----------|-------|
| **Contract ID** | `CAST22E2ZUBSRVFHQ3E6EOZAW33ZA3JM6NDT7RZSUIWP7RH5HC34SOKB` |
| **WASM Hash** | `ae2b30d0f8d71b2e9c4167c90537af24dce085a89b3a8376625a7365a74f8c4c` |
| **Network** | Testnet |
| **Admin** | `GCW4XHQLIK3VHXUGNXJ4NFLW5JDWZZ3UT2XJCYTZSO6AQJZUUDB7RVMS` |

### Transaction Links

- **Deployment TX**: https://stellar.expert/explorer/testnet/tx/c214e941ea8a305804ae5a1347e050c1f3fccf8a128f62ef2910831504ba6c00
- **Operation ID**: `1051309209833473`
- **Ledger**: 244511

---

## Initial State

### Token Parameters

| Parameter | Value |
|-----------|-------|
| **Initial Supply** | 100,000,000,000,000,000,000 (100 KCHNG) |
| **Decimals** | 18 (stroop precision) |
| **Admin Balance** | 100,000,000,000,000,000,000 |

### Verified Functions

```bash
# Total Supply
$ soroban contract invoke --id CAST22E2ZUBSRVFHQ3E6EOZAW33ZA3JM6NDT7RZSUIWP7RH5HC34SOKB \
    --source kchng_admin --network testnet -- total_supply
"100000000000000000000"

# Admin Balance
$ soroban contract invoke --id CAST22E2ZUBSRVFHQ3E6EOZAW33ZA3JM6NDT7RZSUIWP7RH5HC34SOKB \
    --source kchng_admin --network testnet -- balance \
    --account "GCW4XHQLIK3VHXUGNXJ4NFLW5JDWZZ3UT2XJCYTZSO6AQJZUUDB7RVMS"
"100000000000000000000"
```

---

## Contract Architecture

### Storage Layout

| Key | Type | Description |
|-----|------|-------------|
| `U256(0)` | `Address` | Admin address |
| `U256(2)` | `Map<Address, AccountData>` | Account balances & activity |
| `U256(3)` | `U256` | Total supply |
| `U256(4)` | `Map<Address, AppDemurrageEntry>` | Registered apps |

### Data Structures

```rust
pub struct AccountData {
    pub last_activity: u64,
    pub balance: U256,
}

pub struct AppDemurrageEntry {
    pub app_id: Address,
    pub additional_rate: u64,
}
```

---

## Contract Functions

### Read Functions

| Function | Parameters | Returns | Description |
|----------|------------|---------|-------------|
| `total_supply` | - | `U256` | Get total token supply |
| `balance` | `account: Address` | `U256` | Get balance with demurrage applied |

### Write Functions

| Function | Parameters | Auth | Description |
|----------|------------|------|-------------|
| `transfer` | `from, to, amount` | From | Transfer tokens between accounts |
| `mint` | `admin, to, amount` | Admin | Mint new tokens (admin only) |
| `register_app` | `admin, app_id, rate` | Admin | Register app for additional demurrage |
| `init` | `creator, initial_supply` | Creator | Legacy initialization (idempotent) |

### Constructor

| Function | Parameters | Description |
|----------|------------|-------------|
| `__constructor` | `creator, initial_supply` | Auto-initializes contract on deployment |

---

## Demurrage Mechanics

### Parameters

| Constant | Value | Description |
|----------|-------|-------------|
| `SECONDS_PER_DAY` | 86,400 | Seconds in a day |
| `DEMURRAGE_PERIOD_DAYS` | 7 | Days per demurrage period |
| `DEMURRAGE_AMOUNT` | 2 | KCHNG burned per period |

### Calculation

```
inactive_seconds = current_timestamp - last_activity
inactive_days = inactive_seconds / 86,400
periods = inactive_days / 7
burn_amount = periods * 2

if balance > burn_amount:
    new_balance = balance - burn_amount
else:
    new_balance = 0
```

---

## Deployment Process

### 1. Build Contract

```bash
cd packages/contracts
RUSTFLAGS="-C target-feature=-reference-types" cargo build --release --target wasm32-unknown-unknown
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/kchng_contract.wasm \
    --wasm-out target/wasm32-unknown-unknown/release/kchng_contract.optimized.wasm
```

### 2. Deploy with Constructor

```bash
soroban contract deploy \
    --wasm packages/contracts/target/wasm32-unknown-unknown/release/kchng_contract.optimized.wasm \
    --source kchng_admin \
    --network testnet \
    --salt 123456789 \
    -- \
    --creator "GCW4XHQLIK3VHXUGNXJ4NFLW5JDWZZ3UT2XJCYTZSO6AQJZUUDB7RVMS" \
    --initial_supply "100000000000000000000"
```

### Key Learnings

1. **`--` Separator**: Required to distinguish CLI flags from constructor arguments
2. **Constructor Arguments**: Passed after the `--` separator
3. **Salt**: Explicit salt for deterministic contract ID calculation
4. **"xdr value invalid" Error**: Can be ignored if transaction succeeds on Horizon

---

## Network Configuration

### RPC Endpoints

| Network | RPC URL | Status |
|---------|---------|--------|
| Testnet | `https://soroban-testnet.stellar.org` | ✅ Working |
| Testnet | `https://testnet.soroban.rpc.stellar.org` | ❌ DNS fails |
| Testnet | `https://soroban-rpc.testnet.stellar.gateway.fm` | ✅ Alternative |

### Updated Config

`packages/shared/src/networks.ts`:
```typescript
testnet: {
  networkUrl: "https://horizon-testnet.stellar.org",
  rpcUrl: "https://soroban-testnet.stellar.org",
  networkPassphrase: "Test SDF Network ; September 2015",
  contractId: "CAST22E2ZUBSRVFHQ3E6EOZAW33ZA3JM6NDT7RZSUIWP7RH5HC34SOKB",
}
```

---

## Technical Specifications

### Soroban SDK

- **Version**: 22.0.8
- **Rust Version**: 1.91.1
- **Target**: `wasm32-unknown-unknown`

### Build Flags

```bash
RUSTFLAGS="-C target-feature=-reference-types"
```

Disables reference types for Soroban compatibility.

### WASM Output

| File | Size |
|------|------|
| `kchng_contract.wasm` | 20,190 bytes |
| `kchng_contract.optimized.wasm` | 10,345 bytes |

---

## Testing Commands

### Invoke Contract

```bash
# Total Supply
soroban contract invoke \
    --id CAST22E2ZUBSRVFHQ3E6EOZAW33ZA3JM6NDT7RZSUIWP7RH5HC34SOKB \
    --source kchng_admin \
    --network testnet \
    -- total_supply

# Balance Check
soroban contract invoke \
    --id CAST22E2ZUBSRVFHQ3E6EOZAW33ZA3JM6NDT7RZSUIWP7RH5HC34SOKB \
    --source kchng_admin \
    --network testnet \
    -- balance \
    --account "GCW4XHQLIK3VHXUGNXJ4NFLW5JDWZZ3UT2XJCYTZSO6AQJZUUDB7RVMS"

# Transfer (write, requires --send=yes)
soroban contract invoke \
    --id CAST22E2ZUBSRVFHQ3E6EOZAW33ZA3JM6NDT7RZSUIWP7RH5HC34SOKB \
    --source kchng_admin \
    --network testnet \
    -- transfer \
    --from "GCW4XHQLIK3VHXUGNXJ4NFLW5JDWZZ3UT2XJCYTZSO6AQJZUUDB7RVMS" \
    --to "<recipient_address>" \
    --amount "1000000000000000000"
```

---

## Next Steps

### 1. Wallet Integration Testing
- [x] Deploy contract
- [x] Verify initialization
- [ ] Test wallet connection flow
- [ ] Verify balance display
- [ ] Test demurrage visualization

### 2. Frontend Integration
- [ ] Update contract client with new contract ID
- [ ] Test transfer functionality
- [ ] Implement demurrage countdown UI
- [ ] Add error handling for non-initialized contracts

### 3. Mainnet Deployment
- [ ] Security audit
- [ ] Set initial supply for mainnet
- [ ] Deploy to mainnet
- [ ] Verify all functions on mainnet

---

## References

### Documentation

- [Upload and Deploy Contract | Stellar Docs](https://developers.stellar.org/docs/tools/lab/smart-contracts/upload-deploy-contract)
- [CAP-58: Support deploy with constructor arguments](https://github.com/stellar/stellar-cli/issues/1561)
- [Install and deploy a smart contract | Stellar Docs](https://developers.stellar.org/docs/build/guides/transactions/install-deploy-contract-with-code)

### Explorers

- [Stellar Expert](https://stellar.expert/explorer/testnet)
- [Stellar Lab](https://laboratory.stellar.org/#deployer?network=test)

### Contract Source

- **Location**: `packages/contracts/src/lib.rs`
- **WASM**: `packages/contracts/target/wasm32-unknown-unknown/release/kchng_contract.optimized.wasm`

---

## Appendix: Deployment History

### Previous Deployments (Uninitialized)

| Attempt | Contract ID | Status |
|---------|-------------|--------|
| 1 | `CCHRJ2VFA7265R6FNHVD3HZTZ6DAXME6BT7UYDYFDPQNUS5YZXNX5XJD` | Deployed, not initialized |
| 2 | `CBWX2LIGYXGGVIDJPZJDSY44YULMS6ENLLGOR3VBAZTCDVR47EGJIGB6` | Deployed, not initialized |
| 3 | `CCIBPPG74YWQVZRRFCE6KZFLSUEPHPY2DVSWOPILZOBYW7TIMCPEFR4U` | Deployed, not initialized |
| 4 | `BD5DUG46EPL5AMDUMTFBCSOG7AOOSFFH7E6YZK6JXVV5NM3XY3O5M6SJTUHQ` | Calculation error |

### Successful Deployment (with Constructor)

| Attempt | Contract ID | Status | TX Hash |
|---------|-------------|--------|---------|
| 5 | `CAST22E2ZUBSRVFHQ3E6EOZAW33ZA3JM6NDT7RZSUIWP7RH5HC34SOKB` | ✅ Initialized | `c214e941ea8a305804ae5a1347e050c1f3fccf8a128f62ef2910831504ba6c00` |

---

*Report generated: 2025-12-31*
*KCHNG Project*
