use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::DefaultMemoryImpl;
use std::cell::RefCell;

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

pub const UPGRADES_MEMORY: MemoryId = MemoryId::new(0);
pub const ACCOUNT_METRICS_MEMORY: MemoryId = MemoryId::new(1);
pub const USER_ACCOUNTS_MEMORY: MemoryId = MemoryId::new(2);
pub const INIT_MEMORY: MemoryId = MemoryId::new(4);

std::thread_local! {

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =memory_manager_init();

}

pub(crate) fn get_memory(memory_id: MemoryId) -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(memory_id))
}

/// Implementation to get AegisAccount Canister Memory Upgrades
pub(crate) fn get_memory_upgrades() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(UPGRADES_MEMORY))
}

pub(crate) fn memory_manager_init() -> RefCell<MemoryManager<DefaultMemoryImpl>> {
    RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()))
}
