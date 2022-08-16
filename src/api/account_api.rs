use mongodb::results::InsertOneResult;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::models::account_model::{Account, AccountType, Balance};
use crate::MongoRepo;

#[post("/account", data="<new_account>")]
pub fn create_account(
    db: &State<MongoRepo>,
    new_account: Json<Account>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Account {
        id: None,
        account: new_account.account.to_owned(),
        acc_type: AccountType::Active,
        crt_balance: Balance::new(0.0_f32)
    };
    let account_detail = db.accounts().create_account(data);
    match account_detail {
        Ok(account) => Ok(Json(account)),
        Err(_) => Err(Status::InternalServerError),
    }
}