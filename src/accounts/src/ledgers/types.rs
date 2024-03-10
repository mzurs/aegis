use candid::Principal;

pub enum ICRCLedgerType {
    ICP,
    CKBTC,
    CKETH,
}

#[derive(Debug, Copy, Clone)]
pub struct Ledger(pub Principal);

#[derive(Debug, Copy, Clone)]
pub struct CKBTCMinter(pub Principal);

