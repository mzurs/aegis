import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type BitcoinNetwork = { 'mainnet' : null } |
  { 'regtest' : null } |
  { 'testnet' : null };
export type ExecuteUnstakeAmountRes = { 'ErrorMessage' : string } |
  { 'TransferError' : TransferError } |
  { 'Success' : null };
export type IcrcAsset = { 'ICP' : null } |
  { 'AEGIS' : null } |
  { 'CKBTC' : null } |
  { 'CKETH' : null };
export type IcrcTransferFromResult = {
    'TransferFromErrorMessage' : TransferFromError
  } |
  { 'TransferFromSuccess' : bigint } |
  { 'TransferFromErrorString' : string };
export interface InitArgs { 'bitcoin_network' : BitcoinNetwork }
export type Result = { 'Ok' : string } |
  { 'Err' : string };
export type StakeAsset = { 'BTC' : null } |
  { 'ETH' : null } |
  { 'ICRC' : IcrcAsset };
export interface StakeExecutionLogsKeys {
  'asset_type' : StakeAsset,
  'transaction_type' : StakeTransactionType,
  'timestamp' : bigint,
}
export interface StakeExecutionLogsValue { 'message' : string }
export interface StakeIcrcArgs { 'use_account' : boolean, 'amount' : bigint }
export type StakeIcrcRes = { 'ErrorMessage' : string } |
  { 'TransferError' : IcrcTransferFromResult } |
  { 'Success' : null };
export type StakeTransactionType = { 'Stake' : null } |
  { 'ClaimRewards' : null } |
  { 'UnStake' : null };
export type TotalValueLockedRes = { 'BTC' : bigint } |
  { 'ETH' : bigint } |
  { 'ICRC' : bigint };
export type TransferError = {
    'GenericError' : { 'message' : string, 'error_code' : bigint }
  } |
  { 'TemporarilyUnavailable' : null } |
  { 'BadBurn' : { 'min_burn_amount' : bigint } } |
  { 'Duplicate' : { 'duplicate_of' : bigint } } |
  { 'BadFee' : { 'expected_fee' : bigint } } |
  { 'CreatedInFuture' : { 'ledger_time' : bigint } } |
  { 'TooOld' : null } |
  { 'InsufficientFunds' : { 'balance' : bigint } };
export type TransferFromError = {
    'GenericError' : { 'message' : string, 'error_code' : bigint }
  } |
  { 'TemporarilyUnavailable' : null } |
  { 'InsufficientAllowance' : { 'allowance' : bigint } } |
  { 'BadBurn' : { 'min_burn_amount' : bigint } } |
  { 'Duplicate' : { 'duplicate_of' : bigint } } |
  { 'BadFee' : { 'expected_fee' : bigint } } |
  { 'CreatedInFuture' : { 'ledger_time' : bigint } } |
  { 'TooOld' : null } |
  { 'InsufficientFunds' : { 'balance' : bigint } };
export interface UnStakeIcrcArgs { 'to_account' : boolean, 'amount' : bigint }
export interface _SERVICE {
  'convert_u32_to_subaccount' : ActorMethod<[number], Uint8Array | number[]>,
  'get_canister_id' : ActorMethod<[IcrcAsset], Principal>,
  'get_current_timestamp' : ActorMethod<[], bigint>,
  'get_min_stake_delay_' : ActorMethod<[], bigint>,
  'get_min_staking_delay' : ActorMethod<[], bigint>,
  'get_stake_execution_logs' : ActorMethod<
    [],
    Array<[StakeExecutionLogsKeys, StakeExecutionLogsValue]>
  >,
  'get_staked_timestamp' : ActorMethod<[IcrcAsset], bigint>,
  'get_total_value_locked' : ActorMethod<[StakeAsset], TotalValueLockedRes>,
  'get_unstaked_timestamp' : ActorMethod<[IcrcAsset], bigint>,
  'icrc_get_staked_amount_by_principal' : ActorMethod<[IcrcAsset], bigint>,
  'icrc_stake_tokens' : ActorMethod<[IcrcAsset, StakeIcrcArgs], StakeIcrcRes>,
  'icrc_unstake_tokens' : ActorMethod<
    [IcrcAsset, UnStakeIcrcArgs],
    StakeIcrcRes
  >,
  'icrc_unstake_tokens_manual' : ActorMethod<
    [IcrcAsset],
    ExecuteUnstakeAmountRes
  >,
  'if_min_delay_over' : ActorMethod<[IcrcAsset], Result>,
  'principal_to_eth_address' : ActorMethod<[Principal], string>,
  'principal_to_subaccount' : ActorMethod<[Principal], Uint8Array | number[]>,
  'set_canister_id' : ActorMethod<[IcrcAsset, Principal], undefined>,
  'set_min_staking_delay' : ActorMethod<[[] | [bigint]], bigint>,
  'set_rewards_duration' : ActorMethod<[StakeAsset, bigint], undefined>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
