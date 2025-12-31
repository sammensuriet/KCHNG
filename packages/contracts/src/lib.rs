#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, U256};

// Constants for demurrage calculation
const SECONDS_PER_DAY: u64 = 86_400;
const DEMURRAGE_PERIOD_DAYS: u64 = 7;
const DEMURRAGE_AMOUNT: u64 = 2; // KCHNG burned per 7 days of inactivity

/// Tracks the last activity timestamp for each account
#[contracttype]
pub struct AccountData {
    pub last_activity: u64,
    pub balance: U256,
}

/// Data for apps that want to implement additional demurrage
#[contracttype]
pub struct AppDemurrageEntry {
    pub app_id: Address,
    pub additional_rate: u64, // Additional demurrage per period (basis points or custom)
}

/// KCHNG Token Contract with native on-chain demurrage
#[contract]
pub struct KchngToken;

#[contractimpl]
impl KchngToken {
    // Contract constructor - called automatically during deployment
    // Parameters should be passed via soroban contract deploy -- --creator ADDRESS --initial_supply VALUE
    pub fn __constructor(env: Env, creator: Address, initial_supply: U256) {
        // Store the creator as admin
        env.storage().instance().set(&U256::from_u32(&env, 0), &creator);

        // Set initial balance for creator
        let account_data = AccountData {
            last_activity: env.ledger().timestamp(),
            balance: initial_supply.clone(),
        };

        let mut accounts: Map<Address, AccountData> = Map::new(&env);
        accounts.set(creator.clone(), account_data);

        env.storage().persistent().set(&U256::from_u32(&env, 2), &accounts);

        // Track total supply
        env.storage().instance().set(&U256::from_u32(&env, 3), &initial_supply);
    }

    /// Initialize the token with initial supply to the creator (legacy method)
    pub fn init(env: Env, creator: Address, initial_supply: U256) {
        // Check if already initialized by checking if accounts map exists
        if env.storage().persistent().has(&U256::from_u32(&env, 2)) {
            // Verify admin matches (get admin from instance storage)
            let admin_result: Option<Address> = env.storage().instance().get(&U256::from_u32(&env, 0));
            if let Some(admin) = admin_result {
                if admin != creator {
                    panic!("Already initialized with different admin");
                }
                return; // Already initialized by same creator, no-op
            }
        }

        // Store the creator as admin (this creates instance storage)
        env.storage().instance().set(&U256::from_u32(&env, 0), &creator);

        // Set initial balance for creator
        let account_data = AccountData {
            last_activity: env.ledger().timestamp(),
            balance: initial_supply.clone(),
        };

        let mut accounts: Map<Address, AccountData> = Map::new(&env);
        accounts.set(creator.clone(), account_data);

        env.storage().persistent().set(&U256::from_u32(&env, 2), &accounts);

        // Track total supply
        env.storage().instance().set(&U256::from_u32(&env, 3), &initial_supply);
    }

    /// Get the current balance of an account (after applying demurrage)
    pub fn balance(env: Env, account: Address) -> U256 {
        let accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&U256::from_u32(&env, 2)).unwrap();

