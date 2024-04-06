import { IDL } from "@dfinity/candid";
import { Principal } from "@dfinity/principal";

const MetadataValueLedger = IDL.Variant({
  Int: IDL.Int,
  Nat: IDL.Nat,
  Blob: IDL.Vec(IDL.Nat8),
  Text: IDL.Text,
});
const Subaccount = IDL.Vec(IDL.Nat8);
const Account = IDL.Record({
  owner: IDL.Principal,
  subaccount: IDL.Opt(Subaccount),
});
const ChangeFeeCollector = IDL.Variant({
  SetTo: Account,
  Unset: IDL.Null,
});
const FeatureFlags = IDL.Record({ icrc2: IDL.Bool });
const UpgradeArgsLedger = IDL.Record({
  token_symbol: IDL.Opt(IDL.Text),
  transfer_fee: IDL.Opt(IDL.Nat),
  metadata: IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, MetadataValueLedger))),
  maximum_number_of_accounts: IDL.Opt(IDL.Nat64),
  accounts_overflow_trim_quantity: IDL.Opt(IDL.Nat64),
  change_fee_collector: IDL.Opt(ChangeFeeCollector),
  max_memo_length: IDL.Opt(IDL.Nat16),
  token_name: IDL.Opt(IDL.Text),
  feature_flags: IDL.Opt(FeatureFlags),
});
const InitArgsLedger = IDL.Record({
  decimals: IDL.Opt(IDL.Nat8),
  token_symbol: IDL.Text,
  transfer_fee: IDL.Nat,
  metadata: IDL.Vec(IDL.Tuple(IDL.Text, MetadataValueLedger)),
  minting_account: Account,
  initial_balances: IDL.Vec(IDL.Tuple(Account, IDL.Nat)),
  maximum_number_of_accounts: IDL.Opt(IDL.Nat64),
  accounts_overflow_trim_quantity: IDL.Opt(IDL.Nat64),
  fee_collector_account: IDL.Opt(Account),
  archive_options: IDL.Record({
    num_blocks_to_archive: IDL.Nat64,
    max_transactions_per_response: IDL.Opt(IDL.Nat64),
    trigger_threshold: IDL.Nat64,
    more_controller_ids: IDL.Opt(IDL.Vec(IDL.Principal)),
    max_message_size_bytes: IDL.Opt(IDL.Nat64),
    cycles_for_archive_creation: IDL.Opt(IDL.Nat64),
    node_max_memory_size_bytes: IDL.Opt(IDL.Nat64),
    controller_id: IDL.Principal,
  }),
  max_memo_length: IDL.Opt(IDL.Nat16),
  token_name: IDL.Text,
  feature_flags: IDL.Opt(FeatureFlags),
});

export const LedgerArgCandid = IDL.Variant({
  Upgrade: IDL.Opt(UpgradeArgsLedger),
  Init: InitArgsLedger,
});

export type LedgerArgJS = { Init: InitArgsJS };

export interface FeatureFlags {
  icrc2: boolean;
}
export type Subaccount = Uint8Array | number[];

export interface Account {
  owner: Principal;
  subaccount: [] | [Subaccount];
}
export type MetadataValueLedger =
  | { Int: bigint }
  | { Nat: bigint }
  | { Blob: Uint8Array | number[] }
  | { Text: string };
export interface InitArgsJS {
  decimals: [] | [number];
  token_symbol: string;
  transfer_fee: bigint;
  metadata: Array<[string, MetadataValueLedger]>;
  minting_account: Account;
  initial_balances: Array<[Account, bigint]>;
  maximum_number_of_accounts: [] | [bigint];
  accounts_overflow_trim_quantity: [] | [bigint];
  fee_collector_account: [] | [Account];
  archive_options: {
    num_blocks_to_archive: bigint;
    max_transactions_per_response: [] | [bigint];
    trigger_threshold: bigint;
    more_controller_ids: [] | [Array<Principal>];
    max_message_size_bytes: [] | [bigint];
    cycles_for_archive_creation: [] | [bigint];
    node_max_memory_size_bytes: [] | [bigint];
    controller_id: Principal;
  };
  max_memo_length: [] | [number];
  token_name: string;
  feature_flags: [] | [FeatureFlags];
}

