import { Actor, createIdentity, PocketIc } from "@hadronous/pic";
import { IDL } from "@dfinity/candid";

import {
  CANISTER_IDS_MAP,
  CANISTERS_NAME,
  CONTRACT_ADDRESS,
  MINTER_PRINCIPAL,
  MINTING_ACCOUNT_IDENTIFIER,
} from "./constants";
import { Principal } from "@dfinity/principal";
import {
  SERVICES,
  _ACCOUNTS,
  _AEGIS_LEDGER,
  _CKBTC_LEDGER,
  _CKBTC_MINTER,
  _CKETH_LEDGER,
  _CKETH_MINTER,
  _ICP_LEDGER,
  _INSURANCE,
  _KYT,
  _MAIN,
  idlFactoryAccounts,
  idlFactoryAegisLedger,
  idlFactoryCkbtcLedger,
  idlFactoryCkbtcMinter,
  idlFactoryCkethLedger,
  idlFactoryCkethMinter,
  idlFactoryInsurance,
  idlFactoryKYT,
  idlFactoryMain,
} from "./exports";
import { Identity } from "@dfinity/agent";
import {
  ckBTCMinterArgJS,
  ckETHMinterArgJS,
  LedgerArgCandid,
  LedgerArgJS,
  ckBTCMinterArgCandid,
  ckETHMinterArgCandid,
  ICPLedgerCanisterPayloadJS,
  ICPLedgerCanisterPayloadCandid,
  KYTLifecycleArgJS,
  KYTLifecycleArgCandid,
} from "./types";
import { humanToE8s } from "./helpers";
import { parseEther } from "ethers";

let kytArgs: KYTLifecycleArgJS = {
  InitArg: {
    maintainers: [MINTER_PRINCIPAL()],
    mode: {
      AcceptAll: null,
    },
    minter_id: CANISTER_IDS_MAP.get(CANISTERS_NAME.CKBTC_MINTER)!,
  },
};
let ckethMinterArgs: ckETHMinterArgJS = {
  InitArg: {
    ethereum_network: {
      Mainnet: null,
    },
    last_scraped_block_number: 5_642_528n,
    ecdsa_key_name: "master_ecdsa_public_key_yndj2-3ybaa-aaaaa-aaaap-yai",
    next_transaction_nonce: 222n,
    ledger_id: CANISTER_IDS_MAP.get(CANISTERS_NAME.CKETH_LEDGER)!,
    ethereum_contract_address: [CONTRACT_ADDRESS],
    minimum_withdrawal_amount: 10_000_000_000_000_000n,
    ethereum_block_height: {
      Finalized: null,
    },
  },
};

let ckbtcMinterArgs: ckBTCMinterArgJS = {
  Init: {
    kyt_principal: [CANISTER_IDS_MAP.get(CANISTERS_NAME.KYT)!],
    ecdsa_key_name: "dfx_test_key",
    mode: {
      GeneralAvailability: null,
    },
    retrieve_btc_min_amount: 5_000n,
    ledger_id: CANISTER_IDS_MAP.get(CANISTERS_NAME.CKBTC_LEDGER)!,
    max_time_in_queue_nanos: 0n,
    btc_network: {
      Regtest: null,
    },
    min_confirmations: [1],
    kyt_fee: [10n],
  },
};

let icpArgs: ICPLedgerCanisterPayloadJS = {
  Init: {
    send_whitelist: [],
    token_symbol: ["ICP"],
    transfer_fee: [{ e8s: 10000n }],
    minting_account: MINTING_ACCOUNT_IDENTIFIER,
    maximum_number_of_accounts: [],
    accounts_overflow_trim_quantity: [],
    transaction_window: [],
    max_message_size_bytes: [],
    icrc1_minting_account: [
      {
        owner: MINTER_PRINCIPAL(),
        subaccount: [],
      },
    ],
    archive_options: [
      {
        num_blocks_to_archive: 0n,
        max_transactions_per_response: [],
        trigger_threshold: 0n,
        more_controller_ids: [],
        max_message_size_bytes: [],
        cycles_for_archive_creation: [],
        node_max_memory_size_bytes: [],
        controller_id: MINTER_PRINCIPAL(),
      },
    ],
    initial_values: [],
    token_name: ["ICP"],
    feature_flags: [{ icrc2: true }],
  },
};

