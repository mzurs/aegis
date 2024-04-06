import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Account {
  'principal' : Principal,
  'user_name' : [] | [string],
  'user_id' : bigint,
}
export type BitcoinNetwork = { 'mainnet' : null } |
  { 'regtest' : null } |
  { 'testnet' : null };
export interface Constants {
  'minter_ids' : MinterIds,
  'ledger_ids' : LedgerIds,
}
export type ICRCLedgerType = { 'ICP' : null } |
  { 'CKBTC' : null } |
  { 'CKETH' : null };
export interface InitArgs { 'bitcoin_network' : BitcoinNetwork }
export interface LedgerIds {
  'ckbtc_ledger_id' : Principal,
  'icp_ledger_id' : Principal,
  'cketh_ledger_id' : Principal,
}
export type Metric = { 'UserCounts' : null } |
  { 'ActiveUsers' : null };
export type MetricValues = { 'UserCounts' : bigint } |
  { 'ActiveUsers' : bigint };
export interface MinterIds {
  'ckbtc_minter_id' : Principal,
  'cketh_minter_id' : Principal,
}
export interface PendingUtxo {
  'confirmations' : number,
  'value' : bigint,
  'outpoint' : UtxoOutpoint,
}
export type RejectionCode = { 'NoError' : null } |
  { 'CanisterError' : null } |
  { 'SysTransient' : null } |
  { 'DestinationInvalid' : null } |
  { 'Unknown' : null } |
  { 'SysFatal' : null } |
  { 'CanisterReject' : null };
export type Result = { 'Ok' : boolean } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : [Account] } |
  { 'Err' : [RejectionCode, string] };
export type Result_2 = { 'Ok' : [RetrieveBtcRet] } |
  { 'Err' : [RejectionCode, string] };
export type Result_3 = { 'Ok' : bigint } |
  { 'Err' : TransferError };
export type Result_4 = { 'Ok' : null } |
  { 'Err' : string };
export type Result_5 = { 'Ok' : [UpdateBalanceRet] } |
  { 'Err' : [RejectionCode, string] };
export type RetrieveBtcError = { 'MalformedAddress' : string } |
  { 'GenericError' : { 'error_message' : string, 'error_code' : bigint } } |
  { 'TemporarilyUnavailable' : string } |
  { 'AlreadyProcessing' : null } |
  { 'AmountTooLow' : bigint } |
  { 'InsufficientFunds' : { 'balance' : bigint } };
export interface RetrieveBtcOk { 'block_index' : bigint }
export type RetrieveBtcRet = { 'Ok' : RetrieveBtcOk } |
  { 'Err' : RetrieveBtcError };
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
export type UpdateBalanceError = {
    'GenericError' : { 'error_message' : string, 'error_code' : bigint }
  } |
  { 'TemporarilyUnavailable' : string } |
  { 'AlreadyProcessing' : null } |
  {
    'NoNewUtxos' : {
      'required_confirmations' : number,
      'pending_utxos' : [] | [Array<PendingUtxo>],
      'current_confirmations' : [] | [number],
    }
  };
export type UpdateBalanceRet = { 'Ok' : Array<UtxoStatus> } |
  { 'Err' : UpdateBalanceError };
export interface Utxo {
  'height' : number,
  'value' : bigint,
  'outpoint' : UtxoOutpoint,
}
export interface UtxoOutpoint {
  'txid' : Uint8Array | number[],
  'vout' : number,
}
export type UtxoStatus = { 'ValueTooSmall' : Utxo } |
  { 'Tainted' : Utxo } |
  {
    'Minted' : {
      'minted_amount' : bigint,
      'block_index' : bigint,
      'utxo' : Utxo,
    }
  } |
  { 'Checked' : Utxo };
export interface _SERVICE {
  'create_account' : ActorMethod<[], Result>,
  'get_account' : ActorMethod<[], [] | [Account]>,
  'get_bitcoin_network' : ActorMethod<[], BitcoinNetwork>,
  'get_btc_address' : ActorMethod<[], string>,
  'get_btc_balance' : ActorMethod<[string], bigint>,
  'get_constants' : ActorMethod<[], Constants>,
  'get_deposit_fee' : ActorMethod<[], bigint>,
  'get_id' : ActorMethod<[], Principal>,
  'get_metrics' : ActorMethod<[Metric], MetricValues>,
  'get_user_balance' : ActorMethod<[], bigint>,
  'get_withdrawal_account' : ActorMethod<[], Result_1>,
  'principal_to_hex' : ActorMethod<[Principal], string>,
  'principal_to_subaccount' : ActorMethod<[Principal], Uint8Array | number[]>,
  'retrieve_btc' : ActorMethod<[string, bigint], Result_2>,
  'set_ledger_ids' : ActorMethod<[LedgerIds], undefined>,
  'set_minter_ids' : ActorMethod<[MinterIds], undefined>,
  'transfer_from_account' : ActorMethod<[bigint, ICRCLedgerType], Result_3>,
  'update_account_user_name' : ActorMethod<[string], Result_4>,
  'update_btc_balance' : ActorMethod<[], Result_5>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
