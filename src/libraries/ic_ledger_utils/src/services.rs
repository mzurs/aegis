#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};
use ic_cdk::api::call::CallResult as Result;
use icrc_ledger_types::icrc1::account::Account;

#[derive(CandidType, Deserialize)]
pub struct FeatureFlags {
    pub icrc2: bool,
}

#[derive(CandidType, Deserialize)]
pub struct UpgradeArgs {
    pub maximum_number_of_accounts: Option<u64>,
    pub icrc1_minting_account: Option<Account>,
    pub feature_flags: Option<FeatureFlags>,
}

#[derive(CandidType, Deserialize)]
pub struct Tokens {
    pub e8s: u64,
}

#[derive(CandidType, Deserialize)]
pub struct Duration {
    pub secs: u64,
    pub nanos: u32,
}

#[derive(CandidType, Deserialize)]
pub struct ArchiveOptions {
    pub num_blocks_to_archive: u64,
    pub max_transactions_per_response: Option<u64>,
    pub trigger_threshold: u64,
    pub max_message_size_bytes: Option<u64>,
    pub cycles_for_archive_creation: Option<u64>,
    pub node_max_memory_size_bytes: Option<u64>,
    pub controller_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct InitArgs {
    pub send_whitelist: Vec<Principal>,
    pub token_symbol: Option<String>,
    pub transfer_fee: Option<Tokens>,
    pub minting_account: String,
    pub maximum_number_of_accounts: Option<u64>,
    pub accounts_overflow_trim_quantity: Option<u64>,
    pub transaction_window: Option<Duration>,
    pub max_message_size_bytes: Option<u64>,
    pub icrc1_minting_account: Option<Account>,
    pub archive_options: Option<ArchiveOptions>,
    pub initial_values: Vec<(String, Tokens)>,
    pub token_name: Option<String>,
    pub feature_flags: Option<FeatureFlags>,
}

#[derive(CandidType, Deserialize)]
pub enum LedgerCanisterPayload {
    Upgrade(Option<UpgradeArgs>),
    Init(Box<InitArgs>),
}

#[derive(CandidType, Deserialize)]
pub struct BinaryAccountBalanceArgs {
    pub account: serde_bytes::ByteBuf,
}

#[derive(CandidType, Deserialize)]
pub struct AccountBalanceArgs {
    pub account: String,
}

#[derive(CandidType, Deserialize)]
pub struct ArchiveInfo {
    pub canister_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct Archives {
    pub archives: Vec<ArchiveInfo>,
}

#[derive(CandidType, Deserialize)]
pub struct Decimals {
    pub decimals: u32,
}

#[derive(CandidType, Deserialize)]
pub enum MetadataValue {
    Int(candid::Int),
    Nat(candid::Nat),
    Blob(serde_bytes::ByteBuf),
    Text(String),
}

#[derive(CandidType, Deserialize)]
pub struct StandardRecord {
    pub url: String,
    pub name: String,
}

#[derive(CandidType, Deserialize)]
pub struct TransferArg {
    pub to: Account,
    pub fee: Option<candid::Nat>,
    pub memo: Option<[u8; 32]>,
    pub from_subaccount: Option<[u8; 32]>,
    pub created_at_time: Option<u64>,
    pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum TransferError {
    GenericError { message: String, error_code: candid::Nat },
    TemporarilyUnavailable,
    BadBurn { min_burn_amount: candid::Nat },
    Duplicate { duplicate_of: candid::Nat },
    BadFee { expected_fee: candid::Nat },
    CreatedInFuture { ledger_time: u64 },
    TooOld,
    InsufficientFunds { balance: candid::Nat },
}

#[derive(CandidType, Deserialize)]
pub enum TransferResult {
    Ok(candid::Nat),
    Err(TransferError),
}

#[derive(CandidType, Deserialize)]
pub struct AllowanceArgs {
    pub account: Account,
    pub spender: Account,
}

#[derive(CandidType, Deserialize)]
pub struct Allowance {
    pub allowance: candid::Nat,
    pub expires_at: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct ApproveArgs {
    pub fee: Option<candid::Nat>,
    pub memo: Option<[u8; 32]>,
    pub from_subaccount: Option<[u8; 32]>,
    pub created_at_time: Option<u64>,
    pub amount: candid::Nat,
    pub expected_allowance: Option<candid::Nat>,
    pub expires_at: Option<u64>,
    pub spender: Account,
}

#[derive(CandidType, Deserialize)]
pub enum ApproveError {
    GenericError { message: String, error_code: candid::Nat },
    TemporarilyUnavailable,
    Duplicate { duplicate_of: candid::Nat },
    BadFee { expected_fee: candid::Nat },
    AllowanceChanged { current_allowance: candid::Nat },
    CreatedInFuture { ledger_time: u64 },
    TooOld,
    Expired { ledger_time: u64 },
    InsufficientFunds { balance: candid::Nat },
}

#[derive(CandidType, Deserialize)]
pub enum Result1 {
    Ok(candid::Nat),
    Err(ApproveError),
}

#[derive(CandidType, Deserialize)]
pub struct TransferFromArgs {
    pub to: Account,
    pub fee: Option<candid::Nat>,
    pub spender_subaccount: Option<[u8; 32]>,
    pub from: Account,
    pub memo: Option<[u8; 32]>,
    pub created_at_time: Option<u64>,
    pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum TransferFromError {
    GenericError { message: String, error_code: candid::Nat },
    TemporarilyUnavailable,
    InsufficientAllowance { allowance: candid::Nat },
    BadBurn { min_burn_amount: candid::Nat },
    Duplicate { duplicate_of: candid::Nat },
    BadFee { expected_fee: candid::Nat },
    CreatedInFuture { ledger_time: u64 },
    TooOld,
    InsufficientFunds { balance: candid::Nat },
}

#[derive(CandidType, Deserialize)]
pub enum Result2 {
    Ok(candid::Nat),
    Err(TransferFromError),
}

#[derive(CandidType, Deserialize)]
pub struct Name {
    pub name: String,
}

#[derive(CandidType, Deserialize)]
pub struct GetBlocksArgs {
    pub start: u64,
    pub length: u64,
}

#[derive(CandidType, Deserialize)]
pub struct TimeStamp {
    pub timestamp_nanos: u64,
}

#[derive(CandidType, Deserialize)]
pub enum CandidOperation {
    Approve {
        fee: Tokens,
        from: serde_bytes::ByteBuf,
        allowance_e8s: candid::Int,
        allowance: Tokens,
        expected_allowance: Option<Tokens>,
        expires_at: Option<TimeStamp>,
        spender: serde_bytes::ByteBuf,
    },
    Burn {
        from: serde_bytes::ByteBuf,
        amount: Tokens,
        spender: Option<[u8; 32]>,
    },
    Mint {
        to: serde_bytes::ByteBuf,
        amount: Tokens,
    },
    Transfer {
        to: serde_bytes::ByteBuf,
        fee: Tokens,
        from: serde_bytes::ByteBuf,
        amount: Tokens,
        spender: Option<[u8; 32]>,
    },
}

#[derive(CandidType, Deserialize)]
pub struct CandidTransaction {
    pub memo: u64,
    pub icrc1_memo: Option<[u8; 32]>,
    pub operation: Option<CandidOperation>,
    pub created_at_time: TimeStamp,
}

#[derive(CandidType, Deserialize)]
pub struct CandidBlock {
    pub transaction: CandidTransaction,
    pub timestamp: TimeStamp,
    pub parent_hash: Option<[u8; 32]>,
}

#[derive(CandidType, Deserialize)]
pub struct BlockRange {
    pub blocks: Vec<CandidBlock>,
}

#[derive(CandidType, Deserialize)]
pub enum GetBlocksError {
    BadFirstBlockIndex { requested_index: u64, first_valid_index: u64 },
    Other { error_message: String, error_code: u64 },
}

#[derive(CandidType, Deserialize)]
pub enum Result3 {
    Ok(BlockRange),
    Err(GetBlocksError),
}

candid::define_function!(pub ArchivedBlocksRangeCallback : (GetBlocksArgs) -> (
Result3,
) query);
#[derive(CandidType, Deserialize)]
pub struct ArchivedBlocksRange {
    pub callback: ArchivedBlocksRangeCallback,
    pub start: u64,
    pub length: u64,
}

#[derive(CandidType, Deserialize)]
pub struct QueryBlocksResponse {
    pub certificate: Option<[u8; 32]>,
    pub blocks: Vec<CandidBlock>,
    pub chain_length: u64,
    pub first_block_index: u64,
    pub archived_blocks: Vec<ArchivedBlocksRange>,
}

#[derive(CandidType, Deserialize)]
pub enum Result4 {
    Ok(Vec<serde_bytes::ByteBuf>),
    Err(GetBlocksError),
}

candid::define_function!(pub ArchivedEncodedBlocksRangeCallback : (
GetBlocksArgs,
) -> (Result4) query);
#[derive(CandidType, Deserialize)]
pub struct ArchivedEncodedBlocksRange {
    pub callback: ArchivedEncodedBlocksRangeCallback,
    pub start: u64,
    pub length: u64,
}

#[derive(CandidType, Deserialize)]
pub struct QueryEncodedBlocksResponse {
    pub certificate: Option<[u8; 32]>,
    pub blocks: Vec<serde_bytes::ByteBuf>,
    pub chain_length: u64,
    pub first_block_index: u64,
    pub archived_blocks: Vec<ArchivedEncodedBlocksRange>,
}

#[derive(CandidType, Deserialize)]
pub struct SendArgs {
    pub to: String,
    pub fee: Tokens,
    pub memo: u64,
    pub from_subaccount: Option<[u8; 32]>,
    pub created_at_time: Option<TimeStamp>,
    pub amount: Tokens,
}

#[derive(CandidType, Deserialize)]
pub struct Symbol {
    pub symbol: String,
}

#[derive(CandidType, Deserialize)]
pub struct TransferArgs {
    pub to: serde_bytes::ByteBuf,
    pub fee: Tokens,
    pub memo: u64,
    pub from_subaccount: Option<[u8; 32]>,
    pub created_at_time: Option<TimeStamp>,
    pub amount: Tokens,
}

#[derive(CandidType, Deserialize)]
pub enum TransferError1 {
    TxTooOld { allowed_window_nanos: u64 },
    BadFee { expected_fee: Tokens },
    TxDuplicate { duplicate_of: u64 },
    TxCreatedInFuture,
    InsufficientFunds { balance: Tokens },
}

#[derive(CandidType, Deserialize)]
pub enum Result5 {
    Ok(u64),
    Err(TransferError1),
}

#[derive(CandidType, Deserialize)]
pub struct TransferFeeArg {}

#[derive(CandidType, Deserialize)]
pub struct TransferFee {
    pub transfer_fee: Tokens,
}
