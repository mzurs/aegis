import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface BuyInsuranceArgs {
  'premium' : bigint,
  'insurance_id' : number,
}
export type BuyInsuranceRes = { 'ErrorMessage' : string } |
  { 'TransferError' : IcrcTransferFromResult } |
  { 'Success' : null };
export type Country = { 'UK' : null } |
  { 'US' : null };
export type ExecuteInsuranceContractRes = { 'ErrorMessage' : string } |
  { 'TransferError' : IcrcTransferResult } |
  { 'Success' : null };
export interface HttpHeader { 'value' : string, 'name' : string }
export interface HttpResponse {
  'status' : bigint,
  'body' : Uint8Array | number[],
  'headers' : Array<HttpHeader>,
}
export type IcrcTransferFromResult = {
    'TransferFromErrorMessage' : TransferFromError
  } |
  { 'TransferFromSuccess' : bigint } |
  { 'TransferFromErrorString' : string };
export type IcrcTransferResult = { 'TransferErrorString' : string } |
  { 'TransferErrorMessage' : TransferError } |
  { 'TransferSuccess' : bigint };
export interface InflationBasedInsurance {
  'country' : Country,
  'target_expiry' : bigint,
  'inflation_target' : number,
}
export interface Insurance {
  'multiplier' : InsuranceRewardsMultiplier,
  'status' : InsuranceContractStatus,
  'title' : string,
  'last_pool_balance' : bigint,
  'min_premium_amount' : bigint,
  'description' : string,
  'is_muliple_seller_allowed' : boolean,
  'issuer' : Principal,
  'expiry_date' : bigint,
  'category' : InsuranceCategory,
  'insurance_asset' : InsuranceAssets,
  'last_executed_time' : bigint,
  'last_premium_balance' : bigint,
  'min_share_amount' : [] | [bigint],
}
export interface InsuranceActiveListKey {
  'principal' : Principal,
  'insurance_id' : number,
}
export type InsuranceAssets = { 'ICP' : null } |
  { 'CKBTC' : null } |
  { 'CKETH' : null };
export interface InsuranceBuyersKey {
  'principal' : Principal,
  'insurance_id' : number,
  'time_stamp' : bigint,
}
export type InsuranceCategory = {
    'InflationBasedInsurance' : InflationBasedInsurance
  };
export interface InsuranceContractExecutionLogsKeys {
  'insurance_id' : number,
  'message' : string,
  'timestamp' : bigint,
}
export interface InsuranceContractInitArgs {
  'multiplier' : InsuranceRewardsMultiplier,
  'title' : string,
  'min_premium_amount' : bigint,
  'description' : string,
  'is_muliple_seller_allowed' : boolean,
  'expiry_date' : bigint,
  'category' : InsuranceCategory,
  'insurance_asset' : InsuranceAssets,
  'amount' : bigint,
  'min_share_amount' : [] | [bigint],
}
export type InsuranceContractStatus = { 'OPEN' : null } |
  { 'CLOSED' : null };
export type InsuranceInitRes = { 'ErrorMessage' : string } |
  { 'TransferError' : IcrcTransferFromResult } |
  { 'Success' : number };
export type InsuranceRewardsMultiplier = { 'M2X' : null } |
  { 'M3X' : null } |
  { 'M4X' : null };
export type Result = { 'Ok' : number } |
  { 'Err' : string };
export interface SellInsuranceArgs {
  'insurance_id' : number,
  'amount' : bigint,
}
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
export interface TransformArgs {
  'context' : Uint8Array | number[],
  'response' : HttpResponse,
}
export interface UserInsuranceListHistoryKey {
  'principal' : Principal,
  'insurance_id' : number,
}
export interface _SERVICE {
  'buy_insurance_contract' : ActorMethod<[BuyInsuranceArgs], BuyInsuranceRes>,
  'calculate_buy_insurance_contract_premium' : ActorMethod<[number], bigint>,
  'convert_u64_to_date' : ActorMethod<[[] | [bigint]], string>,
  'create_insurance_contract' : ActorMethod<
    [InsuranceContractInitArgs],
    InsuranceInitRes
  >,
  'division' : ActorMethod<[bigint, bigint], number>,
  'execute_insurance_contract_manual' : ActorMethod<
    [number],
    ExecuteInsuranceContractRes
  >,
  'f64_to_biguint' : ActorMethod<[number], bigint>,
  'get_all_contract_execution_logs' : ActorMethod<
    [],
    Array<InsuranceContractExecutionLogsKeys>
  >,
  'get_buy_insurance_contract_list_by_princicpal' : ActorMethod<
    [],
    Array<InsuranceBuyersKey>
  >,
  'get_contract_execution_logs_by_insurance_id' : ActorMethod<
    [number],
    Array<InsuranceContractExecutionLogsKeys>
  >,
  'get_inflation_data' : ActorMethod<[Country, [] | [string]], Result>,
  'get_insurance_by_id' : ActorMethod<[number], [] | [Insurance]>,
  'get_ledger_canister_id' : ActorMethod<[InsuranceAssets], Principal>,
  'get_pool_balance_by_insurance_id' : ActorMethod<[number], bigint>,
  'get_premium_pool_balance_by_insurance_id' : ActorMethod<[number], bigint>,
  'get_seller_insurance_contract_list_by_princicpal' : ActorMethod<
    [],
    Array<InsuranceBuyersKey>
  >,
  'get_user_insurance_history_by_principal' : ActorMethod<
    [],
    Array<[UserInsuranceListHistoryKey, bigint]>
  >,
  'list_insurance_contract' : ActorMethod<
    [],
    Array<[InsuranceActiveListKey, null]>
  >,
  'multiplication' : ActorMethod<[number, number], number>,
  'nat_to_f64' : ActorMethod<[bigint], number>,
  'sell_insurance_contract' : ActorMethod<[SellInsuranceArgs], BuyInsuranceRes>,
  'set_ledger_canister_id' : ActorMethod<
    [InsuranceAssets, Principal],
    undefined
  >,
  'transform_fred' : ActorMethod<[TransformArgs], HttpResponse>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
