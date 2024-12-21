import {
  _SERVICE as _ACCOUNTS,
  idlFactory as idlFactoryAccounts,
  init as initAccounts,
  Account,
} from "../../declarations/accounts/accounts.did";

import {
  _SERVICE as _XRC,
  idlFactory as idlFactoryXrc,
  init as initXrc,
} from "../../declarations/xrc/xrc.did";
import {
  _SERVICE as _CKUSDT_LEDGER,
  idlFactory as idlFactoryCkusdtLedger,
  init as initCkusdtLedger,
} from "../../declarations/ckusdt_ledger/ckusdt_ledger.did";

import {
  _SERVICE as _MAIN,
  idlFactory as idlFactoryMain,
  init as initMain,
} from "../../declarations/main/main.did";

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

import {
  _SERVICE as _AEGIS_LEDGER,
  idlFactory as idlFactoryAegisLedger,
  init as initAEGISLedger,
} from "../../declarations/aegis_ledger/aegis_ledger.did";

import {
  _SERVICE as _OPTIONS,
  idlFactory as idlFactoryOptions,
  init as initOptions,
} from "../../declarations/options/options.did";

export {
  _CKUSDT_LEDGER,
  _XRC,
  _ICP_LEDGER,
  _CKBTC_MINTER,
  _CKETH_LEDGER,
  _CKETH_MINTER,
  _ACCOUNTS,
  _INSURANCE,
  _MAIN,
  _CKBTC_LEDGER,
  _AEGIS_LEDGER,
  _KYT,
  _OPTIONS,
  idlFactoryCkusdtLedger,
  idlFactoryKYT,
  idlFactoryAccounts,
  idlFactoryInsurance,
  idlFactoryCkbtcLedger,
  idlFactoryCkbtcMinter,
  idlFactoryCkethLedger,
  idlFactoryCkethMinter,
  idlFactoryIcpLedger,
  idlFactoryAegisLedger,
  idlFactoryMain,
  idlFactoryOptions,
  idlFactoryXrc,
};

export type SERVICES =
  | _CKUSDT_LEDGER
  | _XRC
  | _MAIN
  | _AEGIS_LEDGER
  | _ICP_LEDGER
  | _CKBTC_MINTER
  | _CKETH_LEDGER
  | _CKETH_MINTER
  | _ACCOUNTS
  | _INSURANCE
  | _CKBTC_LEDGER
  | _KYT
  | _OPTIONS;

export type IDLS =
  | typeof idlFactoryCkusdtLedger
  | typeof idlFactoryXrc
  | typeof idlFactoryAccounts
  | typeof idlFactoryInsurance
  | typeof idlFactoryCkbtcLedger
  | typeof idlFactoryCkbtcMinter
  | typeof idlFactoryCkethLedger
  | typeof idlFactoryCkethMinter
  | typeof idlFactoryIcpLedger
  | typeof idlFactoryKYT
  | typeof idlFactoryAegisLedger
  | typeof idlFactoryMain
  | typeof idlFactoryOptions;

export {
  initCkusdtLedger,
  initAccounts,
  initInsurance,
  initCkBtcLedger,
  initCkBtcMinter,
  initCkEthLedger,
  initCkEthMinter,
  initIcpLedger,
  intKYT,
  initAEGISLedger,
  initMain,
  initOptions,
  initXrc,
};

export { Account };
