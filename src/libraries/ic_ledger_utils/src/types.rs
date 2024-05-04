pub mod icrc_types {
    use candid::{CandidType, Nat};
    use icrc_ledger_types::{icrc1::transfer::TransferError, icrc2::transfer_from::TransferFromError};
    use serde::Deserialize;

    pub type Fee = Nat;
    pub type ErrorMessage = String;

    pub enum IcrcFee {
        Fee(Nat),
        ErrorMessage(String),
    }

    #[derive(CandidType, Deserialize)]
    pub enum TransferResult {
        Ok(candid::Nat),
        Err(TransferError),
    }

    #[derive(CandidType, Deserialize)]
    pub enum IcrcTransferResult {
        TransferSuccess(Nat),
        TransferErrorMessage(TransferError),
        TransferErrorString(ErrorMessage),
    }

    #[derive(CandidType, Deserialize)]
    pub enum TransferFromResult {
        Ok(candid::Nat),
        Err(TransferFromError),
    }

    #[derive(CandidType, Deserialize)]
    pub enum IcrcTransferFromResult {
        TransferFromSuccess(Nat),
        TransferFromErrorMessage(TransferFromError),
        TransferFromErrorString(ErrorMessage),
    }
}
