use ic_cdk::post_upgrade;
use stable_memory::get_reader;

use crate::{api::interfaces::state::State, init_state, memory::get_memory_upgrades};

#[post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("uogrding");

    let memory = get_memory_upgrades();
    let reader = get_reader(&memory);

    let data: State = serializer::deserialize(reader).unwrap();

    init_state(data);
}
