use candid::Principal;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, StableCell};
use std::cell::RefCell;

use crate::types::memory::Memory;
use crate::types::states::{Account, AccountMetrics};

const UPGRADES_MEMORY: MemoryId = MemoryId::new(0);
const METRICS_MEMORY: MemoryId = MemoryId::new(1);
const USER_ACCOUNTS_MEMORY: MemoryId = MemoryId::new(2);

std::thread_local! {
     // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
    // return a memory that can be used by stable structures
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // StableCell to store Account Canister Metrics
    pub static ACCOUNT_METRICS: RefCell<StableCell<AccountMetrics, Memory>> = RefCell::new(
            StableCell::init(
                MEMORY_MANAGER.with(|m| m.borrow().get(METRICS_MEMORY)),
               AccountMetrics::default()
        ).unwrap()
    );

    // StableMemory For Storing Derivatives Contracts
    pub static USER_ACCOUNTS: RefCell<StableBTreeMap<Principal , Account , Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(USER_ACCOUNTS_MEMORY)),
        )
    );
}

// *title: Implementation to get Account Canister Memory Upgrades
pub fn get_memory_upgrades() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(UPGRADES_MEMORY))
}