//---------------------MINTER--------------------------------------

const Mode = IDL.Variant({
  RestrictedTo: IDL.Vec(IDL.Principal),
  DepositsRestrictedTo: IDL.Vec(IDL.Principal),
  ReadOnly: IDL.Null,
  GeneralAvailability: IDL.Null,
});
const UpgradeArgsMinter = IDL.Record({
  kyt_principal: IDL.Opt(IDL.Principal),
  mode: IDL.Opt(Mode),
  retrieve_btc_min_amount: IDL.Opt(IDL.Nat64),
  max_time_in_queue_nanos: IDL.Opt(IDL.Nat64),
  min_confirmations: IDL.Opt(IDL.Nat32),
  kyt_fee: IDL.Opt(IDL.Nat64),
});
const BtcNetwork = IDL.Variant({
  Mainnet: IDL.Null,
  Regtest: IDL.Null,
  Testnet: IDL.Null,
});
const InitArgsMinter = IDL.Record({
  kyt_principal: IDL.Opt(IDL.Principal),
  ecdsa_key_name: IDL.Text,
  mode: Mode,
  retrieve_btc_min_amount: IDL.Nat64,
  ledger_id: IDL.Principal,
  max_time_in_queue_nanos: IDL.Nat64,
  btc_network: BtcNetwork,
  min_confirmations: IDL.Opt(IDL.Nat32),
  kyt_fee: IDL.Opt(IDL.Nat64),
});

export const ckBTCMinterArgCandid = IDL.Variant({
  Upgrade: IDL.Opt(UpgradeArgsMinter),
  Init: InitArgsMinter,
});

export interface ckBTCMinterInitArgsJS {
  kyt_principal: [] | [Principal];
  ecdsa_key_name: string;
  mode: Mode;
  retrieve_btc_min_amount: bigint;
  ledger_id: Principal;
  max_time_in_queue_nanos: bigint;
  btc_network: BtcNetwork;
  min_confirmations: [] | [number];
  kyt_fee: [] | [bigint];
}
export type ckBTCMinterArgJS = { Init: ckBTCMinterInitArgsJS };
export type Mode =
  | { RestrictedTo: Array<Principal> }
  | { DepositsRestrictedTo: Array<Principal> }
  | { ReadOnly: null }
  | { GeneralAvailability: null };
export type BtcNetwork =
  | { Mainnet: null }
  | { Regtest: null }
  | { Testnet: null };

const BlockTag = IDL.Variant({
  Safe: IDL.Null,
  Finalized: IDL.Null,
  Latest: IDL.Null,
});
const ckETHMinterUpgradeArg = IDL.Record({
  next_transaction_nonce: IDL.Opt(IDL.Nat),
  ethereum_contract_address: IDL.Opt(IDL.Text),
  minimum_withdrawal_amount: IDL.Opt(IDL.Nat),
  ethereum_block_height: IDL.Opt(BlockTag),
});
const EthereumNetwork = IDL.Variant({
  Mainnet: IDL.Null,
  Sepolia: IDL.Null,
});
const ckethMinterInitArg = IDL.Record({
  ethereum_network: EthereumNetwork,
  last_scraped_block_number: IDL.Nat,
  ecdsa_key_name: IDL.Text,
  next_transaction_nonce: IDL.Nat,
  ledger_id: IDL.Principal,
  ethereum_contract_address: IDL.Opt(IDL.Text),
  minimum_withdrawal_amount: IDL.Nat,
  ethereum_block_height: BlockTag,
});
export const ckETHMinterArgCandid = IDL.Variant({
  UpgradeArg: ckETHMinterUpgradeArg,
  InitArg: ckethMinterInitArg,
});

export interface ckETHMinterInitArg {
  ethereum_network: EthereumNetwork;
  last_scraped_block_number: bigint;
  ecdsa_key_name: string;
  next_transaction_nonce: bigint;
  ledger_id: Principal;
  ethereum_contract_address: [] | [string];
  minimum_withdrawal_amount: bigint;
  ethereum_block_height: BlockTag;
}
export type ckETHMinterArgJS = {
  InitArg: ckETHMinterInitArg;
};
export type EthereumNetwork = { Mainnet: null } | { Sepolia: null };
export type BlockTag = { Safe: null } | { Finalized: null } | { Latest: null };

