use ic_cdk::init;

use crate::{
    api::interface::{
        lifecycle::InsuranceInitArgs,
        state::{StableStates, State},
    },
    init_state, mutate_state,
};

#[init]
fn init(args: InsuranceInitArgs) {
    let _ = args;
    init_state(State::default());

    mutate_state(|s| {
        let _res: &mut StableStates = &mut s.stable_state;
    });
}