        if let Some(data) = accounts.get(account.clone()) {
            // Calculate and return balance with demurrage applied
            Self::calculate_balance_with_demurrage(&env, account, &data)
        } else {
            U256::from_u32(&env, 0)
        }
    }

    /// Transfer tokens from one account to another
    pub fn transfer(env: Env, from: Address, to: Address, amount: U256) {
        from.require_auth();

        let accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&U256::from_u32(&env, 2)).unwrap();

        // Check if sender exists and has balance
        let from_data = match accounts.get(from.clone()) {
            Some(data) => data,
            None => panic!("Insufficient balance"),
        };

        // Apply demurrage to sender
        let balance_after_demurrage =
            Self::calculate_balance_with_demurrage(&env, from.clone(), &from_data);

        if balance_after_demurrage < amount {
            panic!("Insufficient balance");
        }

        let mut accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&U256::from_u32(&env, 2)).unwrap();

        // Update sender with demurrage applied and transfer amount deducted
        let mut updated_from = from_data;
        updated_from.balance = balance_after_demurrage.sub(&amount);
        updated_from.last_activity = env.ledger().timestamp();
        accounts.set(from.clone(), updated_from);

        // Get and update recipient balance
        let current_time = env.ledger().timestamp();
        let to_data = accounts.get(to.clone()).unwrap_or(AccountData {
            last_activity: current_time,
            balance: U256::from_u32(&env, 0),
        });

        let mut updated_to = to_data;
        updated_to.balance = updated_to.balance.add(&amount);
        updated_to.last_activity = current_time;
        accounts.set(to.clone(), updated_to);

        env.storage().persistent().set(&U256::from_u32(&env, 2), &accounts);
    }

    /// Mint new tokens (admin only)
    pub fn mint(env: Env, admin: Address, to: Address, amount: U256) {
        // Verify admin
        let stored_admin: Address = env.storage().instance().get(&U256::from_u32(&env, 0)).unwrap();
        admin.require_auth();
        if admin != stored_admin {
            panic!("Not authorized");
        }

        let mut accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&U256::from_u32(&env, 2)).unwrap();

        let current_time = env.ledger().timestamp();
        let to_data = accounts.get(to.clone()).unwrap_or(AccountData {
            last_activity: current_time,
            balance: U256::from_u32(&env, 0),
        });

        let mut updated_to = to_data;
        updated_to.balance = updated_to.balance.add(&amount);
        updated_to.last_activity = current_time;
        accounts.set(to.clone(), updated_to);

        env.storage().persistent().set(&U256::from_u32(&env, 2), &accounts);

        // Update total supply
        let mut total_supply: U256 = env.storage().instance().get(&U256::from_u32(&env, 3)).unwrap();
        total_supply = total_supply.add(&amount);
        env.storage().instance().set(&U256::from_u32(&env, 3), &total_supply);
    }

    /// Get the total supply
    pub fn total_supply(env: Env) -> U256 {
        env.storage().instance().get(&U256::from_u32(&env, 3)).unwrap()
    }

    /// Register an app for additional demurrage logic
    pub fn register_app(env: Env, admin: Address, app_id: Address, additional_rate: u64) {
        let stored_admin: Address = env.storage().instance().get(&U256::from_u32(&env, 0)).unwrap();
        admin.require_auth();
        if admin != stored_admin {
            panic!("Not authorized");
        }

        let mut apps: Map<Address, AppDemurrageEntry> =
            env.storage().persistent().get(&U256::from_u32(&env, 4)).unwrap();

        let entry = AppDemurrageEntry {
            app_id: app_id.clone(),
            additional_rate,
        };
        apps.set(app_id, entry);

        env.storage().persistent().set(&U256::from_u32(&env, 4), &apps);
    }

    /// Calculate balance after applying demurrage
    fn calculate_balance_with_demurrage(
        env: &Env,
        _account: Address,
        data: &AccountData,
    ) -> U256 {
        let current_timestamp = env.ledger().timestamp();
        let last_activity: u64 = data.last_activity;

        if current_timestamp <= last_activity {
            return data.balance.clone();
        }

        let inactive_seconds = current_timestamp - last_activity;
        let inactive_days = inactive_seconds / SECONDS_PER_DAY;

        if inactive_days < DEMURRAGE_PERIOD_DAYS {
            return data.balance.clone();
        }

        // Calculate number of complete 7-day periods
        let periods = inactive_days / DEMURRAGE_PERIOD_DAYS;
        let base_burn = periods * DEMURRAGE_AMOUNT;

        // Apply base demurrage - use from_u128 for larger values
        let burn_amount = U256::from_u128(env, base_burn as u128);

        if data.balance > burn_amount {
            data.balance.sub(&burn_amount)
        } else {
            U256::from_u32(env, 0)
        }
    }
}

#[cfg(test)]
mod test;
