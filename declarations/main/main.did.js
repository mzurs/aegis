export const idlFactory = ({ IDL }) => {
  const BitcoinNetwork = IDL.Variant({
    'mainnet' : IDL.Null,
    'regtest' : IDL.Null,
    'testnet' : IDL.Null,
  });
  const InitArgs = IDL.Record({ 'bitcoin_network' : BitcoinNetwork });
  const IcrcAsset = IDL.Variant({
    'ICP' : IDL.Null,
    'AEGIS' : IDL.Null,
    'CKBTC' : IDL.Null,
    'CKETH' : IDL.Null,
  });
  const StakeAsset = IDL.Variant({
    'BTC' : IDL.Null,
    'ETH' : IDL.Null,
    'ICRC' : IcrcAsset,
  });
  const StakeTransactionType = IDL.Variant({
    'Stake' : IDL.Null,
    'ClaimRewards' : IDL.Null,
    'UnStake' : IDL.Null,
  });
  const StakeExecutionLogsKeys = IDL.Record({
    'asset_type' : StakeAsset,
    'transaction_type' : StakeTransactionType,
    'timestamp' : IDL.Nat64,
  });
  const StakeExecutionLogsValue = IDL.Record({ 'message' : IDL.Text });
  const TotalValueLockedRes = IDL.Variant({
    'BTC' : IDL.Nat64,
    'ETH' : IDL.Nat64,
    'ICRC' : IDL.Nat,
  });
  const StakeIcrcArgs = IDL.Record({
    'use_account' : IDL.Bool,
    'amount' : IDL.Nat,
  });
  const TransferFromError = IDL.Variant({
    'GenericError' : IDL.Record({
      'message' : IDL.Text,
      'error_code' : IDL.Nat,
    }),
    'TemporarilyUnavailable' : IDL.Null,
    'InsufficientAllowance' : IDL.Record({ 'allowance' : IDL.Nat }),
    'BadBurn' : IDL.Record({ 'min_burn_amount' : IDL.Nat }),
    'Duplicate' : IDL.Record({ 'duplicate_of' : IDL.Nat }),
    'BadFee' : IDL.Record({ 'expected_fee' : IDL.Nat }),
    'CreatedInFuture' : IDL.Record({ 'ledger_time' : IDL.Nat64 }),
    'TooOld' : IDL.Null,
    'InsufficientFunds' : IDL.Record({ 'balance' : IDL.Nat }),
  });
  const IcrcTransferFromResult = IDL.Variant({
    'TransferFromErrorMessage' : TransferFromError,
    'TransferFromSuccess' : IDL.Nat,
    'TransferFromErrorString' : IDL.Text,
  });
  const StakeIcrcRes = IDL.Variant({
    'ErrorMessage' : IDL.Text,
    'TransferError' : IcrcTransferFromResult,
    'Success' : IDL.Null,
  });
  const UnStakeIcrcArgs = IDL.Record({
    'to_account' : IDL.Bool,
    'amount' : IDL.Nat,
  });
  const TransferError = IDL.Variant({
    'GenericError' : IDL.Record({
      'message' : IDL.Text,
      'error_code' : IDL.Nat,
    }),
    'TemporarilyUnavailable' : IDL.Null,
    'BadBurn' : IDL.Record({ 'min_burn_amount' : IDL.Nat }),
    'Duplicate' : IDL.Record({ 'duplicate_of' : IDL.Nat }),
    'BadFee' : IDL.Record({ 'expected_fee' : IDL.Nat }),
    'CreatedInFuture' : IDL.Record({ 'ledger_time' : IDL.Nat64 }),
    'TooOld' : IDL.Null,
    'InsufficientFunds' : IDL.Record({ 'balance' : IDL.Nat }),
  });
  const ExecuteUnstakeAmountRes = IDL.Variant({
    'ErrorMessage' : IDL.Text,
    'TransferError' : TransferError,
    'Success' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text });
  return IDL.Service({
    'convert_u32_to_subaccount' : IDL.Func(
        [IDL.Nat32],
        [IDL.Vec(IDL.Nat8)],
        ['query'],
      ),
    'get_canister_id' : IDL.Func([IcrcAsset], [IDL.Principal], ['query']),
    'get_current_timestamp' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_min_stake_delay_' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_min_staking_delay' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_stake_execution_logs' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(StakeExecutionLogsKeys, StakeExecutionLogsValue))],
        ['query'],
      ),
    'get_staked_timestamp' : IDL.Func([IcrcAsset], [IDL.Nat64], ['query']),
    'get_total_value_locked' : IDL.Func(
        [StakeAsset],
        [TotalValueLockedRes],
        ['query'],
      ),
    'get_unstaked_timestamp' : IDL.Func([IcrcAsset], [IDL.Nat64], ['query']),
    'icrc_get_staked_amount_by_principal' : IDL.Func(
        [IcrcAsset],
        [IDL.Nat],
        ['query'],
      ),
    'icrc_stake_tokens' : IDL.Func(
        [IcrcAsset, StakeIcrcArgs],
        [StakeIcrcRes],
        [],
      ),
    'icrc_unstake_tokens' : IDL.Func(
        [IcrcAsset, UnStakeIcrcArgs],
        [StakeIcrcRes],
        [],
      ),
    'icrc_unstake_tokens_manual' : IDL.Func(
        [IcrcAsset],
        [ExecuteUnstakeAmountRes],
        [],
      ),
    'if_min_delay_over' : IDL.Func([IcrcAsset], [Result], ['query']),
    'principal_to_eth_address' : IDL.Func(
        [IDL.Principal],
        [IDL.Text],
        ['query'],
      ),
    'principal_to_subaccount' : IDL.Func(
        [IDL.Principal],
        [IDL.Vec(IDL.Nat8)],
        ['query'],
      ),
    'set_canister_id' : IDL.Func([IcrcAsset, IDL.Principal], [], []),
    'set_min_staking_delay' : IDL.Func([IDL.Opt(IDL.Nat64)], [IDL.Nat64], []),
    'set_rewards_duration' : IDL.Func([StakeAsset, IDL.Nat64], [], []),
  });
};
export const init = ({ IDL }) => {
  const BitcoinNetwork = IDL.Variant({
    'mainnet' : IDL.Null,
    'regtest' : IDL.Null,
    'testnet' : IDL.Null,
  });
  const InitArgs = IDL.Record({ 'bitcoin_network' : BitcoinNetwork });
  return [InitArgs];
};
