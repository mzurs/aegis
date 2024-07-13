use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::DefaultMemoryImpl;
use std::cell::RefCell;

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

pub const UPGRADES_MEMORY: MemoryId = MemoryId::new(0);
pub const INSURANCE_DATA_MEMORY: MemoryId = MemoryId::new(1);
pub const INIT_MEMORY: MemoryId = MemoryId::new(2);
pub const INSURANCE_TIMERS_MEMORY: MemoryId = MemoryId::new(3);
pub const INSURANCE_BUYERS_MEMORY: MemoryId = MemoryId::new(4);
pub const INSURANCE_SELLERS_MEMORY: MemoryId = MemoryId::new(5);
pub const INSURANCE_CONTRACT_EXECUTION_LOGS: MemoryId = MemoryId::new(6);
pub const USER_INSURANCE_LIST_HISTORY: MemoryId = MemoryId::new(7);
pub const INSURANCE_ACTIVE_CONTRACT_LIST: MemoryId = MemoryId::new(8);

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
