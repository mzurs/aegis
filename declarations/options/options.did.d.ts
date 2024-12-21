import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type CanisterName = { 'ExchangeRate' : null };
export interface CreateOptionArgs {
  'asset' : OptionsAssets,
  'strike_price' : bigint,
  'options_type' : OptionsType,
  'contract_state' : OptionsContractState,
  'offer_duration' : bigint,
  'asset_amount' : bigint,
  'use_exchange_account' : boolean,
  'contract_expiry' : bigint,
}
export interface HttpHeader { 'value' : string, 'name' : string }
export interface HttpResponse {
  'status' : bigint,
  'body' : Uint8Array | number[],
  'headers' : Array<HttpHeader>,
}
export interface Options {
  'asset' : OptionsAssets,
  'strike_price' : bigint,
  'options_type' : OptionsType,
  'contract_state' : OptionsContractState,
  'offer_duration' : bigint,
  'name' : string,
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
  'offer_duration' : bigint,
  'timestamp' : bigint,
}
export type OptionsAssets = { 'BTC' : null } |
  { 'ETH' : null } |
  { 'ICRC' : OptionsAssetsIcrc };
export type OptionsAssetsByNames = { 'BTC' : null } |
  { 'ETH' : null } |
  { 'ICP' : null } |
  { 'USDT' : null } |
  { 'CKBTC' : null } |
  { 'CKETH' : null };
export type OptionsAssetsIcrc = { 'ICP' : null } |
  { 'CKUSDT' : null } |
  { 'CKBTC' : null } |
  { 'CKETH' : null };
export type OptionsContractState = { 'EXECUTED' : null } |
  { 'OPEN' : null } |
  { 'EXPIRED' : null } |
  { 'OFFER' : null } |
  { 'CLOSED' : null };
export type OptionsType = { 'PUT' : null } |
  { 'CALL' : null };
export type Result = { 'Ok' : number } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : string } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : null } |
  { 'Err' : string };
export type Result_3 = { 'Ok' : bigint } |
  { 'Err' : string };
export interface TradedOptionsContractsKey {
  'id' : bigint,
  'principal' : Principal,
  'contract_state' : string,
  'timestamp' : bigint,
}
export interface TradedOptionsContractsValue {
  'trade_timestamp' : bigint,
  'options_name' : string,
  'options_type' : string,
}
export interface TransformArgs {
  'context' : Uint8Array | number[],
  'response' : HttpResponse,
}
export interface _SERVICE {
  'calculate_premium' : ActorMethod<
    [bigint, OptionsType, bigint, OptionsAssets],
    Result
  >,
  'convert_asset_amount_to_human' : ActorMethod<
    [OptionsAssets, bigint],
    number
  >,
  'convert_asset_amount_to_non_human' : ActorMethod<
    [OptionsAssets, number],
    bigint
  >,
  'convert_premium_amount_to_non_humans' : ActorMethod<
    [OptionsAssets, number],
    bigint
  >,
  'convert_xrc_human_to_non_humans' : ActorMethod<[number], bigint>,
  'convert_xrc_non_human_to_human' : ActorMethod<[bigint], number>,
  'create_icrc_options' : ActorMethod<
    [OptionsAssetsIcrc, CreateOptionArgs],
    Result_1
  >,
  'execute_manual' : ActorMethod<[OptionsAssetsIcrc, bigint], Result_2>,
  'get_all_options' : ActorMethod<[], Array<Options>>,
  'get_all_options_ids' : ActorMethod<[], BigUint64Array | bigint[]>,
  'get_call_options_by_asset' : ActorMethod<
    [OptionsAssetsByNames],
    Array<[OptionsActiveListKey, null]>
  >,
  'get_canister_id' : ActorMethod<[CanisterName], Principal>,
  'get_exchange_rate' : ActorMethod<[OptionsAssets], Result_3>,
  'get_ledger_canister_id' : ActorMethod<[OptionsAssetsIcrc], Principal>,
  'get_options_trade_history_by_principal' : ActorMethod<
    [OptionsContractState],
    Array<[TradedOptionsContractsKey, TradedOptionsContractsValue]>
  >,
  'get_put_options_by_asset' : ActorMethod<
    [OptionsAssetsByNames],
    Array<[OptionsActiveListKey, null]>
  >,
  'remaining_time_in_years' : ActorMethod<[bigint], number>,
  'set_canister_id' : ActorMethod<[CanisterName, Principal], undefined>,
  'set_ledger_canister_id' : ActorMethod<
    [OptionsAssetsIcrc, Principal],
    undefined
  >,
  'trade_icrc_options' : ActorMethod<[OptionsAssetsIcrc, bigint], Result_1>,
  'transform_fred' : ActorMethod<[TransformArgs], HttpResponse>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
