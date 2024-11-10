export const idlFactory = ({ IDL }) => {
  const OptionsAssetsIcrc = IDL.Variant({
    'ICP' : IDL.Null,
    'CKBTC' : IDL.Null,
    'CKETH' : IDL.Null,
  });
  const OptionsAssets = IDL.Variant({
    'BTC' : IDL.Null,
    'ETH' : IDL.Null,
    'ICRC' : OptionsAssetsIcrc,
  });
  const OptionsType = IDL.Variant({ 'PUT' : IDL.Null, 'CALL' : IDL.Null });
  const OptionsContractState = IDL.Variant({
    'EXECUTED' : IDL.Null,
    'OPEN' : IDL.Null,
    'EXPIRED' : IDL.Null,
    'CLOSED' : IDL.Null,
  });
  const CreateOptionArgs = IDL.Record({
    'asset' : OptionsAssets,
    'options_type' : OptionsType,
    'contract_state' : OptionsContractState,
    'asset_amount' : IDL.Nat,
    'contract_expiry' : IDL.Nat64,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text });
  const Options = IDL.Record({
    'asset' : OptionsAssets,
    'options_type' : OptionsType,
    'contract_state' : OptionsContractState,
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
    'CKBTC' : IDL.Null,
    'CKETH' : IDL.Null,
  });
  const OptionsActiveListKey = IDL.Record({
    'id' : IDL.Nat64,
    'options_asset' : OptionsAssetsByNames,
    'options_type' : OptionsType,
    'timestamp' : IDL.Nat64,
  });
  return IDL.Service({
    'create_icrc_options' : IDL.Func(
        [OptionsAssetsIcrc, CreateOptionArgs],
        [Result],
        [],
      ),
    'get_all_options' : IDL.Func([], [IDL.Vec(Options)], ['query']),
    'get_all_options_ids' : IDL.Func([], [IDL.Vec(IDL.Nat64)], ['query']),
    'get_call_options_by_asset' : IDL.Func(
        [OptionsAssetsByNames],
        [IDL.Vec(IDL.Tuple(OptionsActiveListKey, IDL.Null))],
        ['query'],
      ),
    'get_ledger_canister_id' : IDL.Func(
        [OptionsAssetsIcrc],
        [IDL.Principal],
        [],
      ),
    'get_put_options_by_asset' : IDL.Func(
        [OptionsAssetsByNames],
        [IDL.Vec(IDL.Tuple(OptionsActiveListKey, IDL.Null))],
        ['query'],
      ),
    'set_ledger_canister_id' : IDL.Func(
        [OptionsAssetsIcrc, IDL.Principal],
        [],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return [IDL.Record({})]; };
