use ic_cdk::init;

use crate::{
    api::{
        interfaces::state::{StableStates, State},
        lifecycle::init::InitArgs,
    },
    init_state, mutate_state,
};

#[init]
fn init(args: InitArgs) {
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
        let res: &mut StableStates = &mut s.stable_state;
        let _ = res.init.set(args);
    });
}
