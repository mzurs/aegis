// import { IDL } from "@dfinity/candid";
import { Principal } from "@dfinity/principal";
import { resolve } from "path";

export enum CANISTERS_NAME {
  ACCOUNTS,
  INSURANCE,
  ICP_LEDGER,
  CKBTC_LEDGER,
  CKETH_LEDGER,
  CKBTC_MINTER,
  CKETH_MINTER,
  KYT,
}

export const CONTRACT_ADDRESS = "0xb44B5e756A894775FC32EDdf3314Bb1B1944dC34";

export const MINTING_ACCOUNT_IDENTIFIER: string =
  "45d0216adec1b2bad4363f729427c3a1f234c42e40a7c0abb9db21ece6e652d9";

export const MINTER_PRINCIPAL = (): Principal => {
  const principal = Principal.fromText(
    "rdhnv-n3lnd-mzihd-j44oc-k2mau-ajkst-5yxhr-grepj-eb2xn-uwpej-3qe"
  );

  return principal;
};
let CANISTER_IDS: (CANISTERS_NAME | Principal)[][] = [

  [
    CANISTERS_NAME.ICP_LEDGER,
    Principal.fromText("4fq4b-sqaaa-aaaar-qadna-cai"),
  ],
  [
    CANISTERS_NAME.CKBTC_LEDGER,
    Principal.fromText("3hbro-siaaa-aaaar-qaeaa-cai"),
  ],
  [
    CANISTERS_NAME.CKETH_LEDGER,
    Principal.fromText("apia6-jaaaa-aaaar-qabma-cai"),
  ],
  [
    CANISTERS_NAME.CKBTC_MINTER,
    Principal.fromText("3sgad-taaaa-aaaar-qaedq-cai"),
  ],
  [
    CANISTERS_NAME.CKETH_MINTER,
    Principal.fromText("55zjc-4qaaa-aaaar-qadja-cai"),
  ],
  [CANISTERS_NAME.ACCOUNTS, Principal.fromText("222qi-2qaaa-aaaao-anesa-cai")],
  [CANISTERS_NAME.KYT, Principal.fromText("3rrzb-lyaaa-aaaar-qad6q-cai")],
  
  // [
  //   CANISTERS_NAME.ICP_LEDGER,
  //   Principal.fromText("ryjl3-tyaaa-aaaaa-aaaba-cai"),
  // ],
  // [
  //   CANISTERS_NAME.CKBTC_LEDGER,
  //   Principal.fromText("mxzaz-hqaaa-aaaar-qaada-cai"),
  // ],
  // [
  //   CANISTERS_NAME.CKETH_LEDGER,
  //   Principal.fromText("apia6-jaaaa-aaaar-qabma-cai"),
  // ],
  // [
  //   CANISTERS_NAME.CKBTC_MINTER,
  //   Principal.fromText("mqygn-kiaaa-aaaar-qaadq-cai"),
  // ],
  // [
  //   CANISTERS_NAME.CKETH_MINTER,
  //   Principal.fromText("jzenf-aiaaa-aaaar-qaa7q-cai"),
  // ],
  // [CANISTERS_NAME.ACCOUNTS, Principal.fromText("222qi-2qaaa-aaaao-anesa-cai")],
  // [CANISTERS_NAME.INSURANCE,Principal.fromText("suaf3-hqaaa-aaaaf-bfyoa-cai")],
  // [CANISTERS_NAME.KYT, Principal.fromText("pjihx-aaaaa-aaaar-qaaka-cai")],
];

export let CANISTER_IDS_MAP = new Map(
  CANISTER_IDS.map(([key, value]) => [key, value as Principal])
);

export const CANISTERS: string[] = [
  "accounts",
  "insurance",
  "icp_ledger",
  "ckbtc_ledger",
  "cketh_ledger",
  "xrc",
  "ckbtc_minter",
  "cketh_minter",
  "kyt",
];

export const ACCOUNTS_WASM_PATH = resolve(
  __dirname,
  "..",
  "..",
  ".dfx",
  "local",
  "canisters",
  "accounts",
  "accounts.wasm"
);

export const INSURANCE_WASM_PATH = resolve(
  __dirname,
  "..",
  "..",
  ".dfx",
  "local",
  "canisters",
  "insurance",
  "insurance.wasm"
);

export const ICP_LEDGER_WASM_PATH = resolve(
  __dirname,
  "..",
  "..",
  ".dfx",
  "local",
  "canisters",
  "icp_ledger",
  "icp_ledger.wasm"
);

export const NNS_STATE_PATH = resolve(
  __dirname,
  "..",
  "..",
  "src",
  "nns_state.tar.gz"
);

export const NNS_SUBNET_ID =
  "nt6ha-vabpm-j6nog-bkr62-vbgbt-swwzc-u54zn-odtoy-igwlu-ab7uj-4qe";

export const CKBTC_LEDGER_WASM_PATH = resolve(
  __dirname,
  "..",
  "..",
  ".dfx",
  "local",
  "canisters",
  "ckbtc_ledger",
  "ckbtc_ledger.wasm"
);

export const CKETH_LEDGER_WASM_PATH = resolve(
  __dirname,
  "..",
  "..",
  ".dfx",
  "local",
  "canisters",
  "ckbtc_ledger",
  "ckbtc_ledger.wasm"
);

export const CKBTC_MINTER_WASM_PATH = resolve(
  __dirname,
  "..",
  "..",
  "src",
  "minter",
  "ckbtc_minter.wasm"
);

export const CKETH_MINTER_WASM_PATH = resolve(
  __dirname,
  "..",
  "..",
  "src",
  "minter",
  "cketh_minter.wasm"
);

export const KYT_WASM_PATH = resolve(
  __dirname,
  "..",
  "..",
  ".dfx",
  "local",
  "canisters",
  "kyt",
  "kyt.wasm"
);
