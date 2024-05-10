pub trait Wallet<T> {
    type Balance;
    type TransferFromAccountRes;
    type TransferToAccountRes;

    fn waller_balance() -> Self::Balance;
    fn transfer_from_account() -> Self::TransferToAccountRes;
    fn transfer_to_account() -> Self::TransferToAccountRes;
}
