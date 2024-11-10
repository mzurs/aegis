import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface CreateOptionArgs {
  'asset' : OptionsAssets,
  'options_type' : OptionsType,
  'contract_state' : OptionsContractState,
  'asset_amount' : bigint,
  'contract_expiry' : bigint,
}
export interface Options {
  'asset' : OptionsAssets,
  'options_type' : OptionsType,
  'contract_state' : OptionsContractState,
  'seller' : Principal,
  'asset_amount' : bigint,
  'timestamp' : bigint,
  'contract_expiry' : bigint,
  'buyer' : [] | [Principal],
}
export interface OptionsActiveListKey {
  'id' : bigint,
  'options_asset' : OptionsAssetsByNames,
  'options_type' : OptionsType,
  'timestamp' : bigint,
}
export type OptionsAssets = { 'BTC' : null } |
  { 'ETH' : null } |
  { 'ICRC' : OptionsAssetsIcrc };
export type OptionsAssetsByNames = { 'BTC' : null } |
  { 'ETH' : null } |
  { 'ICP' : null } |
  { 'CKBTC' : null } |
  { 'CKETH' : null };
export type OptionsAssetsIcrc = { 'ICP' : null } |
  { 'CKBTC' : null } |
  { 'CKETH' : null };
export type OptionsContractState = { 'EXECUTED' : null } |
  { 'OPEN' : null } |
  { 'EXPIRED' : null } |
  { 'CLOSED' : null };
export type OptionsType = { 'PUT' : null } |
  { 'CALL' : null };
export type Result = { 'Ok' : string } |
  { 'Err' : string };
export interface _SERVICE {
  'create_icrc_options' : ActorMethod<
    [OptionsAssetsIcrc, CreateOptionArgs],
    Result
  >,
  'get_all_options' : ActorMethod<[], Array<Options>>,
  'get_all_options_ids' : ActorMethod<[], BigUint64Array | bigint[]>,
  'get_call_options_by_asset' : ActorMethod<
    [OptionsAssetsByNames],
    Array<[OptionsActiveListKey, null]>
  >,
  'get_ledger_canister_id' : ActorMethod<[OptionsAssetsIcrc], Principal>,
  'get_put_options_by_asset' : ActorMethod<
    [OptionsAssetsByNames],
    Array<[OptionsActiveListKey, null]>
  >,
  'set_ledger_canister_id' : ActorMethod<
    [OptionsAssetsIcrc, Principal],
    undefined
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
