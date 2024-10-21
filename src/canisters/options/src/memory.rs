use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::DefaultMemoryImpl;
use std::cell::RefCell;

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

pub const UPGRADES_MEMORY: MemoryId = MemoryId::new(0);
pub const INIT_MEMORY: MemoryId = MemoryId::new(1);
pub const ICRC_STAKE_MEMORY: MemoryId = MemoryId::new(2);
pub const ICRC_STAKE_TIMESTAMP_MEMORY: MemoryId = MemoryId::new(3);
pub const ICRC_UNSTAKE_TIMESTAMP_MEMORY: MemoryId = MemoryId::new(4);
pub const ICRC_TOTAL_VALUE_LOCKED_MEMORY: MemoryId = MemoryId::new(5);
pub const STAKE_EXECUTION_LOGS_MEMORY: MemoryId = MemoryId::new(6);

std::thread_local! {

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =memory_manager_init();

}

pub(crate) fn _get_memory(memory_id: MemoryId) -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(memory_id))
}

/// Implementation to get AegisAccount Canister Memory Upgrades
pub(crate) fn get_memory_upgrades() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(UPGRADES_MEMORY))
}

pub(crate) fn memory_manager_init() -> RefCell<MemoryManager<DefaultMemoryImpl>> {
    RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()))
}
