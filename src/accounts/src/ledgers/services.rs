pub mod minter {
    // This is an experimental feature to generate Rust binding from Candid.
    // You may want to manually adjust some of the types.
    #![allow(dead_code, unused_imports)]
    use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};
    use ic_cdk::api::call::CallResult as Result;

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
        pub subaccount: Option<serde_bytes::ByteBuf>,
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
        pub module_hash: Option<serde_bytes::ByteBuf>,
    }

    #[derive(CandidType, Deserialize)]
    pub struct GetEventsArg {
        pub start: u64,
        pub length: u64,
    }

    #[derive(CandidType, Deserialize)]
    pub struct Account {
        pub owner: Principal,
        pub subaccount: Option<serde_bytes::ByteBuf>,
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
        TaintedDestination {
            kyt_fee: u64,
            kyt_provider: Principal,
        },
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
        ReimbursedFailedDeposit {
            burn_block_index: u64,
            mint_block_index: u64,
        },
    }

    #[derive(CandidType, Deserialize)]
    pub struct GetKnownUtxosArg {
        pub owner: Option<Principal>,
        pub subaccount: Option<serde_bytes::ByteBuf>,
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
        GenericError {
            error_message: String,
            error_code: u64,
        },
        TemporarilyUnavailable(String),
        AlreadyProcessing,
        AmountTooLow(u64),
        InsufficientFunds {
            balance: u64,
        },
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
        pub from_subaccount: Option<serde_bytes::ByteBuf>,
        pub address: String,
        pub amount: u64,
    }

    #[derive(CandidType, Deserialize)]
    pub enum RetrieveBtcWithApprovalError {
        MalformedAddress(String),
        GenericError {
            error_message: String,
            error_code: u64,
        },
        TemporarilyUnavailable(String),
        InsufficientAllowance {
            allowance: u64,
        },
        AlreadyProcessing,
        AmountTooLow(u64),
        InsufficientFunds {
            balance: u64,
        },
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

    pub struct Service(pub Principal);

    impl Service {
        pub async fn estimate_withdrawal_fee(
            &self,
            arg0: EstimateWithdrawalFeeArg,
        ) -> Result<(EstimateWithdrawalFeeRet,)> {
            ic_cdk::call(self.0, "estimate_withdrawal_fee", (arg0,)).await
        }
        pub async fn get_btc_address(&self, arg0: GetBtcAddressArg) -> Result<(String,)> {
            ic_cdk::call(self.0, "get_btc_address", (arg0,)).await
        }
        pub async fn get_canister_status(&self) -> Result<(CanisterStatusResponse,)> {
            ic_cdk::call(self.0, "get_canister_status", ()).await
        }
        pub async fn get_deposit_fee(&self) -> Result<(u64,)> {
            ic_cdk::call(self.0, "get_deposit_fee", ()).await
        }
        pub async fn get_events(&self, arg0: GetEventsArg) -> Result<(Vec<Event>,)> {
            ic_cdk::call(self.0, "get_events", (arg0,)).await
        }
        pub async fn get_known_utxos(&self, arg0: GetKnownUtxosArg) -> Result<(Vec<Utxo>,)> {
            ic_cdk::call(self.0, "get_known_utxos", (arg0,)).await
        }
        pub async fn get_minter_info(&self) -> Result<(MinterInfo,)> {
            ic_cdk::call(self.0, "get_minter_info", ()).await
        }
        pub async fn get_withdrawal_account(&self) -> Result<(Account,)> {
            ic_cdk::call(self.0, "get_withdrawal_account", ()).await
        }
        pub async fn retrieve_btc(&self, arg0: RetrieveBtcArgs) -> Result<(RetrieveBtcRet,)> {
            ic_cdk::call(self.0, "retrieve_btc", (arg0,)).await
        }
        pub async fn retrieve_btc_status(
            &self,
            arg0: RetrieveBtcStatusArg,
        ) -> Result<(RetrieveBtcStatus,)> {
            ic_cdk::call(self.0, "retrieve_btc_status", (arg0,)).await
        }
        pub async fn retrieve_btc_status_v_2(
            &self,
            arg0: RetrieveBtcStatusV2Arg,
        ) -> Result<(RetrieveBtcStatusV2,)> {
            ic_cdk::call(self.0, "retrieve_btc_status_v2", (arg0,)).await
        }
        pub async fn retrieve_btc_status_v_2_by_account(
            &self,
            arg0: Option<Account>,
        ) -> Result<(Vec<RetrieveBtcStatusV2ByAccountRetItem>,)> {
            ic_cdk::call(self.0, "retrieve_btc_status_v2_by_account", (arg0,)).await
        }
        pub async fn retrieve_btc_with_approval(
            &self,
            arg0: RetrieveBtcWithApprovalArgs,
        ) -> Result<(RetrieveBtcWithApprovalRet,)> {
            ic_cdk::call(self.0, "retrieve_btc_with_approval", (arg0,)).await
        }
        pub async fn update_balance(&self, arg0: UpdateBalanceArg) -> Result<(UpdateBalanceRet,)> {
            ic_cdk::call(self.0, "update_balance", (arg0,)).await
        }
    }
}

