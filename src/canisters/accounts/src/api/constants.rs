use std::collections::BTreeMap;

use candid::Principal;

use crate::{mutate_state, read_state};

use super::interfaces::constants::CanisterName;

pub(crate) fn set_canister_id(key: CanisterName, value: Principal) {
    mutate_state(|s| {
        s.heap_state.canister_ids.insert(key, value);
    })
}

pub(crate) fn get_canister_id(key: CanisterName) -> Principal {
    read_state(|s| *s.heap_state.canister_ids.get(&key).unwrap())
}

pub(crate) fn init_canister_ids() -> BTreeMap<CanisterName, Principal> {
    let mut map: BTreeMap<CanisterName, Principal> = BTreeMap::new();

    map.insert(
        CanisterName::ICP,
        Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
    );
    map.insert(
        CanisterName::CKBTC,
        Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai").unwrap(),
    );
    map.insert(
        CanisterName::CKETH,
        Principal::from_text("ss2fx-dyaaa-aaaar-qacoq-cai").unwrap(),
    );
    map.insert(
        CanisterName::CKBTCMINTER,
        Principal::from_text("mqygn-kiaaa-aaaar-qaadq-cai").unwrap(),
    );
    map.insert(
        CanisterName::CKETHMINTER,
        Principal::from_text("sv3dd-oaaaa-aaaar-qacoa-cai").unwrap(),
    );

    map
}
