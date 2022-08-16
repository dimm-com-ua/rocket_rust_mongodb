use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub account: String,
    pub acc_type: AccountType,
    pub crt_balance: Balance,
}

impl Account {
    pub fn new(id: String, account: String, account_type: AccountType) -> Account {
        Self {
            id: None,
            account: account,
            acc_type: account_type,
            crt_balance: Balance::new(0.0_f32)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AccountType {
    Active,
    Passive,
    ActivePassive
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub balance: f32,
    pub balance_qty: f32,
}

impl Balance {
    pub fn new(balance: f32) -> Balance {
        Self{
            balance,
            balance_qty: 0.0_f32
        }
    }
}