export interface Tokens {
  e8s: bigint;
}

export interface ICPInitArgsJS {
  send_whitelist: Array<Principal>;
  token_symbol: [] | [string];
  transfer_fee: [] | [Tokens];
  minting_account: string;
  maximum_number_of_accounts: [] | [bigint];
  accounts_overflow_trim_quantity: [] | [bigint];
  transaction_window: [] | [Duration];
  max_message_size_bytes: [] | [bigint];
  icrc1_minting_account: [] | [Account];
  archive_options: [] | [ArchiveOptions];
  initial_values: Array<[string, Tokens]>;
  token_name: [] | [string];
  feature_flags: [] | [FeatureFlags];
}
export type ICPLedgerCanisterPayloadJS = { Init: ICPInitArgsJS };
export interface ArchiveOptions {
  num_blocks_to_archive: bigint;
  max_transactions_per_response: [] | [bigint];
  trigger_threshold: bigint;
  more_controller_ids: [] | [Array<Principal>];
  max_message_size_bytes: [] | [bigint];
  cycles_for_archive_creation: [] | [bigint];
  node_max_memory_size_bytes: [] | [bigint];
  controller_id: Principal;
}
export interface Duration {
  secs: bigint;
  nanos: number;
}

const UpgradeArgs = IDL.Record({
  maximum_number_of_accounts: IDL.Opt(IDL.Nat64),
  icrc1_minting_account: IDL.Opt(Account),
  feature_flags: IDL.Opt(FeatureFlags),
});
const Tokens = IDL.Record({ e8s: IDL.Nat64 });
const TextAccountIdentifier = IDL.Text;
const Duration = IDL.Record({ secs: IDL.Nat64, nanos: IDL.Nat32 });
const ArchiveOptions = IDL.Record({
  num_blocks_to_archive: IDL.Nat64,
  max_transactions_per_response: IDL.Opt(IDL.Nat64),
  trigger_threshold: IDL.Nat64,
  more_controller_ids: IDL.Opt(IDL.Vec(IDL.Principal)),
  max_message_size_bytes: IDL.Opt(IDL.Nat64),
  cycles_for_archive_creation: IDL.Opt(IDL.Nat64),
  node_max_memory_size_bytes: IDL.Opt(IDL.Nat64),
  controller_id: IDL.Principal,
});
const InitArgs = IDL.Record({
  send_whitelist: IDL.Vec(IDL.Principal),
  token_symbol: IDL.Opt(IDL.Text),
  transfer_fee: IDL.Opt(Tokens),
  minting_account: TextAccountIdentifier,
  maximum_number_of_accounts: IDL.Opt(IDL.Nat64),
  accounts_overflow_trim_quantity: IDL.Opt(IDL.Nat64),
  transaction_window: IDL.Opt(Duration),
  max_message_size_bytes: IDL.Opt(IDL.Nat64),
  icrc1_minting_account: IDL.Opt(Account),
  archive_options: IDL.Opt(ArchiveOptions),
  initial_values: IDL.Vec(IDL.Tuple(TextAccountIdentifier, Tokens)),
  token_name: IDL.Opt(IDL.Text),
  feature_flags: IDL.Opt(FeatureFlags),
});

export const ICPLedgerCanisterPayloadCandid = IDL.Variant({
  Upgrade: IDL.Opt(UpgradeArgs),
  Init: InitArgs,
});
export interface KYTInitArg {
  maintainers: Array<Principal>;
  mode: KYTMode;
  minter_id: Principal;
}
export type KYTLifecycleArgJS = { InitArg: KYTInitArg };
export type KYTMode =
  | { RejectAll: null }
  | { Normal: null }
  | { AcceptAll: null };
const KYTMode = IDL.Variant({
  RejectAll: IDL.Null,
  Normal: IDL.Null,
  AcceptAll: IDL.Null,
});

const InitArg = IDL.Record({
  maintainers: IDL.Vec(IDL.Principal),
  mode: KYTMode,
  minter_id: IDL.Principal,
});
export const KYTLifecycleArgCandid = IDL.Variant({
  InitArg: InitArg,
});
