use mongodb::bson::extjson::de::Error;
use mongodb::results::InsertOneResult;
use mongodb::sync::{Collection, Database};
use crate::models::account_model::Account;
use crate::MongoRepo;

pub struct AccountRepo {
    col: Collection<Account>
}

impl AccountRepo {
    pub fn create(col: Collection<Account>) -> Self {
        AccountRepo {
            col: col
        }
    }

    pub fn create_account(&self, new_account: Account) -> Result<InsertOneResult, Error> {
        let new_account = Account {
            id: None,
            account: new_account.account,
            acc_type: new_account.acc_type,
            crt_balance: new_account.crt_balance
        };
        let account = self
            .col
            .insert_one(new_account, None)
            .ok()
            .expect("Error creating account");
        Ok(account)
    }
}