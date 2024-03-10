use candid::Principal;

use crate::{
    guard::_if_account_exist,
    memory::USER_ACCOUNTS,
    types::states::Account,
    utils::{_generate_random_number, increment_user_count},
};

impl Account {
    pub fn get_account() -> Option<Self> {
        USER_ACCOUNTS.with(|accounts| accounts.borrow().get(&ic_cdk::caller()))
    }

    pub async fn create_account() -> Result<bool, String> {
        let user_id: u64 = match _generate_random_number().await {
            Ok(id) => id,
            Err(err) => return Err(err),
        };

        let account_args: Account = Account {
            user_id,
            principal: ic_cdk::caller(),
            user_name: Option::None,
        };

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

    pub fn update_account_user_name(user_name: String) -> Result<(), String> {
        let principal: &Principal = &ic_cdk::caller();

        if _if_account_exist(*principal) {
            let _account: Account = Self::get_account().unwrap();

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
}
