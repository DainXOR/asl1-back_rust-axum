use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionRequestDTO {
    pub sender_account_number: String,
    pub receiver_account_number: String,
    pub amount: f64,
}
