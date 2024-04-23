use candid::{CandidType, Principal};
use serde::Deserialize;

#[derive(CandidType, Deserialize, Clone)]
pub struct Constants {
    pub ledger_ids: LedgerIds,
    pub minter_ids: MinterIds,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct LedgerIds {
    pub ckbtc_ledger_id: Principal,
    pub cketh_ledger_id: Principal,
    pub icp_ledger_id: Principal,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct MinterIds {
    pub ckbtc_minter_id: Principal,
    pub cketh_minter_id: Principal,
}
