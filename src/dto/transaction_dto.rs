use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionDTO {
    pub id: u32,
    pub sender_account_number: String,
    pub receiver_account_number: String,
    pub amount: f64,
    pub timestamp: String,
}