pub mod ledger {
    // This is an experimental feature to generate Rust binding from Candid.
    // You may want to manually adjust some of the types.
    #![allow(dead_code, unused_imports)]
    use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};
    use ic_cdk::api::call::CallResult as Result;

    #[derive(CandidType, Deserialize)]
    pub struct Account {
        pub owner: Principal,
        pub subaccount: Option<serde_bytes::ByteBuf>,
    }

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
        Init(InitArgs),
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
        pub memo: Option<serde_bytes::ByteBuf>,
        pub from_subaccount: Option<serde_bytes::ByteBuf>,
        pub created_at_time: Option<u64>,
        pub amount: candid::Nat,
    }

    #[derive(CandidType, Deserialize)]
    pub enum TransferError {
        GenericError {
            message: String,
            error_code: candid::Nat,
        },
        TemporarilyUnavailable,
        BadBurn {
            min_burn_amount: candid::Nat,
        },
        Duplicate {
            duplicate_of: candid::Nat,
        },
        BadFee {
            expected_fee: candid::Nat,
        },
        CreatedInFuture {
            ledger_time: u64,
        },
        TooOld,
        InsufficientFunds {
            balance: candid::Nat,
        },
    }

    #[derive(CandidType, Deserialize)]
    pub enum Result_ {
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
        pub memo: Option<serde_bytes::ByteBuf>,
        pub from_subaccount: Option<serde_bytes::ByteBuf>,
        pub created_at_time: Option<u64>,
        pub amount: candid::Nat,
        pub expected_allowance: Option<candid::Nat>,
        pub expires_at: Option<u64>,
        pub spender: Account,
    }

    #[derive(CandidType, Deserialize)]
    pub enum ApproveError {
        GenericError {
            message: String,
            error_code: candid::Nat,
        },
        TemporarilyUnavailable,
        Duplicate {
            duplicate_of: candid::Nat,
        },
        BadFee {
            expected_fee: candid::Nat,
        },
        AllowanceChanged {
            current_allowance: candid::Nat,
        },
        CreatedInFuture {
            ledger_time: u64,
        },
        TooOld,
        Expired {
            ledger_time: u64,
        },
        InsufficientFunds {
            balance: candid::Nat,
        },
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
        pub spender_subaccount: Option<serde_bytes::ByteBuf>,
        pub from: Account,
        pub memo: Option<serde_bytes::ByteBuf>,
        pub created_at_time: Option<u64>,
        pub amount: candid::Nat,
    }

    #[derive(CandidType, Deserialize)]
    pub enum TransferFromError {
        GenericError {
            message: String,
            error_code: candid::Nat,
        },
        TemporarilyUnavailable,
        InsufficientAllowance {
            allowance: candid::Nat,
        },
        BadBurn {
            min_burn_amount: candid::Nat,
        },
        Duplicate {
            duplicate_of: candid::Nat,
        },
        BadFee {
            expected_fee: candid::Nat,
        },
        CreatedInFuture {
            ledger_time: u64,
        },
        TooOld,
        InsufficientFunds {
            balance: candid::Nat,
        },
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
            spender: Option<serde_bytes::ByteBuf>,
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
            spender: Option<serde_bytes::ByteBuf>,
        },
    }

    #[derive(CandidType, Deserialize)]
    pub struct CandidTransaction {
        pub memo: u64,
        pub icrc1_memo: Option<serde_bytes::ByteBuf>,
        pub operation: Option<CandidOperation>,
        pub created_at_time: TimeStamp,
    }

    #[derive(CandidType, Deserialize)]
    pub struct CandidBlock {
        pub transaction: CandidTransaction,
        pub timestamp: TimeStamp,
        pub parent_hash: Option<serde_bytes::ByteBuf>,
    }

    #[derive(CandidType, Deserialize)]
    pub struct BlockRange {
        pub blocks: Vec<CandidBlock>,
    }

    #[derive(CandidType, Deserialize)]
    pub enum GetBlocksError {
        BadFirstBlockIndex {
            requested_index: u64,
            first_valid_index: u64,
        },
        Other {
            error_message: String,
            error_code: u64,
        },
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
        pub certificate: Option<serde_bytes::ByteBuf>,
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
        pub certificate: Option<serde_bytes::ByteBuf>,
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
        pub from_subaccount: Option<serde_bytes::ByteBuf>,
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
        pub from_subaccount: Option<serde_bytes::ByteBuf>,
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

    pub struct Service(pub Principal);
    impl Service {
        pub async fn account_balance(&self, arg0: BinaryAccountBalanceArgs) -> Result<(Tokens,)> {
            ic_cdk::call(self.0, "account_balance", (arg0,)).await
        }
        pub async fn account_balance_dfx(&self, arg0: AccountBalanceArgs) -> Result<(Tokens,)> {
            ic_cdk::call(self.0, "account_balance_dfx", (arg0,)).await
        }
        pub async fn account_identifier(&self, arg0: Account) -> Result<(serde_bytes::ByteBuf,)> {
            ic_cdk::call(self.0, "account_identifier", (arg0,)).await
        }
        pub async fn archives(&self) -> Result<(Archives,)> {
            ic_cdk::call(self.0, "archives", ()).await
        }
        pub async fn decimals(&self) -> Result<(Decimals,)> {
            ic_cdk::call(self.0, "decimals", ()).await
        }
        pub async fn icrc_1_balance_of(&self, arg0: Account) -> Result<(candid::Nat,)> {
            ic_cdk::call(self.0, "icrc1_balance_of", (arg0,)).await
        }
        pub async fn icrc_1_decimals(&self) -> Result<(u8,)> {
            ic_cdk::call(self.0, "icrc1_decimals", ()).await
        }
        pub async fn icrc_1_fee(&self) -> Result<(candid::Nat,)> {
            ic_cdk::call(self.0, "icrc1_fee", ()).await
        }
        pub async fn icrc_1_metadata(&self) -> Result<(Vec<(String, MetadataValue)>,)> {
            ic_cdk::call(self.0, "icrc1_metadata", ()).await
        }
        pub async fn icrc_1_minting_account(&self) -> Result<(Option<Account>,)> {
            ic_cdk::call(self.0, "icrc1_minting_account", ()).await
        }
        pub async fn icrc_1_name(&self) -> Result<(String,)> {
            ic_cdk::call(self.0, "icrc1_name", ()).await
        }
        pub async fn icrc_1_supported_standards(&self) -> Result<(Vec<StandardRecord>,)> {
            ic_cdk::call(self.0, "icrc1_supported_standards", ()).await
        }
        pub async fn icrc_1_symbol(&self) -> Result<(String,)> {
            ic_cdk::call(self.0, "icrc1_symbol", ()).await
        }
        pub async fn icrc_1_total_supply(&self) -> Result<(candid::Nat,)> {
            ic_cdk::call(self.0, "icrc1_total_supply", ()).await
        }
        pub async fn icrc_1_transfer(&self, arg0: TransferArg) -> Result<(Result_,)> {
            ic_cdk::call(self.0, "icrc1_transfer", (arg0,)).await
        }
        pub async fn icrc_2_allowance(&self, arg0: AllowanceArgs) -> Result<(Allowance,)> {
            ic_cdk::call(self.0, "icrc2_allowance", (arg0,)).await
        }
        pub async fn icrc_2_approve(&self, arg0: ApproveArgs) -> Result<(Result1,)> {
            ic_cdk::call(self.0, "icrc2_approve", (arg0,)).await
        }
        pub async fn icrc_2_transfer_from(&self, arg0: TransferFromArgs) -> Result<(Result2,)> {
            ic_cdk::call(self.0, "icrc2_transfer_from", (arg0,)).await
        }
        pub async fn name(&self) -> Result<(Name,)> {
            ic_cdk::call(self.0, "name", ()).await
        }
        pub async fn query_blocks(&self, arg0: GetBlocksArgs) -> Result<(QueryBlocksResponse,)> {
            ic_cdk::call(self.0, "query_blocks", (arg0,)).await
        }
        pub async fn query_encoded_blocks(
            &self,
            arg0: GetBlocksArgs,
        ) -> Result<(QueryEncodedBlocksResponse,)> {
            ic_cdk::call(self.0, "query_encoded_blocks", (arg0,)).await
        }
        pub async fn send_dfx(&self, arg0: SendArgs) -> Result<(u64,)> {
            ic_cdk::call(self.0, "send_dfx", (arg0,)).await
        }
        pub async fn symbol(&self) -> Result<(Symbol,)> {
            ic_cdk::call(self.0, "symbol", ()).await
        }
        pub async fn transfer(&self, arg0: TransferArgs) -> Result<(Result5,)> {
            ic_cdk::call(self.0, "transfer", (arg0,)).await
        }
        pub async fn transfer_fee(&self, arg0: TransferFeeArg) -> Result<(TransferFee,)> {
            ic_cdk::call(self.0, "transfer_fee", (arg0,)).await
        }
    }
}
