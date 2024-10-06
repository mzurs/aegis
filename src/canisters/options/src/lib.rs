pub mod api;
pub mod guard;
pub mod lifecycle;
pub mod memory;
pub mod queries;
pub mod updates;

canister_state!(State);

// type  a=Nat
export_candid!();
