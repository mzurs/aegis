use crate::{api::interface::insurance::Insurance, read_state};

pub fn rewrite_contract_expiry() {
    read_state(|s| {
        s.stable_state.insurance_contract_timer.iter().for_each(|k| {
            let contract_expiry: u64 = k.0;
            let insurance_id: u32 = k.1;

            let _ = Insurance::set_contract_timer(contract_expiry, insurance_id);
        })
    })
}