let aegisArgs: LedgerArgJS = {
  Init: {
    decimals: [8],
    token_symbol: "AEGIS",
    transfer_fee: 100_000n,
    metadata: [],
    minting_account: {
      owner: MINTER_PRINCIPAL()!,
      subaccount: [],
    },
    initial_balances: [],
    maximum_number_of_accounts: [],
    accounts_overflow_trim_quantity: [],
    fee_collector_account: [],
    archive_options: {
      num_blocks_to_archive: 0n,
      max_transactions_per_response: [],
      trigger_threshold: 0n,
      max_message_size_bytes: [],
      cycles_for_archive_creation: [],
      node_max_memory_size_bytes: [],
      controller_id: MINTER_PRINCIPAL()!,
      more_controller_ids: [], //[sender!],
    },
    token_name: "AEGIS",
    max_memo_length: [],
    feature_flags: [{ icrc2: true }],
  },
};

let ckbtcArgs: LedgerArgJS = {
  Init: {
    decimals: [8],
    token_symbol: "ckBTC",
    transfer_fee: 10n,
    metadata: [],
    minting_account: {
      owner: CANISTER_IDS_MAP.get(CANISTERS_NAME.CKBTC_MINTER)!,
      subaccount: [],
    },
    initial_balances: [
      [{ owner: MINTER_PRINCIPAL(), subaccount: [] }, humanToE8s(1000)],
    ],
    maximum_number_of_accounts: [],
    accounts_overflow_trim_quantity: [],
    fee_collector_account: [],
    archive_options: {
      num_blocks_to_archive: 0n,
      max_transactions_per_response: [],
      trigger_threshold: 0n,
      more_controller_ids: [],
      max_message_size_bytes: [],
      cycles_for_archive_creation: [],
      node_max_memory_size_bytes: [],
      controller_id: CANISTER_IDS_MAP.get(CANISTERS_NAME.CKBTC_MINTER)!,
    },
    max_memo_length: [],
    token_name: "ckBTC",
    feature_flags: [{ icrc2: true }],
  },
};
let ckethArgs: LedgerArgJS = {
  Init: {
    decimals: [18],
    token_symbol: "ckETH",
    transfer_fee: 10_000_000_000n,
    metadata: [],
    minting_account: {
      owner: CANISTER_IDS_MAP.get(CANISTERS_NAME.CKETH_MINTER)!,
      subaccount: [],
    },
    initial_balances: [
      [{ owner: MINTER_PRINCIPAL(), subaccount: [] }, parseEther("2")],
    ],
    maximum_number_of_accounts: [],
    accounts_overflow_trim_quantity: [],
    fee_collector_account: [],
    archive_options: {
      num_blocks_to_archive: 0n,
      max_transactions_per_response: [],
      trigger_threshold: 0n,
      // more_controller_ids: [],
      max_message_size_bytes: [],
      cycles_for_archive_creation: [],
      node_max_memory_size_bytes: [],
      controller_id: CANISTER_IDS_MAP.get(CANISTERS_NAME.CKETH_MINTER)!,
      more_controller_ids: [], //[sender!],
    },
    token_name: "ckETH",
    max_memo_length: [],
    feature_flags: [{ icrc2: true }],
  },
};
export const createCanisters = async (
  pic: PocketIc,
  subnetId: Principal,
  sender: Principal
) => {
  const accountsCanisterId = await pic.createCanister({
    sender,
    targetSubnetId: subnetId,
  });
  const mainCanisterId = await pic.createCanister({
    sender,
    targetSubnetId: subnetId,
  });
  const icpCanisterId = await pic.createCanister({
    sender,
    targetSubnetId: subnetId,
  });
  const aegisCanisterId = await pic.createCanister({
    sender,
    targetSubnetId: subnetId,
  });
  const ckbtcCanisterId = await pic.createCanister({
    sender,
    targetSubnetId: subnetId,
  });
  const ckethCanisterId = await pic.createCanister({
    sender,
    targetSubnetId: subnetId,
  });
  const ckbtcMinterCanisterId = await pic.createCanister({
    sender,
    targetSubnetId: subnetId,
  });
  const ckethMinterCanisterId = await pic.createCanister({
    sender,
    targetSubnetId: subnetId,
  });

  CANISTER_IDS_MAP.set(CANISTERS_NAME.ACCOUNTS, accountsCanisterId);
  CANISTER_IDS_MAP.set(CANISTERS_NAME.ICP_LEDGER, icpCanisterId);
  CANISTER_IDS_MAP.set(CANISTERS_NAME.CKBTC_LEDGER, ckbtcCanisterId);
  CANISTER_IDS_MAP.set(CANISTERS_NAME.CKETH_LEDGER, ckethCanisterId);
  CANISTER_IDS_MAP.set(CANISTERS_NAME.CKBTC_MINTER, ckbtcMinterCanisterId);
  CANISTER_IDS_MAP.set(CANISTERS_NAME.CKETH_MINTER, ckethMinterCanisterId);
  CANISTER_IDS_MAP.set(CANISTERS_NAME.AEGIS_LEDGER, aegisCanisterId);
  CANISTER_IDS_MAP.set(CANISTERS_NAME.CKETH_MINTER, mainCanisterId);

  // console.log(CANISTER_IDS_MAP.get(CANISTERS_NAME.ACCOUNTS)?.toString());
  // console.log(CANISTER_IDS_MAP.get(CANISTERS_NAME.CKETH_LEDGER)?.toString());

  return;
};

