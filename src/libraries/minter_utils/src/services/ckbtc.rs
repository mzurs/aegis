// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};
use ic_cdk::api::call::CallResult as Result;
use icrc_ledger_types::icrc1::account::Account;

#[derive(CandidType, Deserialize)]
pub enum Mode {
    RestrictedTo(Vec<Principal>),
    DepositsRestrictedTo(Vec<Principal>),
    ReadOnly,
    GeneralAvailability,
}

#[derive(CandidType, Deserialize)]
pub struct UpgradeArgs {
    pub kyt_principal: Option<Principal>,
    pub mode: Option<Mode>,
    pub retrieve_btc_min_amount: Option<u64>,
    pub max_time_in_queue_nanos: Option<u64>,
    pub min_confirmations: Option<u32>,
    pub kyt_fee: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub enum BtcNetwork {
    Mainnet,
    Regtest,
    Testnet,
}

#[derive(CandidType, Deserialize)]
pub struct InitArgs {
    pub kyt_principal: Option<Principal>,
    pub ecdsa_key_name: String,
    pub mode: Mode,
    pub retrieve_btc_min_amount: u64,
    pub ledger_id: Principal,
    pub max_time_in_queue_nanos: u64,
    pub btc_network: BtcNetwork,
    pub min_confirmations: Option<u32>,
    pub kyt_fee: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub enum MinterArg {
    Upgrade(Option<UpgradeArgs>),
    Init(InitArgs),
}

#[derive(CandidType, Deserialize)]
pub struct EstimateWithdrawalFeeArg {
    pub amount: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct EstimateWithdrawalFeeRet {
    pub minter_fee: u64,
    pub bitcoin_fee: u64,
}

#[derive(CandidType, Deserialize)]
pub struct GetBtcAddressArg {
    pub owner: Option<Principal>,
    pub subaccount: Option<[u8; 32]>,
}

#[derive(CandidType, Deserialize)]
pub enum CanisterStatusType {
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "stopping")]
    Stopping,
    #[serde(rename = "running")]
    Running,
}

#[derive(CandidType, Deserialize)]
pub struct DefiniteCanisterSettings {
    pub freezing_threshold: candid::Nat,
    pub controllers: Vec<Principal>,
    pub memory_allocation: candid::Nat,
    pub compute_allocation: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct QueryStats {
    pub response_payload_bytes_total: candid::Nat,
    pub num_instructions_total: candid::Nat,
    pub num_calls_total: candid::Nat,
    pub request_payload_bytes_total: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct CanisterStatusResponse {
    pub status: CanisterStatusType,
    pub memory_size: candid::Nat,
    pub cycles: candid::Nat,
    pub settings: DefiniteCanisterSettings,
    pub query_stats: QueryStats,
    pub idle_cycles_burned_per_day: candid::Nat,
    pub module_hash: Option<[u8; 32]>,
}

#[derive(CandidType, Deserialize)]
pub struct GetEventsArg {
    pub start: u64,
    pub length: u64,
}

#[derive(CandidType, Deserialize)]
pub struct UtxoOutpoint {
    pub txid: serde_bytes::ByteBuf,
    pub vout: u32,
}

#[derive(CandidType, Deserialize)]
pub struct Utxo {
    pub height: u32,
    pub value: u64,
    pub outpoint: UtxoOutpoint,
}

#[derive(CandidType, Deserialize)]
pub enum ReimbursementReason {
    CallFailed,
    TaintedDestination { kyt_fee: u64, kyt_provider: Principal },
}

#[derive(CandidType, Deserialize)]
pub struct EventSentTransactionChangeOutputInner {
    pub value: u64,
    pub vout: u32,
}

#[derive(CandidType, Deserialize)]
pub enum BitcoinAddress {
    #[serde(rename = "p2wsh_v0")]
    P2WshV0(serde_bytes::ByteBuf),
    #[serde(rename = "p2tr_v1")]
    P2TrV1(serde_bytes::ByteBuf),
    #[serde(rename = "p2sh")]
    P2Sh(serde_bytes::ByteBuf),
    #[serde(rename = "p2wpkh_v0")]
    P2WpkhV0(serde_bytes::ByteBuf),
    #[serde(rename = "p2pkh")]
    P2Pkh(serde_bytes::ByteBuf),
}

#[derive(CandidType, Deserialize)]
pub struct EventReplacedTransactionChangeOutput {
    pub value: u64,
    pub vout: u32,
}

#[derive(CandidType, Deserialize)]
pub enum Event {
    #[serde(rename = "received_utxos")]
    ReceivedUtxos {
        to_account: Account,
        mint_txid: Option<u64>,
        utxos: Vec<Utxo>,
    },
    #[serde(rename = "schedule_deposit_reimbursement")]
    ScheduleDepositReimbursement {
        burn_block_index: u64,
        account: Account,
        amount: u64,
        reason: ReimbursementReason,
    },
    #[serde(rename = "sent_transaction")]
    SentTransaction {
        fee: Option<u64>,
        change_output: Option<EventSentTransactionChangeOutputInner>,
        txid: serde_bytes::ByteBuf,
        utxos: Vec<Utxo>,
        requests: Vec<u64>,
        submitted_at: u64,
    },
    #[serde(rename = "distributed_kyt_fee")]
    DistributedKytFee {
        block_index: u64,
        amount: u64,
        kyt_provider: Principal,
    },
    #[serde(rename = "init")]
    Init(InitArgs),
    #[serde(rename = "upgrade")]
    Upgrade(UpgradeArgs),
    #[serde(rename = "retrieve_btc_kyt_failed")]
    RetrieveBtcKytFailed {
        block_index: u64,
        owner: Principal,
        uuid: String,
        address: String,
        amount: u64,
        kyt_provider: Principal,
    },
    #[serde(rename = "accepted_retrieve_btc_request")]
    AcceptedRetrieveBtcRequest {
        received_at: u64,
        block_index: u64,
        address: BitcoinAddress,
        reimbursement_account: Option<Account>,
        amount: u64,
        kyt_provider: Option<Principal>,
    },
    #[serde(rename = "checked_utxo")]
    CheckedUtxo {
        clean: bool,
        utxo: Utxo,
        uuid: String,
        kyt_provider: Option<Principal>,
    },
    #[serde(rename = "removed_retrieve_btc_request")]
    RemovedRetrieveBtcRequest { block_index: u64 },
    #[serde(rename = "confirmed_transaction")]
    ConfirmedTransaction { txid: serde_bytes::ByteBuf },
    #[serde(rename = "replaced_transaction")]
    ReplacedTransaction {
        fee: u64,
        change_output: EventReplacedTransactionChangeOutput,
        old_txid: serde_bytes::ByteBuf,
        new_txid: serde_bytes::ByteBuf,
        submitted_at: u64,
    },
    #[serde(rename = "ignored_utxo")]
    IgnoredUtxo { utxo: Utxo },
    #[serde(rename = "reimbursed_failed_deposit")]
    ReimbursedFailedDeposit { burn_block_index: u64, mint_block_index: u64 },
}

#[derive(CandidType, Deserialize)]
pub struct GetKnownUtxosArg {
    pub owner: Option<Principal>,
    pub subaccount: Option<[u8; 32]>,
}

#[derive(CandidType, Deserialize)]
pub struct MinterInfo {
    pub retrieve_btc_min_amount: u64,
    pub min_confirmations: u32,
    pub kyt_fee: u64,
}

#[derive(CandidType, Deserialize)]
pub struct RetrieveBtcArgs {
    pub address: String,
    pub amount: u64,
}

#[derive(CandidType, Deserialize)]
pub struct RetrieveBtcOk {
    pub block_index: u64,
}

#[derive(CandidType, Deserialize)]
pub enum RetrieveBtcError {
    MalformedAddress(String),
    GenericError { error_message: String, error_code: u64 },
    TemporarilyUnavailable(String),
    AlreadyProcessing,
    AmountTooLow(u64),
    InsufficientFunds { balance: u64 },
}

#[derive(CandidType, Deserialize)]
pub enum RetrieveBtcRet {
    Ok(RetrieveBtcOk),
    Err(RetrieveBtcError),
}

#[derive(CandidType, Deserialize)]
pub struct RetrieveBtcStatusArg {
    pub block_index: u64,
}

#[derive(CandidType, Deserialize)]
pub enum RetrieveBtcStatus {
    Signing,
    Confirmed { txid: serde_bytes::ByteBuf },
    Sending { txid: serde_bytes::ByteBuf },
    AmountTooLow,
    Unknown,
    Submitted { txid: serde_bytes::ByteBuf },
    Pending,
}

#[derive(CandidType, Deserialize)]
pub struct RetrieveBtcStatusV2Arg {
    pub block_index: u64,
}

#[derive(CandidType, Deserialize)]
pub struct ReimbursementRequest {
    pub account: Account,
    pub amount: u64,
    pub reason: ReimbursementReason,
}

#[derive(CandidType, Deserialize)]
pub struct ReimbursedDeposit {
    pub account: Account,
    pub mint_block_index: u64,
    pub amount: u64,
    pub reason: ReimbursementReason,
}

#[derive(CandidType, Deserialize)]
pub enum RetrieveBtcStatusV2 {
    Signing,
    Confirmed { txid: serde_bytes::ByteBuf },
    Sending { txid: serde_bytes::ByteBuf },
    AmountTooLow,
    WillReimburse(ReimbursementRequest),
    Unknown,
    Submitted { txid: serde_bytes::ByteBuf },
    Reimbursed(ReimbursedDeposit),
    Pending,
}

#[derive(CandidType, Deserialize)]
pub struct RetrieveBtcStatusV2ByAccountRetItem {
    pub block_index: u64,
    pub status_v2: Option<RetrieveBtcStatusV2>,
}

#[derive(CandidType, Deserialize)]
pub struct RetrieveBtcWithApprovalArgs {
    pub from_subaccount: Option<[u8; 32]>,
    pub address: String,
    pub amount: u64,
}

#[derive(CandidType, Deserialize)]
pub enum RetrieveBtcWithApprovalError {
    MalformedAddress(String),
    GenericError { error_message: String, error_code: u64 },
    TemporarilyUnavailable(String),
    InsufficientAllowance { allowance: u64 },
    AlreadyProcessing,
    AmountTooLow(u64),
    InsufficientFunds { balance: u64 },
}

#[derive(CandidType, Deserialize)]
pub enum RetrieveBtcWithApprovalRet {
    Ok(RetrieveBtcOk),
    Err(RetrieveBtcWithApprovalError),
}

#[derive(CandidType, Deserialize)]
pub struct UpdateBalanceArg {
    pub owner: Option<Principal>,
    pub subaccount: Option<[u8; 32]>,
}

#[derive(CandidType, Deserialize)]
pub enum UtxoStatus {
    ValueTooSmall(Utxo),
    Tainted(Utxo),
    Minted {
        minted_amount: u64,
        block_index: u64,
        utxo: Utxo,
    },
    Checked(Utxo),
}

#[derive(CandidType, Deserialize)]
pub struct PendingUtxoOutpoint {
    pub txid: serde_bytes::ByteBuf,
    pub vout: u32,
}

#[derive(CandidType, Deserialize)]
pub struct PendingUtxo {
    pub confirmations: u32,
    pub value: u64,
    pub outpoint: PendingUtxoOutpoint,
}

#[derive(CandidType, Deserialize)]
pub enum UpdateBalanceError {
    GenericError {
        error_message: String,
        error_code: u64,
    },
    TemporarilyUnavailable(String),
    AlreadyProcessing,
    NoNewUtxos {
        required_confirmations: u32,
        pending_utxos: Option<Vec<PendingUtxo>>,
        current_confirmations: Option<u32>,
    },
}

#[derive(CandidType, Deserialize)]
pub enum UpdateBalanceRet {
    Ok(Vec<UtxoStatus>),
    Err(UpdateBalanceError),
}
