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

    // mutate_state(|s| {
    //     {
    //         *s = State {
    //             stable_state: init_stable_states(),
    //             heap_state: init_heap_state(),
    //         }
    //     }
    // });

    mutate_state(|s| {
        let _res: &mut StableStates = &mut s.stable_state;
        //     let _ = res.init.set(args);
    });
}
