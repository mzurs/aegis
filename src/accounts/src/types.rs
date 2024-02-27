pub mod memory {
    use ic_stable_structures::memory_manager::VirtualMemory;
    use ic_stable_structures::DefaultMemoryImpl;

    pub type Memory = VirtualMemory<DefaultMemoryImpl>;
}

pub mod states {

    use candid::{CandidType, Principal};
    use serde::Deserialize;

    /**
     The Account struct for a user. It holds necessary information of Account and their derived types
    */
    #[derive(CandidType, Clone, Deserialize)]
    pub struct Account {
        pub user_id: u64,
        pub principal: Principal,
        pub user_name: Option<String>,
    }

    #[derive(CandidType, Clone, Deserialize)]

    pub struct AccountMetrics {
        pub user_counts: u64,
        pub active_users: u64,
    }
}

pub mod interfaces {
    pub mod account {
        use candid::{CandidType, Principal};

        #[derive(CandidType, Clone, Debug)]
        pub struct AccountAddresses {
            pub icrc: Principal,
            pub eth: String,
            pub btc: String,
        }
    }
}
