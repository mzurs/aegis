use ic_cdk::post_upgrade;
use stable_memory::get_reader;

use crate::{
    api::{interface::state::State, utils::rewrite_set_timer::rewrite_contract_expiry},
    init_state,
    memory::get_memory_upgrades,
};

#[post_upgrade]
fn post_upgrade() {
    let memory = get_memory_upgrades();
    let reader = get_reader(&memory);

    let data: State = serializer::deserialize(reader).unwrap();

    init_state(data);

    rewrite_contract_expiry()

    // info!(version = %args.wasm_version, "Post-upgrade complete");
}
