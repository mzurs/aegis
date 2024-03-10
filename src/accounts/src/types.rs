pub mod memory {
    use ic_stable_structures::memory_manager::VirtualMemory;
    use ic_stable_structures::DefaultMemoryImpl;

    pub type Memory = VirtualMemory<DefaultMemoryImpl>;
}

pub mod states {

    use candid::{CandidType, Principal};
    use serde::Deserialize;

    pub mod initialization_configs {
        use candid::CandidType;
        use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
        use serde::Deserialize;

        /// Accounts Canister Intialization Arguments
        #[derive(CandidType, Deserialize)]
        pub struct InitArgs {
            pub bitcoin_network: BitcoinNetwork,
        }
    }
    /**
     The Account struct for a user. It holds necessary information of Account and their derived types
    */
    #[derive(CandidType, Deserialize)]
    pub struct Account {
        pub user_id: u64,
        pub principal: Principal,
        pub user_name: Option<String>,
    }

    #[derive(CandidType, Deserialize)]
    pub struct AccountMetrics {
        pub user_counts: u64,
        pub active_users: u64,
    }

    #[derive(CandidType, Deserialize, Clone)]
    pub struct LedgerIds {
        pub ckbtc_ledger_id: Principal,
        pub cketh_ledger_id: Principal,
        pub icp_ledger_id: Principal,
    }

    #[derive(CandidType, Deserialize, Clone)]
    pub struct MinterIds {
        pub ckbtc_minter_id: Principal,
        pub cketh_minter_id: Principal,
    }

    #[derive(CandidType, Deserialize, Clone)]
    pub struct Constants {
        pub ledger_ids: LedgerIds,
        pub minter_ids: MinterIds,
    }
}

pub mod interfaces {

    pub mod account {
        use candid::{CandidType, Principal};

        #[derive(CandidType, Debug)]
        pub struct AccountAddresses {
            pub icrc: Principal,
            pub eth: String,
            pub btc: String,
        }
    }
}

pub mod types_impls {

    use candid::{Decode, Encode, Principal};
    use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
    use ic_stable_structures::{storable::Bound, Storable};
    use std::borrow::Cow;

    use super::states::{
        initialization_configs::InitArgs, Account, AccountMetrics, Constants, LedgerIds, MinterIds,
    };

    impl Storable for Account {
        fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
            Cow::Owned(Encode!(self).unwrap())
        }

        fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
            Decode!(bytes.as_ref(), Self).unwrap()
        }

        const BOUND: Bound = Bound::Unbounded;
    }

    impl Default for AccountMetrics {
        fn default() -> Self {
            Self {
                user_counts: 0,
                active_users: 1,
            }
        }
    }

    impl Storable for AccountMetrics {
        fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
            Cow::Owned(Encode!(self).unwrap())
        }

        fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
            Decode!(bytes.as_ref(), Self).unwrap()
        }

        const BOUND: Bound = Bound::Unbounded;
    }

    impl Storable for Constants {
        fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
            Cow::Owned(Encode!(self).unwrap())
        }

        fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
            Decode!(bytes.as_ref(), Self).unwrap()
        }

        const BOUND: Bound = Bound::Unbounded;
    }

    impl Default for Constants {
        fn default() -> Self {
            Self {
                ledger_ids: LedgerIds {
                    ckbtc_ledger_id: Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai").unwrap(),
                    cketh_ledger_id: Principal::from_text("ss2fx-dyaaa-aaaar-qacoq-cai").unwrap(),
                    icp_ledger_id: Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
                },
                minter_ids: MinterIds {
                    ckbtc_minter_id: Principal::from_text("mqygn-kiaaa-aaaar-qaadq-cai").unwrap(),
                    cketh_minter_id: Principal::from_text("sv3dd-oaaaa-aaaar-qacoa-cai").unwrap(),
                },
            }
        }
    }

    impl Storable for InitArgs {
        fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
            Cow::Owned(Encode!(self).unwrap())
        }

        fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
            Decode!(bytes.as_ref(), Self).unwrap()
        }

        const BOUND: Bound = Bound::Unbounded;
    }

    impl Default for InitArgs {
        fn default() -> Self {
            Self {
                bitcoin_network: BitcoinNetwork::Regtest,
            }
        }
    }


impl Constants {
    
}
}
