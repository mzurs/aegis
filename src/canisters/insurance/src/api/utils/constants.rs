use std::collections::BTreeMap;

use candid::Principal;

use crate::{api::interface::insurance::InsuranceAssets, mutate_state, read_state};

pub(crate) fn set_ledger_canister_id(key: InsuranceAssets, value: Principal) {
    mutate_state(|s| {
        s.heap_state.ledger_ids.insert(key, value);
    })
}

pub(crate) fn get_ledger_canister_id(key: InsuranceAssets) -> Principal {
    read_state(|s| *s.heap_state.ledger_ids.get(&key).unwrap())
}

pub(crate) fn init_ledger_ids() -> BTreeMap<InsuranceAssets, Principal> {
    let mut map: BTreeMap<InsuranceAssets, Principal> = BTreeMap::new();

    map.insert(
        InsuranceAssets::ICP,
        Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
    );
    map.insert(
        InsuranceAssets::CKBTC,
        Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai").unwrap(),
    );
    map.insert(
        InsuranceAssets::CKETH,
        Principal::from_text("ss2fx-dyaaa-aaaar-qacoq-cai").unwrap(),
    );

    map
}
