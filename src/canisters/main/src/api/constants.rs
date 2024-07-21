use std::collections::BTreeMap;

use candid::Principal;

use crate::{mutate_state, read_state};

use super::interfaces::constants::IcrcAsset;

pub(crate) fn set_ledger_canister_id(key: IcrcAsset, value: Principal) {
    mutate_state(|s| {
        s.heap_state.ledger_ids.insert(key, value);
    })
}

pub(crate) fn get_ledger_canister_id(key: IcrcAsset) -> Principal {
    read_state(|s| *s.heap_state.ledger_ids.get(&key).unwrap())
}

pub(crate) fn init_ledger_ids() -> BTreeMap<IcrcAsset, Principal> {
    let mut map: BTreeMap<IcrcAsset, Principal> = BTreeMap::new();

    map.insert(IcrcAsset::ICP, Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap());
    map.insert(IcrcAsset::CKBTC, Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai").unwrap());
    map.insert(IcrcAsset::CKETH, Principal::from_text("ss2fx-dyaaa-aaaar-qacoq-cai").unwrap());
    map.insert(IcrcAsset::AEGIS, Principal::from_text("2jymc-fyaaa-aaaar-qad2q-cai").unwrap());

    map
}
