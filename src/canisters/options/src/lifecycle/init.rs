use ic_cdk::init;

use crate::{
    api::interfaces::{
        lifecycle::InitArgs,
        state::{StableStates, State},
    },
    init_state, mutate_state,
};

#[init]
fn init(_args: InitArgs) {
    ic_cdk::println!("Init Start ");

    init_state(State::default());

    mutate_state(|s| {
        let _res: &mut StableStates = &mut s.stable_state;
        // let _ = res.init.set(args);
    });

    ic_cdk::println!("Init End ");
}

impl Default for InitArgs {
    fn default() -> Self {
        Self {}
    }
}
