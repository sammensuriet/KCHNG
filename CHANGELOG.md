# KCHNG Contract Changelog

All notable changes to the KCHNG smart contract deployments will be documented in this file.

## Contract Addresses Summary

| Network | Version | Contract ID | Status |
|---------|---------|-------------|--------|
| Mainnet | v2 | `CCPZSMXRKN3FM7WDIA3NZMJMZ6E577YDXFBUKACFQKTLBP7HZH63A5OK` | **Active** (frontend) |
| Mainnet | v4 | `CAKJ4KXWR57TPAPBKM3KTK4RCAJNBT3TVOBXTXDZVVBNLBAL7UQKHLRO` | Deployed (not connected) |
| Testnet | v3 | `CDMSMELWB6ERPXOSD7L3DXXJIG5A6PMBT6R6VFV6FOENKYYN7QNQPBFH` | Listed on /about page |
| Testnet | v5 | `CCKS2V6RGWI3TVPPMARHNTN22WIXAYOD423NKME6HAKBWKTAKGLRQLVO` | **Active** (frontend) |

---

## [v5] - 2026-03-09

### Testnet Deployment
- **Contract ID:** `CCKS2V6RGWI3TVPPMARHNTN22WIXAYOD423NKME6HAKBWKTAKGLRQLVO`
- **Explorer:** [stellar.expert](https://stellar.expert/explorer/testnet/contract/CCKS2V6RGWI3TVPPMARHNTN22WIXAYOD423NKME6HAKBWKTAKGLRQLVO)

### Added
- `migrate_data()` function for contract upgrades (restores instance storage from old contract)
- Instance storage getters: `get_admin()`, `get_protocol_version()`, `get_total_supply_raw()`, `get_next_claim_id()`, `get_next_proposal_id()`, `get_migration_status()`
- `MigrationStatus` and `MigrationResult` types
- `MigrationCompleted` event

### Migration Workflow
1. Deploy new contract version
2. Call `new_contract.migrate_data(admin, old_contract_address, expected_version)`
3. Verify `MigrationResult.success == true`
4. Protocol version auto-increments by 1

---

## [v4] - 2026-03-08

### Mainnet Deployment
- **Contract ID:** `CAKJ4KXWR57TPAPBKM3KTK4RCAJNBT3TVOBXTXDZVVBNLBAL7UQKHLRO`
- **Explorer:** [stellar.expert](https://stellar.expert/explorer/public/contract/CAKJ4KXWR57TPAPBKM3KTK4RCAJNBT3TVOBXTXDZVVBNLBAL7UQKHLRO)

### Added
- Grace abuse penalty system (report_grace_abuse function)
- Reputation penalties for grace period abuse (-50 worker, -25 member)
- `REP_EVENT_GRACE_ABUSED` event type

### Note
- This contract was deployed but the frontend was not updated to use it
- Frontend still points to v2 mainnet contract

---

## [v3] - 2026-03-01

### Testnet Deployment
- **Contract ID:** `CDMSMELWB6ERPXOSD7L3DXXJIG5A6PMBT6R6VFV6FOENKYYN7QNQPBFH`
- **Explorer:** [stellar.expert](https://stellar.expert/explorer/testnet/contract/CDMSMELWB6ERPXOSD7L3DXXJIG5A6PMBT6R6VFV6FOENKYYN7QNQPBFH)

### Added
- Contract version visibility to console and /about page
- Comprehensive i18n support (EN, ES, RU, ZH)

---

## [v2] - 2026-02-19

### Mainnet Deployment (Current Active)
- **Contract ID:** `CCPZSMXRKN3FM7WDIA3NZMJMZ6E577YDXFBUKACFQKTLBP7HZH63A5OK`
- **Explorer:** [stellar.expert](https://stellar.expert/explorer/public/contract/CCPZSMXRKN3FM7WDIA3NZMJMZ6E577YDXFBUKACFQKTLBP7HZH63A5OK)

### Added
- Anti-gaming protections (transfer minimums, cooldowns, pattern detection)
- Reputation system (0-1000 score, starts at 500)
- TF2T (Tit-for-2-Tats) pattern penalty system
- Role-based reputation tracking (Governor, Verifier, Oracle, Worker, Member)
- Verifier probation and removal thresholds
- Multi-trust verifier support (700+ reputation required)
- Governance proposals with quorum requirements
- Oracle grace period grants with yearly limits
- `RoleReleased` event for voluntary step-downs

### Testnet (same version)
- **Contract ID:** `CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX`

---

## [v1] - 2026-02-11

### Mainnet Deployment (Initial)
- **Contract ID:** `CDMKVYIU6KAATZXLLFT6KTJCKXCWP3HPYNMA3HMEUUWYVCM5DJVZ5AQS`

### Features
- Core token transfers with balance checking
- Admin-only mint function
- Percentage-based demurrage (Wörgl model)
- Time-based activity tracking
- Trust system (register/join/leave)
- Work verification with multi-verifier assignment
- Grace periods (Emergency, Illness, Community)

---

## Pre-Release Testnet History

| Date | Contract ID | Description |
|------|-------------|-------------|
| 2026-01-01 | `CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX` | Full-featured KCHNG contract |
| 2025-12-31 | `CAST22E2ZUBSRVFHQ3E6EOZAW33ZA3JM6NDT7RZSUIWP7RH5HC34SOKB` | Added __constructor |
| 2025-12-31 | `CBWX2LIGYXGGVIDJPZJDSY44YULMS6ENLLGOR3VBAZTCDVR47EGJIGB6` | Initial testnet deployment |
| 2025-12-14 | `CAHNH3AROXTDUILFB3WSM3NDA3ENUOLG5CSDK4AKRFWY2EZSMB3HZCLA` | Time-Standard Economic Model |

---

## Known Issues

### Contract Address Mismatch
The following files have inconsistent contract addresses:

| File | Network | Contract ID | Issue |
|------|---------|-------------|-------|
| `packages/shared/src/networks.ts` | Mainnet | `CCPZSMXRKN3FM7WDIA3NZMJMZ6E577YDXFBUKACFQKTLBP7HZH63A5OK` | v2 (active) |
| `packages/shared/src/networks.ts` | Testnet | `CCKS2V6RGWI3TVPPMARHNTN22WIXAYOD423NKME6HAKBWKTAKGLRQLVO` | v5 (active) |
| `packages/frontend/src/routes/about/+page.svelte` | Mainnet | `CAKJ4KXWR57TPAPBKM3KTK4RCAJNBT3TVOBXTXDZVVBNLBAL7UQKHLRO` | v4 (not connected) |
| `packages/frontend/src/routes/about/+page.svelte` | Testnet | `CDMSMELWB6ERPXOSD7L3DXXJIG5A6PMBT6R6VFV6FOENKYYN7QNQPBFH` | v3 (outdated) |
| `README.md` | Mainnet | `CAKJ4KXWR57TPAPBKM3KTK4RCAJNBT3TVOBXTXDZVVBNLBAL7UQKHLRO` | v4 (not connected) |
| `README.md` | Testnet | `CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX` | v2 (outdated) |
| `docs/PRD.md` | Mainnet | `CAKJ4KXWR57TPAPBKM3KTK4RCAJNBT3TVOBXTXDZVVBNLBAL7UQKHLRO` | v4 (not connected) |

### Recommended Actions
1. Update `/about` page and `README.md` to use `networks.ts` as single source of truth
2. Decide on mainnet upgrade path: stay on v2 or migrate to v5
3. If migrating to v5, deploy v5 to mainnet and use `migrate_data()` to restore instance storage
