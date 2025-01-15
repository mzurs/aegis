use candid::Principal;

use crate::{api::interfaces::options_assets::OptionsAssetsIcrc, read_state};

pub fn fetch_icrc_ledger_ids(args: OptionsAssetsIcrc) -> Principal {
    return match read_state(|s| s.heap_state.icrc_ledger_ids.get(&args).cloned()) {
        Some(id) => id.clone(),
        None => panic!("No Ledger Id Found"),
    };
}
