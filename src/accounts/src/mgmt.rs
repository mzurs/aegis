use candid::Principal;
use ic_cdk::{
    api::{
        call::{call_with_payment, CallResult},
        management_canister::bitcoin::{GetBalanceRequest, Satoshi},
    },
    call,
};

use crate::utils;

pub struct MgmtCanister(pub Principal);

impl Default for MgmtCanister {
    fn default() -> Self {
        Self(Principal::management_canister())
    }
}

impl MgmtCanister {
    /// Returns the Management Canister Instance
    pub fn new() -> Self {
        Self::default()
    }

    //  Get the bitcoin balance for a given address
    pub async fn get_balance(&self, address: String, replicated_state: bool) -> u64 {
        if !replicated_state {
            let balance_res: CallResult<(Satoshi,)> = call(
                self.0,
                "bitcoin_get_balance_query",
                (GetBalanceRequest {
                    address,
                    network: utils::get_bitcoin_network(),
                    min_confirmations: None,
                },),
            )
            .await;

            return balance_res.unwrap().0;
        }

        const GET_BALANCE_COST_CYCLES: u64 = 100_000_000;
        // const GET_UTXOS_COST_CYCLES: u64 = 10_000_000_000;
        // const GET_CURRENT_FEE_PERCENTILES_CYCLES: u64 = 100_000_000;
        // const SEND_TRANSACTION_BASE_CYCLES: u64 = 5_000_000_000;
        // const SEND_TRANSACTION_PER_BYTE_CYCLES: u64 = 20_000_000;
        let balance_res: Result<(Satoshi,), _> = call_with_payment(
            self.0,
            "bitcoin_get_balance",
            (GetBalanceRequest {
                address,
                network: utils::get_bitcoin_network(),
                min_confirmations: None,
            },),
            GET_BALANCE_COST_CYCLES,
        )
        .await;

        balance_res.unwrap().0
    }
}
