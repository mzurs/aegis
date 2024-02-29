use candid::Principal;

use crate::{
    guard::_if_account_exist, memory::USER_ACCOUNTS, types::states::Account,
    utils::increment_user_count,
};

pub fn _get_account() -> Option<Account> {
    USER_ACCOUNTS.with(|accounts| accounts.borrow().get(&ic_cdk::caller()))
}

pub async fn _create_account() -> Result<bool, String> {
    Account::new().await
}

pub fn _add_account(account_args: Account) -> Result<bool, String> {
    USER_ACCOUNTS.with(|accounts| {
        let res: Option<Account> = accounts.borrow_mut().insert(ic_cdk::caller(), account_args);

        match res {
            Some(_res) => {
                increment_user_count();
                Ok(true)
            }
            None => Err(String::from("Unable to Insert in Stable Memory")),
        }
    })
}

pub fn _update_account_user_name(user_name: String) -> Result<(), String> {
    let principal: &Principal = &ic_cdk::caller();

    if _if_account_exist(*principal) {
        let _account: Account = _get_account().unwrap();

        USER_ACCOUNTS.with(|accounts| {
            accounts.borrow_mut().insert(
                *principal,
                Account {
                    user_name: Option::Some(user_name),
                    .._account
                },
            )
        });
    } else {
        return Result::Err(String::from("Account Not Exists"));
    }

    Ok(())
}
