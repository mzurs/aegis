export const idlFactory = ({ IDL }) => {
  const BitcoinNetwork = IDL.Variant({
    'mainnet' : IDL.Null,
    'regtest' : IDL.Null,
    'testnet' : IDL.Null,
  });
  const InitArgs = IDL.Record({ 'bitcoin_network' : BitcoinNetwork });
  const Result = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text });
  const Account = IDL.Record({
    'principal' : IDL.Principal,
    'user_name' : IDL.Opt(IDL.Text),
    'user_id' : IDL.Nat64,
  });
  const MinterIds = IDL.Record({
    'ckbtc_minter_id' : IDL.Principal,
    'cketh_minter_id' : IDL.Principal,
  });
  const LedgerIds = IDL.Record({
    'ckbtc_ledger_id' : IDL.Principal,
    'icp_ledger_id' : IDL.Principal,
    'cketh_ledger_id' : IDL.Principal,
  });
  const Constants = IDL.Record({
    'minter_ids' : MinterIds,
    'ledger_ids' : LedgerIds,
  });
  const Metric = IDL.Variant({
    'UserCounts' : IDL.Null,
    'ActiveUsers' : IDL.Null,
  });
  const MetricValues = IDL.Variant({
    'UserCounts' : IDL.Nat64,
    'ActiveUsers' : IDL.Nat64,
  });
  const RejectionCode = IDL.Variant({
    'NoError' : IDL.Null,
    'CanisterError' : IDL.Null,
    'SysTransient' : IDL.Null,
    'DestinationInvalid' : IDL.Null,
    'Unknown' : IDL.Null,
    'SysFatal' : IDL.Null,
    'CanisterReject' : IDL.Null,
  });
  const Result_1 = IDL.Variant({
    'Ok' : IDL.Tuple(Account),
    'Err' : IDL.Tuple(RejectionCode, IDL.Text),
  });
  const RetrieveBtcOk = IDL.Record({ 'block_index' : IDL.Nat64 });
  const RetrieveBtcError = IDL.Variant({
    'MalformedAddress' : IDL.Text,
    'GenericError' : IDL.Record({
      'error_message' : IDL.Text,
      'error_code' : IDL.Nat64,
    }),
    'TemporarilyUnavailable' : IDL.Text,
    'AlreadyProcessing' : IDL.Null,
    'AmountTooLow' : IDL.Nat64,
    'InsufficientFunds' : IDL.Record({ 'balance' : IDL.Nat64 }),
  });
  const RetrieveBtcRet = IDL.Variant({
    'Ok' : RetrieveBtcOk,
    'Err' : RetrieveBtcError,
  });
  const Result_2 = IDL.Variant({
    'Ok' : IDL.Tuple(RetrieveBtcRet),
    'Err' : IDL.Tuple(RejectionCode, IDL.Text),
  });
  const ICRCLedgerType = IDL.Variant({
    'ICP' : IDL.Null,
    'CKBTC' : IDL.Null,
    'CKETH' : IDL.Null,
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
  const Result_3 = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : TransferError });
  const Result_4 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const UtxoOutpoint = IDL.Record({
    'txid' : IDL.Vec(IDL.Nat8),
    'vout' : IDL.Nat32,
  });
  const Utxo = IDL.Record({
    'height' : IDL.Nat32,
    'value' : IDL.Nat64,
    'outpoint' : UtxoOutpoint,
  });
  const UtxoStatus = IDL.Variant({
    'ValueTooSmall' : Utxo,
    'Tainted' : Utxo,
    'Minted' : IDL.Record({
      'minted_amount' : IDL.Nat64,
      'block_index' : IDL.Nat64,
      'utxo' : Utxo,
    }),
    'Checked' : Utxo,
  });
  const PendingUtxo = IDL.Record({
    'confirmations' : IDL.Nat32,
    'value' : IDL.Nat64,
    'outpoint' : UtxoOutpoint,
  });
  const UpdateBalanceError = IDL.Variant({
    'GenericError' : IDL.Record({
      'error_message' : IDL.Text,
      'error_code' : IDL.Nat64,
    }),
    'TemporarilyUnavailable' : IDL.Text,
    'AlreadyProcessing' : IDL.Null,
    'NoNewUtxos' : IDL.Record({
      'required_confirmations' : IDL.Nat32,
      'pending_utxos' : IDL.Opt(IDL.Vec(PendingUtxo)),
      'current_confirmations' : IDL.Opt(IDL.Nat32),
    }),
  });
  const UpdateBalanceRet = IDL.Variant({
    'Ok' : IDL.Vec(UtxoStatus),
    'Err' : UpdateBalanceError,
  });
  const Result_5 = IDL.Variant({
    'Ok' : IDL.Tuple(UpdateBalanceRet),
    'Err' : IDL.Tuple(RejectionCode, IDL.Text),
  });
  return IDL.Service({
    'create_account' : IDL.Func([], [Result], []),
    'get_account' : IDL.Func([], [IDL.Opt(Account)], ['query']),
    'get_bitcoin_network' : IDL.Func([], [BitcoinNetwork], ['query']),
    'get_btc_address' : IDL.Func([], [IDL.Text], []),
    'get_btc_balance' : IDL.Func([IDL.Text], [IDL.Nat64], []),
    'get_constants' : IDL.Func([], [Constants], ['query']),
    'get_deposit_fee' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_id' : IDL.Func([], [IDL.Principal], ['query']),
    'get_metrics' : IDL.Func([Metric], [MetricValues], ['query']),
    'get_user_balance' : IDL.Func([], [IDL.Nat], ['query']),
    'get_withdrawal_account' : IDL.Func([], [Result_1], []),
    'principal_to_hex' : IDL.Func([IDL.Principal], [IDL.Text], ['query']),
    'principal_to_subaccount' : IDL.Func(
        [IDL.Principal],
        [IDL.Vec(IDL.Nat8)],
        ['query'],
      ),
    'retrieve_btc' : IDL.Func([IDL.Text, IDL.Nat64], [Result_2], []),
    'set_ledger_ids' : IDL.Func([LedgerIds], [], []),
    'set_minter_ids' : IDL.Func([MinterIds], [], []),
    'transfer_from_account' : IDL.Func(
        [IDL.Nat64, ICRCLedgerType],
        [Result_3],
        [],
      ),
    'update_account_user_name' : IDL.Func([IDL.Text], [Result_4], []),
    'update_btc_balance' : IDL.Func([], [Result_5], []),
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
