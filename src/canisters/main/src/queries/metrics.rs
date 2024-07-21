use ic_cdk::query;

use crate::api::{
    interfaces::{constants::StakeAsset, metrics::TotalValueLockedRes},
    metrics,
};

#[query]
pub(crate) fn get_total_value_locked(asset: StakeAsset) -> TotalValueLockedRes {
    metrics::get_total_value_locked(asset)
}
