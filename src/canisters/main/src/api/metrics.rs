use super::interfaces::{constants::StakeAsset, icrc_stake::StakeIcrc, metrics::TotalValueLockedRes};

pub(crate) fn get_total_value_locked(asset: StakeAsset) -> TotalValueLockedRes {
    match asset {
        StakeAsset::ICRC(icrc) => TotalValueLockedRes::ICRC(StakeIcrc::get_staked_tvl(icrc).0),
        StakeAsset::BTC => TotalValueLockedRes::BTC(0),
        StakeAsset::ETH => TotalValueLockedRes::ETH(0),
    }
}
