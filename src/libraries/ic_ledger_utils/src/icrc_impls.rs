use std::fmt;

use crate::types::icrc_types::IcrcTransferFromResult;

impl fmt::Display for IcrcTransferFromResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IcrcTransferFromResult::TransferFromSuccess(amount) => {
                write!(f, "Transfer Successful: {}", amount)
            }
            IcrcTransferFromResult::TransferFromErrorMessage(error) => {
                write!(f, "Transfer Error: {:?}", error)
            }
            IcrcTransferFromResult::TransferFromErrorString(error) => {
                write!(f, "Transfer Error: {}", error)
            }
        }
    }
}
