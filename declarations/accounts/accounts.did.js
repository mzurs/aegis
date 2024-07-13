export const idlFactory = ({ IDL }) => {
  const BitcoinNetwork = IDL.Variant({
    'mainnet' : IDL.Null,
    'regtest' : IDL.Null,
    'testnet' : IDL.Null,
  });
  const InitArgs = IDL.Record({ 'bitcoin_network' : BitcoinNetwork });
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
  const IcrcTransferResult = IDL.Variant({
    'TransferErrorString' : IDL.Text,
    'TransferErrorMessage' : TransferError,
    'TransferSuccess' : IDL.Nat,
  });
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
  const ConvertCkBTCResult = IDL.Variant({
    'ConvertSuccess' : IDL.Nat,
    'IcrcTransferResult' : IcrcTransferResult,
    'ErrMessage' : IDL.Text,
    'RetrieveBtcError' : RetrieveBtcError,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text });
  const AegisAccountInfo = IDL.Record({
    'user_name' : IDL.Opt(IDL.Text),
    'user_id' : IDL.Nat64,
  });
  const CanisterName = IDL.Variant({
    'ICP' : IDL.Null,
    'CKBTCMINTER' : IDL.Null,
    'CKETHMINTER' : IDL.Null,
    'CKBTC' : IDL.Null,
    'CKETH' : IDL.Null,
  });
  const Metric = IDL.Variant({ 'UserCounts' : IDL.Null });
  const MetricValues = IDL.Variant({ 'UserCounts' : IDL.Nat64 });
  const Account = IDL.Record({
    'owner' : IDL.Principal,
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
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
  const RetrieveBtcOk = IDL.Record({ 'block_index' : IDL.Nat64 });
  const RetrieveBtcResult = IDL.Variant({
    'RetrieveBtcString' : IDL.Text,
    'RetrieveBtcOk' : RetrieveBtcOk,
    'RetrieveBtcError' : RetrieveBtcError,
  });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  return IDL.Service({
    'convert_ckbtc' : IDL.Func([IDL.Text, IDL.Nat], [ConvertCkBTCResult], []),
    'create_account' : IDL.Func([], [Result], []),
    'get_account' : IDL.Func([], [IDL.Opt(AegisAccountInfo)], ['query']),
    'get_bitcoin_network' : IDL.Func([], [BitcoinNetwork], ['query']),
    'get_btc_address' : IDL.Func([], [IDL.Text], []),
    'get_canister_id' : IDL.Func([CanisterName], [IDL.Principal], ['query']),
    'get_metrics' : IDL.Func([Metric], [MetricValues], ['query']),
    'icrc_get_balance' : IDL.Func([CanisterName], [IDL.Nat], ['query']),
    'icrc_transfer_from_account' : IDL.Func(
        [CanisterName, IDL.Opt(Account), IDL.Nat],
        [IcrcTransferResult],
        [],
      ),
    'icrc_transfer_to_account' : IDL.Func(
        [CanisterName, IDL.Nat],
        [IcrcTransferFromResult],
        [],
      ),
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
    'retrieve_btc' : IDL.Func([IDL.Text, IDL.Nat], [RetrieveBtcResult], []),
    'set_canister_id' : IDL.Func([CanisterName, IDL.Principal], [], []),
    'update_account_user_name' : IDL.Func([IDL.Text], [Result_1], []),
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
