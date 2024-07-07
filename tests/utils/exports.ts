import {
  _SERVICE as _ACCOUNTS,
  idlFactory as idlFactoryAccounts,
  init as initAccounts,
  Account,
} from "../../declarations/accounts/accounts.did";

import {
  _SERVICE as _INSURANCE,
  idlFactory as idlFactoryInsurance,
  init as initInsurance,
} from "../../declarations/insurance/insurance.did";

import {
  _SERVICE as _ICP_LEDGER,
  idlFactory as idlFactoryIcpLedger,
  init as initIcpLedger,
} from "../../declarations/icp_ledger/icp_ledger.did";

import {
  _SERVICE as _CKBTC_LEDGER,
  idlFactory as idlFactoryCkbtcLedger,
  init as initCkBtcLedger,
} from "../../declarations/ckbtc_ledger/ckbtc_ledger.did";

import {
  _SERVICE as _CKETH_LEDGER,
  idlFactory as idlFactoryCkethLedger,
  init as initCkEthLedger,
} from "../../declarations/cketh_ledger/cketh_ledger.did";

import {
  _SERVICE as _CKBTC_MINTER,
  idlFactory as idlFactoryCkbtcMinter,
  init as initCkBtcMinter,
} from "../../declarations/ckbtc_minter/ckbtc_minter.did";

import {
  _SERVICE as _CKETH_MINTER,
  idlFactory as idlFactoryCkethMinter,
  init as initCkEthMinter,
} from "../../declarations/cketh_minter/cketh_minter.did";

import {
  _SERVICE as _KYT,
  idlFactory as idlFactoryKYT,
  init as intKYT,
} from "../../declarations/kyt/kyt.did";
export {
  
  _ICP_LEDGER,
  _CKBTC_MINTER,
  _CKETH_LEDGER,
  _CKETH_MINTER,
  _ACCOUNTS,
  _INSURANCE,
  _CKBTC_LEDGER,
  _KYT,
  idlFactoryKYT,
  idlFactoryAccounts,
  idlFactoryInsurance,
  idlFactoryCkbtcLedger,
  idlFactoryCkbtcMinter,
  idlFactoryCkethLedger,
  idlFactoryCkethMinter,
  idlFactoryIcpLedger,
};

export type SERVICES =
  | _ICP_LEDGER
  | _CKBTC_MINTER
  | _CKETH_LEDGER
  | _CKETH_MINTER
  | _ACCOUNTS
  | _INSURANCE
  | _CKBTC_LEDGER
  | _KYT;

export type IDLS =
  | typeof idlFactoryAccounts
  | typeof idlFactoryInsurance
  | typeof idlFactoryCkbtcLedger
  | typeof idlFactoryCkbtcMinter
  | typeof idlFactoryCkethLedger
  | typeof idlFactoryCkethMinter
  | typeof idlFactoryIcpLedger
  | typeof idlFactoryKYT;

export {
  initAccounts,
  initInsurance,
  initCkBtcLedger,
  initCkBtcMinter,
  initCkEthLedger,
  initCkEthMinter,
  initIcpLedger,
  intKYT,
};

export { Account };