export const installCode = async (
  canisterName: CANISTERS_NAME,
  wasmPath: string,
  pic: PocketIc,
  sender: Principal
) => {
  switch (canisterName) {
    case CANISTERS_NAME.ACCOUNTS:
      await pic.installCode({
        sender,
        wasm: wasmPath,
        canisterId: CANISTER_IDS_MAP.get(CANISTERS_NAME.ACCOUNTS)!,
        arg: IDL.encode(
          [
            IDL.Record({
              bitcoin_network: IDL.Variant({
                mainnet: IDL.Null,
                regtest: IDL.Null,
                testnet: IDL.Null,
              }),
            }),
          ],
          [
            {
              bitcoin_network: { regtest: null },
            },
          ]
        ),
      });
      break;
    case CANISTERS_NAME.ICP_LEDGER:
      await pic.installCode({
        sender,
        wasm: wasmPath,
        canisterId: CANISTER_IDS_MAP.get(CANISTERS_NAME.ICP_LEDGER)!,
      });
      break;

    case CANISTERS_NAME.CKBTC_LEDGER:
      await pic.installCode({
        sender,
        wasm: wasmPath,
        canisterId: CANISTER_IDS_MAP.get(CANISTERS_NAME.CKBTC_LEDGER)!,
      });
      break;

    case CANISTERS_NAME.CKETH_LEDGER:
      await pic.installCode({
        sender,
        wasm: wasmPath,
        canisterId: CANISTER_IDS_MAP.get(CANISTERS_NAME.CKETH_LEDGER)!,
        arg: IDL.encode([LedgerArgCandid], [ckethArgs]),
      });
      break;

    case CANISTERS_NAME.CKBTC_MINTER:
      await pic.installCode({
        sender,
        wasm: wasmPath,
        canisterId: CANISTER_IDS_MAP.get(CANISTERS_NAME.CKBTC_MINTER)!,
      });
      break;

    case CANISTERS_NAME.CKETH_MINTER:
      await pic.installCode({
        sender,
        wasm: wasmPath,
        canisterId: CANISTER_IDS_MAP.get(CANISTERS_NAME.CKETH_MINTER)!,
      });
      break;

    default:
      throw new Error(`${canisterName} Not Found`);
  }
};

