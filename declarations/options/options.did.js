export const idlFactory = ({ IDL }) => {
  const OptionsType = IDL.Variant({ 'PUT' : IDL.Null, 'CALL' : IDL.Null });
  const OptionsAssetsIcrc = IDL.Variant({
    'ICP' : IDL.Null,
    'CKUSDT' : IDL.Null,
    'CKBTC' : IDL.Null,
    'CKETH' : IDL.Null,
  });
  const OptionsAssets = IDL.Variant({
    'BTC' : IDL.Null,
    'ETH' : IDL.Null,
    'ICRC' : OptionsAssetsIcrc,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Float64, 'Err' : IDL.Text });
  const OptionsContractState = IDL.Variant({
    'EXECUTED' : IDL.Null,
    'OPEN' : IDL.Null,
    'EXPIRED' : IDL.Null,
    'OFFER' : IDL.Null,
    'CLOSED' : IDL.Null,
  });
  const CreateOptionArgs = IDL.Record({
    'asset' : OptionsAssets,
    'strike_price' : IDL.Nat,
    'options_type' : OptionsType,
    'contract_state' : OptionsContractState,
    'offer_duration' : IDL.Nat64,
    'asset_amount' : IDL.Nat,
    'use_exchange_account' : IDL.Bool,
    'contract_expiry' : IDL.Nat64,
  });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const Options = IDL.Record({
    'asset' : OptionsAssets,
    'strike_price' : IDL.Nat,
    'options_type' : OptionsType,
    'contract_state' : OptionsContractState,
    'offer_duration' : IDL.Nat64,
    'name' : IDL.Text,
    'seller' : IDL.Principal,
    'asset_amount' : IDL.Nat,
    'timestamp' : IDL.Nat64,
    'contract_expiry' : IDL.Nat64,
    'buyer' : IDL.Opt(IDL.Principal),
  });
  const OptionsAssetsByNames = IDL.Variant({
    'BTC' : IDL.Null,
    'ETH' : IDL.Null,
    'ICP' : IDL.Null,
    'USDT' : IDL.Null,
    'CKBTC' : IDL.Null,
    'CKETH' : IDL.Null,
  });
  const OptionsActiveListKey = IDL.Record({
    'id' : IDL.Nat64,
    'options_asset' : OptionsAssetsByNames,
    'options_type' : OptionsType,
    'offer_duration' : IDL.Nat64,
    'timestamp' : IDL.Nat64,
  });
  const CanisterName = IDL.Variant({ 'ExchangeRate' : IDL.Null });
  const Result_3 = IDL.Variant({ 'Ok' : IDL.Nat64, 'Err' : IDL.Text });
  const TradedOptionsContractsKey = IDL.Record({
    'id' : IDL.Nat64,
    'principal' : IDL.Principal,
    'contract_state' : IDL.Text,
    'timestamp' : IDL.Nat64,
  });
  const TradedOptionsContractsValue = IDL.Record({
    'trade_timestamp' : IDL.Nat64,
    'options_name' : IDL.Text,
    'options_type' : IDL.Text,
  });
  const HttpHeader = IDL.Record({ 'value' : IDL.Text, 'name' : IDL.Text });
  const HttpResponse = IDL.Record({
    'status' : IDL.Nat,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(HttpHeader),
  });
  const TransformArgs = IDL.Record({
    'context' : IDL.Vec(IDL.Nat8),
    'response' : HttpResponse,
  });
  return IDL.Service({
    'calculate_premium' : IDL.Func(
        [IDL.Nat, OptionsType, IDL.Nat64, OptionsAssets],
        [Result],
        [],
      ),
    'convert_asset_amount_to_human' : IDL.Func(
        [OptionsAssets, IDL.Nat],
        [IDL.Float64],
        ['query'],
      ),
    'convert_asset_amount_to_non_human' : IDL.Func(
        [OptionsAssets, IDL.Float64],
        [IDL.Nat],
        ['query'],
      ),
    'convert_premium_amount_to_non_humans' : IDL.Func(
        [OptionsAssets, IDL.Float64],
        [IDL.Nat],
        ['query'],
      ),
    'convert_xrc_human_to_non_humans' : IDL.Func(
        [IDL.Float64],
        [IDL.Nat],
        ['query'],
      ),
    'convert_xrc_non_human_to_human' : IDL.Func(
        [IDL.Nat],
        [IDL.Float64],
        ['query'],
      ),
    'create_icrc_options' : IDL.Func(
        [OptionsAssetsIcrc, CreateOptionArgs],
        [Result_1],
        [],
      ),
    'execute_manual' : IDL.Func([OptionsAssetsIcrc, IDL.Nat64], [Result_2], []),
    'get_all_options' : IDL.Func([], [IDL.Vec(Options)], ['query']),
    'get_all_options_ids' : IDL.Func([], [IDL.Vec(IDL.Nat64)], ['query']),
    'get_call_options_by_asset' : IDL.Func(
        [OptionsAssetsByNames],
        [IDL.Vec(IDL.Tuple(OptionsActiveListKey, IDL.Null))],
        ['query'],
      ),
    'get_canister_id' : IDL.Func([CanisterName], [IDL.Principal], ['query']),
    'get_exchange_rate' : IDL.Func([OptionsAssets], [Result_3], []),
    'get_ledger_canister_id' : IDL.Func(
        [OptionsAssetsIcrc],
        [IDL.Principal],
        ['query'],
      ),
    'get_options_trade_history_by_principal' : IDL.Func(
        [OptionsContractState],
        [
          IDL.Vec(
            IDL.Tuple(TradedOptionsContractsKey, TradedOptionsContractsValue)
          ),
        ],
        ['query'],
      ),
    'get_put_options_by_asset' : IDL.Func(
        [OptionsAssetsByNames],
        [IDL.Vec(IDL.Tuple(OptionsActiveListKey, IDL.Null))],
        ['query'],
      ),
    'remaining_time_in_years' : IDL.Func([IDL.Nat64], [IDL.Float32], ['query']),
    'set_canister_id' : IDL.Func([CanisterName, IDL.Principal], [], []),
    'set_ledger_canister_id' : IDL.Func(
        [OptionsAssetsIcrc, IDL.Principal],
        [],
        [],
      ),
    'trade_icrc_options' : IDL.Func(
        [OptionsAssetsIcrc, IDL.Nat64],
        [Result_1],
        [],
      ),
    'transform_fred' : IDL.Func([TransformArgs], [HttpResponse], ['query']),
  });
};
export const init = ({ IDL }) => { return [IDL.Record({})]; };
