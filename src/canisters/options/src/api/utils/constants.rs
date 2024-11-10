use std::collections::BTreeMap;

use candid::Principal;

use crate::{api::interfaces::options_assets::OptionsAssetsIcrc, mutate_state, read_state};

pub(crate) fn set_icrc_ledger_canister_id(key: OptionsAssetsIcrc, value: Principal) {
    mutate_state(|s| {
        s.heap_state.icrc_ledger_ids.insert(key, value);
    })
}

pub(crate) fn get_icrc_ledger_canister_id(key: OptionsAssetsIcrc) -> Principal {
    read_state(|s| *s.heap_state.icrc_ledger_ids.get(&key).unwrap())
}

pub(crate) fn init_icrc_ledger_ids() -> BTreeMap<OptionsAssetsIcrc, Principal> {
    let mut map: BTreeMap<OptionsAssetsIcrc, Principal> = BTreeMap::new();

    map.insert(
        OptionsAssetsIcrc::ICP,
        Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
    );
    map.insert(
        OptionsAssetsIcrc::CKBTC,
        Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai").unwrap(),
    );
    map.insert(
        OptionsAssetsIcrc::CKETH,
        Principal::from_text("ss2fx-dyaaa-aaaar-qacoq-cai").unwrap(),
    );

    map
}