export function createCanisterActor(
  pic: PocketIc,
  canisterName: CANISTERS_NAME
): Actor<SERVICES> {
  let idlFactory, canisterId;
  let actor: Actor<SERVICES>;

  switch (canisterName) {
    case CANISTERS_NAME.ACCOUNTS:
      idlFactory = idlFactoryAccounts;
      canisterId = CANISTER_IDS_MAP.get(CANISTERS_NAME.ACCOUNTS)!;
      actor = pic.createActor<_ACCOUNTS>(idlFactory, canisterId);
      return actor;

    case CANISTERS_NAME.ICP_LEDGER:
      idlFactory = idlFactoryAccounts;
      canisterId = CANISTER_IDS_MAP.get(CANISTERS_NAME.ICP_LEDGER)!;
      actor = pic.createActor<_ICP_LEDGER>(idlFactory, canisterId);
      return actor;

    case CANISTERS_NAME.CKBTC_LEDGER:
      idlFactory = idlFactoryAccounts;
      canisterId = CANISTER_IDS_MAP.get(CANISTERS_NAME.CKBTC_LEDGER)!;
      actor = pic.createActor<_CKBTC_LEDGER>(idlFactory, canisterId);
      return actor;

    case CANISTERS_NAME.CKBTC_MINTER:
      idlFactory = idlFactoryAccounts;
      canisterId = CANISTER_IDS_MAP.get(CANISTERS_NAME.CKBTC_MINTER)!;
      actor = pic.createActor<_CKBTC_MINTER>(idlFactory, canisterId);
      return actor;

    case CANISTERS_NAME.CKETH_LEDGER:
      idlFactory = idlFactoryAccounts;
      canisterId = CANISTER_IDS_MAP.get(CANISTERS_NAME.CKETH_LEDGER)!;
      console.log("🚀 ~ canisterId:", canisterId.toText());
      actor = pic.createActor<_CKETH_LEDGER>(idlFactory, canisterId);
      return actor;

    case CANISTERS_NAME.CKETH_MINTER:
      idlFactory = idlFactoryAccounts;
      canisterId = CANISTER_IDS_MAP.get(CANISTERS_NAME.CKETH_MINTER)!;
      actor = pic.createActor<_CKETH_MINTER>(idlFactory, canisterId);
      return actor;

    default:
      throw new Error(`${canisterName} Not Found`);
  }
}

