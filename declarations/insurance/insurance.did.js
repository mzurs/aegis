export const idlFactory = ({ IDL }) => {
  const BuyInsuranceArgs = IDL.Record({
    'premium' : IDL.Nat,
    'insurance_id' : IDL.Nat32,
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
  const BuyInsuranceRes = IDL.Variant({
    'ErrorMessage' : IDL.Text,
    'TransferError' : IcrcTransferFromResult,
    'Success' : IDL.Null,
  });
  const InsuranceRewardsMultiplier = IDL.Variant({
    'M2X' : IDL.Null,
    'M3X' : IDL.Null,
    'M4X' : IDL.Null,
  });
  const Country = IDL.Variant({ 'UK' : IDL.Null, 'US' : IDL.Null });
  const InflationBasedInsurance = IDL.Record({
    'country' : Country,
    'target_expiry' : IDL.Nat64,
    'inflation_target' : IDL.Float32,
  });
  const InsuranceCategory = IDL.Variant({
    'InflationBasedInsurance' : InflationBasedInsurance,
  });
  const InsuranceAssets = IDL.Variant({
    'ICP' : IDL.Null,
    'CKBTC' : IDL.Null,
    'CKETH' : IDL.Null,
  });
  const InsuranceContractInitArgs = IDL.Record({
    'multiplier' : InsuranceRewardsMultiplier,
    'title' : IDL.Text,
    'min_premium_amount' : IDL.Nat,
    'description' : IDL.Text,
    'is_muliple_seller_allowed' : IDL.Bool,
    'expiry_date' : IDL.Nat64,
    'category' : InsuranceCategory,
    'insurance_asset' : InsuranceAssets,
    'amount' : IDL.Nat,
    'min_share_amount' : IDL.Opt(IDL.Nat),
  });
  const InsuranceInitRes = IDL.Variant({
    'ErrorMessage' : IDL.Text,
    'TransferError' : IcrcTransferFromResult,
    'Success' : IDL.Nat32,
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
  const IcrcTransferResult = IDL.Variant({
    'TransferErrorString' : IDL.Text,
    'TransferErrorMessage' : TransferError,
    'TransferSuccess' : IDL.Nat,
  });
  const ExecuteInsuranceContractRes = IDL.Variant({
    'ErrorMessage' : IDL.Text,
    'TransferError' : IcrcTransferResult,
    'Success' : IDL.Null,
  });
  const InsuranceContractExecutionLogsKeys = IDL.Record({
    'insurance_id' : IDL.Nat32,
    'message' : IDL.Text,
    'timestamp' : IDL.Nat64,
  });
  const InsuranceBuyersKey = IDL.Record({
    'principal' : IDL.Principal,
    'insurance_id' : IDL.Nat32,
    'time_stamp' : IDL.Nat64,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Float32, 'Err' : IDL.Text });
  const InsuranceContractStatus = IDL.Variant({
    'OPEN' : IDL.Null,
    'CLOSED' : IDL.Null,
  });
  const Insurance = IDL.Record({
    'multiplier' : InsuranceRewardsMultiplier,
    'status' : InsuranceContractStatus,
    'title' : IDL.Text,
    'last_pool_balance' : IDL.Nat,
    'min_premium_amount' : IDL.Nat,
    'description' : IDL.Text,
    'is_muliple_seller_allowed' : IDL.Bool,
    'issuer' : IDL.Principal,
    'expiry_date' : IDL.Nat64,
    'category' : InsuranceCategory,
    'insurance_asset' : InsuranceAssets,
    'last_executed_time' : IDL.Nat64,
    'last_premium_balance' : IDL.Nat,
    'min_share_amount' : IDL.Opt(IDL.Nat),
  });
  const UserInsuranceListHistoryKey = IDL.Record({
    'principal' : IDL.Principal,
    'insurance_id' : IDL.Nat32,
  });
  const InsuranceActiveListKey = IDL.Record({
    'principal' : IDL.Principal,
    'insurance_id' : IDL.Nat32,
  });
  const SellInsuranceArgs = IDL.Record({
    'insurance_id' : IDL.Nat32,
    'amount' : IDL.Nat,
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
    'buy_insurance_contract' : IDL.Func(
        [BuyInsuranceArgs],
        [BuyInsuranceRes],
        [],
      ),
    'convert_u64_to_date' : IDL.Func(
        [IDL.Opt(IDL.Nat64)],
        [IDL.Text],
        ['query'],
      ),
    'create_insurance_contract' : IDL.Func(
        [InsuranceContractInitArgs],
        [InsuranceInitRes],
        [],
      ),
    'division' : IDL.Func([IDL.Nat, IDL.Nat], [IDL.Float64], ['query']),
    'execute_insurance_contract_manual' : IDL.Func(
        [IDL.Nat32],
        [ExecuteInsuranceContractRes],
        [],
      ),
    'f64_to_biguint' : IDL.Func([IDL.Float64], [IDL.Nat], ['query']),
    'get_all_contract_execution_logs' : IDL.Func(
        [],
        [IDL.Vec(InsuranceContractExecutionLogsKeys)],
        ['query'],
      ),
    'get_buy_insurance_contract_list_by_princicpal' : IDL.Func(
        [],
        [IDL.Vec(InsuranceBuyersKey)],
        ['query'],
      ),
    'get_contract_execution_logs_by_insurance_id' : IDL.Func(
        [IDL.Nat32],
        [IDL.Vec(InsuranceContractExecutionLogsKeys)],
        ['query'],
      ),
    'get_inflation_data' : IDL.Func([Country, IDL.Opt(IDL.Text)], [Result], []),
    'get_insurance_by_id' : IDL.Func(
        [IDL.Nat32],
        [IDL.Opt(Insurance)],
        ['query'],
      ),
    'get_ledger_canister_id' : IDL.Func([InsuranceAssets], [IDL.Principal], []),
    'get_pool_balance_by_insurance_id' : IDL.Func(
        [IDL.Nat32],
        [IDL.Nat],
        ['query'],
      ),
    'get_premium_pool_balance_by_insurance_id' : IDL.Func(
        [IDL.Nat32],
        [IDL.Nat],
        ['query'],
      ),
    'get_seller_insurance_contract_list_by_princicpal' : IDL.Func(
        [],
        [IDL.Vec(InsuranceBuyersKey)],
        ['query'],
      ),
    'get_user_insurance_history_by_principal' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(UserInsuranceListHistoryKey, IDL.Nat64))],
        ['query'],
      ),
    'list_insurance_contract' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(InsuranceActiveListKey, IDL.Null))],
        ['query'],
      ),
    'multiplication' : IDL.Func(
        [IDL.Float64, IDL.Float64],
        [IDL.Float64],
        ['query'],
      ),
    'nat_to_f64' : IDL.Func([IDL.Nat], [IDL.Float64], ['query']),
    'sell_insurance_contract' : IDL.Func(
        [SellInsuranceArgs],
        [BuyInsuranceRes],
        [],
      ),
    'set_ledger_canister_id' : IDL.Func(
        [InsuranceAssets, IDL.Principal],
        [],
        [],
      ),
    'transform_fred' : IDL.Func([TransformArgs], [HttpResponse], ['query']),
  });
};
export const init = ({ IDL }) => { return [IDL.Record({})]; };
