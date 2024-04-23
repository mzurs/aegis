use std::borrow::Cow;

use candid::{Decode, Encode, Principal};
use ic_stable_structures::{storable::Bound, Storable};

use crate::mutate_state;

use super::interfaces::{
    constants::{Constants, LedgerIds, MinterIds},
    state::StableStates,
};

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

impl Constants {
    pub fn set_ledger_ids(ids: LedgerIds) -> () {
        let _ = mutate_state(|c| {
            let state: &mut StableStates = &mut c.stable_state;

            let constants: &Constants = state.constants.get();
            state.constants.set(Constants {
                ledger_ids: ids,
                minter_ids: MinterIds { ..constants.minter_ids },
            })
        });
    }

    pub fn set_minter_ids(ids: MinterIds) -> () {
        let _ = mutate_state(|s| {
            let state: &mut StableStates = &mut s.stable_state;

            let constants: &Constants = state.constants.get();

            state.constants.set(Constants {
                ledger_ids: LedgerIds { ..constants.ledger_ids },
                minter_ids: ids,
            })
        });
    }
}
