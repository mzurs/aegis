use byteorder::{BigEndian, ByteOrder};
use candid::Principal;
use ic_cdk::api::{call::RejectionCode, management_canister::bitcoin::BitcoinNetwork};

use crate::{
    enums::{Metric, MetricValues},
    memory::STATE,
    types::states::{AccountMetrics, StableStates},
};

// use ic_cdk::api::call::call_with_payment;
use icrc_ledger_types::icrc1::account::Subaccount;

// use super::{
//     constants::{
//         get_xrc_canister_id,
//         ledger_contracts::{get_ckbtc_ledger, get_cketh_ledger, get_icp_ledger},
//     },
//     enums::xrc::enum_exchange_rate_error,
//     types::{
//         interfaces::{Asset as Assets, Icrcs},
//         other_canister::{
//             Asset, AssetClass, ExchangeRate, GetExchangeRateRequest, GetExchangeRateResult,
//         },
//     },
// };
// use crate::canister_utils::types::interfaces::Asset::{Bitcoin, Ethereum, ICRC};

pub fn _principal_to_subaccount(principal_id: &Principal) -> [u8; 32] {
    let mut subaccount: [u8; 32] = [0; std::mem::size_of::<Subaccount>()];
    let principal_id = principal_id.as_slice();
    subaccount[0] = principal_id.len().try_into().unwrap();
    subaccount[1..1 + principal_id.len()].copy_from_slice(principal_id);

    // let byte_buf: ByteBuf = ByteBuf::from(subaccount);
    // byte_buf
    subaccount
}

pub fn convert_u64_to_subaccount(num: u64) -> [u8; 32] {
    let mut network_bytes: [u8; 32] = [0; 32];
    network_bytes[..8].copy_from_slice(&num.to_ne_bytes());

    // Little-endian byte order
    let mut little_endian_bytes: [u8; 32] = [0; 32];
    little_endian_bytes[..8].copy_from_slice(&num.to_le_bytes());
    little_endian_bytes
}

// pub async fn _get_exchange_rate(asset: Assets) -> Result<ExchangeRate, String> {
//     let base_asset: String = _get_asset_str(asset);

//     let quote_asset = String::from("USDT");
//     let xrc_canister_id: Principal = get_xrc_canister_id();
//     let xrc_canister_cycles_cost = 1_000_000_000;

//     let xrc_args: GetExchangeRateRequest = GetExchangeRateRequest {
//         timestamp: Option::None,
//         quote_asset: Asset {
//             class: AssetClass::Cryptocurrency,
//             symbol: quote_asset,
//         },
//         base_asset: Asset {
//             class: AssetClass::Cryptocurrency,
//             symbol: base_asset,
//         },
//     };
//     let (res): (GetExchangeRateResult,) = call_with_payment(
//         xrc_canister_id,
//         "get_exchange_rate",
//         (xrc_args,),
//         xrc_canister_cycles_cost,
//     )
//     .await
//     .unwrap();

//     let rate = match res {
//         (GetExchangeRateResult::Ok(result),) => result,
//         (GetExchangeRateResult::Err(err),) => return Err(enum_exchange_rate_error(err)),
//     };
//     Ok(rate)
// }
/**
    @title: Implementation of Random Number Generator using Management Canister
*/
pub async fn _generate_random_number() -> Result<u64, String> {
    let random_bytes: Result<(Vec<u8>,), (RejectionCode, String)> =
        ic_cdk::api::management_canister::main::raw_rand().await;

    let random_number: u64 = match random_bytes {
        Ok(rand_bytes) => BigEndian::read_u64(rand_bytes.0.as_slice()),
        Err(err) => return Err(err.1),
    };

    Ok(random_number)
}

/// Increment the total no of user account by one
pub fn increment_user_count() -> () {
    STATE.with_borrow_mut(|m| {
        let state: &mut StableStates = &mut m.stable_state;

        let get_metrics: &AccountMetrics = state.account_metrics.get();

        let _ = state.account_metrics.set(AccountMetrics {
            // ..m
            user_counts: get_metrics.user_counts + 1,
            ..*get_metrics
        });
    });
    ()
}

pub fn get_metrics(metric: Metric) -> MetricValues {
    match metric {
        Metric::UserCounts => MetricValues::UserCounts(STATE.with_borrow(|m| {
            let state: &StableStates = &m.stable_state;

            state.account_metrics.get().user_counts
        })),
        Metric::ActiveUsers => MetricValues::ActiveUsers(0),
    }
}

/// Get the current Bitcoin Network used by Account Canister
pub fn get_bitcoin_network() -> BitcoinNetwork {
    STATE.with_borrow(|n| {
        let state: &StableStates = &n.stable_state;

        state.init.get().bitcoin_network
    })
}

// fn usage() {
//     eprintln!(
//         "USAGE: {} PRINCIPAL",
//         std::env::current_exe().unwrap().display()
//     );
// }

pub fn principal_to_hex(principal: Principal) -> String {
    let principal = principal;

    let n = principal.as_slice().len();
    assert!(n <= 29);
    let mut fixed_bytes = [0u8; 32];
    fixed_bytes[0] = n as u8;
    fixed_bytes[1..=n].copy_from_slice(principal.as_slice());
    format!("0x{}", hex::encode(fixed_bytes))
}