export async function setupCanister(
  pic: PocketIc,
  sender: Principal,
  wasm: string,
  canisterName: CANISTERS_NAME
): Promise<Actor<SERVICES>> {
  let fixture;
  let fiduciarySubnetId = pic.getFiduciarySubnet()?.id;
  // let fiduciarySubnetId=pic.getBitcoinSubnet()?.id;
  const applicationSubnets = pic.getApplicationSubnets();
  let mainSubnetId = applicationSubnets[0].id;

  switch (canisterName) {
    case CANISTERS_NAME.INSURANCE:
      fixture = await pic.setupCanister<_INSURANCE>({
        sender,
        idlFactory: idlFactoryInsurance,
        wasm,
        targetSubnetId: mainSubnetId,
        arg: IDL.encode(
          [
            IDL.Record({
              bitcoin_network: IDL.Variant({
                mainnet: IDL.Null,
                regtest: IDL.Null,
                testnet: IDL.Null,
              }),
            }),
          ],
          [
            {
              bitcoin_network: { regtest: null },
            },
          ]
        ),
      });
      CANISTER_IDS_MAP.set(CANISTERS_NAME.INSURANCE, fixture.canisterId);
      return fixture.actor;

    case CANISTERS_NAME.MAIN:
      fixture = await pic.setupCanister<_MAIN>({
        sender,
        idlFactory: idlFactoryMain,
        wasm,
        targetSubnetId: mainSubnetId,
        arg: IDL.encode(
          [
            IDL.Record({
              bitcoin_network: IDL.Variant({
                mainnet: IDL.Null,
                regtest: IDL.Null,
                testnet: IDL.Null,
              }),
            }),
          ],
          [
            {
              bitcoin_network: { regtest: null },
            },
          ]
        ),
      });
      CANISTER_IDS_MAP.set(CANISTERS_NAME.MAIN, fixture.canisterId);

      return fixture.actor;
    case CANISTERS_NAME.ACCOUNTS:
      fixture = await pic.setupCanister<_ACCOUNTS>({
        sender,
        idlFactory: idlFactoryAccounts,
        wasm,
        targetSubnetId: mainSubnetId,
        arg: IDL.encode(
          [
            IDL.Record({
              bitcoin_network: IDL.Variant({
                mainnet: IDL.Null,
                regtest: IDL.Null,
                testnet: IDL.Null,
              }),
            }),
          ],
          [
            {
              bitcoin_network: { regtest: null },
            },
          ]
        ),
      });
      CANISTER_IDS_MAP.set(CANISTERS_NAME.ACCOUNTS, fixture.canisterId);
      return fixture.actor;

    // -------------------------MINTER SETUP
    case CANISTERS_NAME.CKBTC_MINTER:
      let ecdsaKeyName = `master_ecdsa_public_key_${fiduciarySubnetId}`;

      console.log("🚀 ~ ecdsaKeyName:", ecdsaKeyName);
      ckbtcMinterArgs.Init.ecdsa_key_name = ecdsaKeyName;

      fixture = await pic.setupCanister<_CKBTC_MINTER>({
        targetCanisterId: CANISTER_IDS_MAP.get(CANISTERS_NAME.CKBTC_MINTER),
        sender,
        idlFactory: idlFactoryCkbtcMinter,
        wasm,
        targetSubnetId: fiduciarySubnetId,
        arg: IDL.encode([ckBTCMinterArgCandid], [ckbtcMinterArgs]),
      });
      CANISTER_IDS_MAP.set(CANISTERS_NAME.CKBTC_MINTER, fixture.canisterId);
      return fixture.actor;

    case CANISTERS_NAME.CKETH_MINTER:
      fixture = await pic.setupCanister<_CKETH_MINTER>({
        sender,
        idlFactory: idlFactoryCkethMinter,
        wasm,
        targetSubnetId: fiduciarySubnetId,
        targetCanisterId: CANISTER_IDS_MAP.get(CANISTERS_NAME.CKETH_MINTER),
        arg: IDL.encode([ckETHMinterArgCandid], [ckethMinterArgs]),
      });
      CANISTER_IDS_MAP.set(CANISTERS_NAME.CKETH_MINTER, fixture.canisterId);
      return fixture.actor;

    case CANISTERS_NAME.KYT:
      fixture = await pic.setupCanister<_KYT>({
        sender,
        idlFactory: idlFactoryKYT,
        wasm,
        targetSubnetId: fiduciarySubnetId,
        targetCanisterId: CANISTER_IDS_MAP.get(CANISTERS_NAME.KYT),
        arg: IDL.encode([KYTLifecycleArgCandid], [kytArgs]),
      });
      CANISTER_IDS_MAP.set(CANISTERS_NAME.KYT, fixture.canisterId);
      return fixture.actor;

    //---------------------------LEDGERS SETUP

    case CANISTERS_NAME.ICP_LEDGER:
      fixture = await pic.setupCanister<_ICP_LEDGER>({
        targetCanisterId: CANISTER_IDS_MAP.get(CANISTERS_NAME.ICP_LEDGER),
        sender,
        idlFactory: idlFactoryCkbtcLedger,
        wasm,
        targetSubnetId: fiduciarySubnetId,
        arg: IDL.encode([ICPLedgerCanisterPayloadCandid], [icpArgs]),
      });
      CANISTER_IDS_MAP.set(CANISTERS_NAME.ICP_LEDGER, fixture.canisterId);
      return fixture.actor;

    case CANISTERS_NAME.AEGIS_LEDGER:
      fixture = await pic.setupCanister<_AEGIS_LEDGER>({
        targetCanisterId: CANISTER_IDS_MAP.get(CANISTERS_NAME.AEGIS_LEDGER),
        sender,
        idlFactory: idlFactoryAegisLedger,
        wasm,
        targetSubnetId: fiduciarySubnetId,
        arg: IDL.encode([LedgerArgCandid], [aegisArgs]),
      });
      CANISTER_IDS_MAP.set(CANISTERS_NAME.AEGIS_LEDGER, fixture.canisterId);
      return fixture.actor;

    case CANISTERS_NAME.CKBTC_LEDGER:
      fixture = await pic.setupCanister<_CKBTC_LEDGER>({
        targetCanisterId: CANISTER_IDS_MAP.get(CANISTERS_NAME.CKBTC_LEDGER),
        sender,
        idlFactory: idlFactoryCkbtcLedger,
        wasm,
        targetSubnetId: fiduciarySubnetId,
        arg: IDL.encode([LedgerArgCandid], [ckbtcArgs]),
      });
      CANISTER_IDS_MAP.set(CANISTERS_NAME.CKBTC_LEDGER, fixture.canisterId);
      return fixture.actor;

    case CANISTERS_NAME.CKETH_LEDGER:
      fixture = await pic.setupCanister<_CKETH_LEDGER>({
        sender,
        idlFactory: idlFactoryCkethLedger,
        wasm,
        targetSubnetId: fiduciarySubnetId,
        arg: IDL.encode([LedgerArgCandid], [ckethArgs]),
      });
      CANISTER_IDS_MAP.set(CANISTERS_NAME.CKETH_LEDGER, fixture.canisterId);
      return fixture.actor;

    default:
      throw new Error(`${canisterName} Not Found`);
  }
}

export function createIdentityFromSeed(seed?: string): Identity {
  return createIdentity(seed!);
}

export async function wait(min: number) {
  await new Promise((resolve) => setTimeout(resolve, 60000 * min)); // Wait for 1 minute (60 seconds)
}
