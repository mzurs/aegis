import { Principal } from "@dfinity/principal";

export enum CANISTERS_NAME_NO_PIC {
    ACCOUNTS,
    INSURANCE,
    ICP_LEDGER,
    CKBTC_LEDGER,
    CKETH_LEDGER,
    CKBTC_MINTER,
    CKETH_MINTER,
    KYT,
  }
  
let CANISTER_IDS: (CANISTERS_NAME_NO_PIC | Principal)[][] = [
    [
      CANISTERS_NAME_NO_PIC.ICP_LEDGER,
      Principal.fromText("ryjl3-tyaaa-aaaaa-aaaba-cai"),
    ],
    [
      CANISTERS_NAME_NO_PIC.CKBTC_LEDGER,
      Principal.fromText("mxzaz-hqaaa-aaaar-qaada-cai"),
    ],
    [
      CANISTERS_NAME_NO_PIC.CKETH_LEDGER,
      Principal.fromText("apia6-jaaaa-aaaar-qabma-cai"),
    ],
    [
      CANISTERS_NAME_NO_PIC.CKBTC_MINTER,
      Principal.fromText("mqygn-kiaaa-aaaar-qaadq-cai"),
    ],
    [
      CANISTERS_NAME_NO_PIC.CKETH_MINTER,
      Principal.fromText("jzenf-aiaaa-aaaar-qaa7q-cai"),
    ],
    [CANISTERS_NAME_NO_PIC.ACCOUNTS, Principal.fromText("222qi-2qaaa-aaaao-anesa-cai")],
    [CANISTERS_NAME_NO_PIC.INSURANCE,Principal.fromText("suaf3-hqaaa-aaaaf-bfyoa-cai")],
    [CANISTERS_NAME_NO_PIC.KYT, Principal.fromText("pjihx-aaaaa-aaaar-qaaka-cai")],
  ];

  export let CANISTER_IDS_MAP_NO_PIC = new Map(
    CANISTER_IDS.map(([key, value]) => [key, value as Principal])
  );
  