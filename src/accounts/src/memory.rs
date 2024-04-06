use candid::Principal;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, StableCell};
use std::cell::RefCell;

use crate::types::memory::Memory;
use crate::types::states::initialization_configs::InitArgs;
use crate::types::states::State;
use crate::types::states::{Account, AccountMetrics};
use crate::types::states::{Constants, StableStates};

const UPGRADES_MEMORY: MemoryId = MemoryId::new(0);
const ACCOUNT_METRICS_MEMORY: MemoryId = MemoryId::new(1);
const USER_ACCOUNTS_MEMORY: MemoryId = MemoryId::new(2);
const CONSTANTS_MEMORY: MemoryId = MemoryId::new(3);
const INIT_MEMORY: MemoryId = MemoryId::new(4);

// type UserAccounts=

std::thread_local! {
    pub static STATE: RefCell<State> = RefCell::default();


     // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
    // return a memory that can be used by stable structures
static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =memory_manager_init();


}

fn get_memory(memory_id: MemoryId) -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(memory_id))
}

// *title: Implementation to get Account Canister Memory Upgrades
pub fn get_memory_upgrades() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(UPGRADES_MEMORY))
}

pub fn memory_manager_init() -> RefCell<MemoryManager<DefaultMemoryImpl>> {
    RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()))
}

pub fn user_accounts_init() -> RefCell<StableBTreeMap<Principal, Account, Memory>> {
    RefCell::new(StableBTreeMap::init(get_memory(USER_ACCOUNTS_MEMORY)))
}

pub fn init_stable_states() -> StableStates {
    StableStates {
        user_accounts: StableBTreeMap::init(get_memory(USER_ACCOUNTS_MEMORY)),
        constants: StableCell::init(get_memory(CONSTANTS_MEMORY), Constants::default()).unwrap(),
        init: StableCell::init(get_memory(INIT_MEMORY), InitArgs::default()).unwrap(),
        account_metrics: StableCell::init(
            get_memory(ACCOUNT_METRICS_MEMORY),
            AccountMetrics::default(),
        )
        .unwrap(),
    }
}
