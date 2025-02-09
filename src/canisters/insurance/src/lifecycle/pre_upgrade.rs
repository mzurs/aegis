use ic_cdk::pre_upgrade;
use stable_memory::get_writer;
use tracing::info;

use crate::{memory::get_memory_upgrades, take_state};

#[pre_upgrade]
fn pre_upgrade() {
    info!("Pre-upgrade starting for Insruance Canister ");

    let state = take_state();

    let stable_state = &state;

    let mut memory = get_memory_upgrades();
    let writer = get_writer(&mut memory);

    serializer::serialize(stable_state, writer).unwrap();
}
