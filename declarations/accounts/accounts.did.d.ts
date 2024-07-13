import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Account {
  'owner' : Principal,
  'subaccount' : [] | [Uint8Array | number[]],
}
export interface AegisAccountInfo {
  'user_name' : [] | [string],
  'user_id' : bigint,
}
export type BitcoinNetwork = { 'mainnet' : null } |
  { 'regtest' : null } |
  { 'testnet' : null };
export type CanisterName = { 'ICP' : null } |
  { 'CKBTCMINTER' : null } |
  { 'CKETHMINTER' : null } |
  { 'CKBTC' : null } |
  { 'CKETH' : null };
export type ConvertCkBTCResult = { 'ConvertSuccess' : bigint } |
  { 'IcrcTransferResult' : IcrcTransferResult } |
  { 'ErrMessage' : string } |
  { 'RetrieveBtcError' : RetrieveBtcError };
export type IcrcTransferFromResult = {
    'TransferFromErrorMessage' : TransferFromError
  } |
  { 'TransferFromSuccess' : bigint } |
  { 'TransferFromErrorString' : string };
export type IcrcTransferResult = { 'TransferErrorString' : string } |
  { 'TransferErrorMessage' : TransferError } |
  { 'TransferSuccess' : bigint };
export interface InitArgs { 'bitcoin_network' : BitcoinNetwork }
export type Metric = { 'UserCounts' : null };
export type MetricValues = { 'UserCounts' : bigint };
export type Result = { 'Ok' : boolean } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : null } |
  { 'Err' : string };
export type RetrieveBtcError = { 'MalformedAddress' : string } |
  { 'GenericError' : { 'error_message' : string, 'error_code' : bigint } } |
  { 'TemporarilyUnavailable' : string } |
  { 'AlreadyProcessing' : null } |
  { 'AmountTooLow' : bigint } |
  { 'InsufficientFunds' : { 'balance' : bigint } };
export interface RetrieveBtcOk { 'block_index' : bigint }
export type RetrieveBtcResult = { 'RetrieveBtcString' : string } |
  { 'RetrieveBtcOk' : RetrieveBtcOk } |
  { 'RetrieveBtcError' : RetrieveBtcError };
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
export interface _SERVICE {
  'convert_ckbtc' : ActorMethod<[string, bigint], ConvertCkBTCResult>,
  'create_account' : ActorMethod<[], Result>,
  'get_account' : ActorMethod<[], [] | [AegisAccountInfo]>,
  'get_bitcoin_network' : ActorMethod<[], BitcoinNetwork>,
  'get_btc_address' : ActorMethod<[], string>,
  'get_canister_id' : ActorMethod<[CanisterName], Principal>,
  'get_metrics' : ActorMethod<[Metric], MetricValues>,
  'icrc_get_balance' : ActorMethod<[CanisterName], bigint>,
  'icrc_transfer_from_account' : ActorMethod<
    [CanisterName, [] | [Account], bigint],
    IcrcTransferResult
  >,
  'icrc_transfer_to_account' : ActorMethod<
    [CanisterName, bigint],
    IcrcTransferFromResult
  >,
  'principal_to_eth_address' : ActorMethod<[Principal], string>,
  'principal_to_subaccount' : ActorMethod<[Principal], Uint8Array | number[]>,
  'retrieve_btc' : ActorMethod<[string, bigint], RetrieveBtcResult>,
  'set_canister_id' : ActorMethod<[CanisterName, Principal], undefined>,
  'update_account_user_name' : ActorMethod<[string], Result_1>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
