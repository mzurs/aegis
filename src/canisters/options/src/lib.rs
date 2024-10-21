use api::interfaces::state::State;
use canister_state_macro::canister_state;

pub mod api;
pub mod guard;
pub mod lifecycle;
pub mod memory;
pub mod updates;
pub mod queries;

canister_state!(State);

// type  a=Nat
// export_candid!();
