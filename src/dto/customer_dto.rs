use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerDTO {
    pub id: u32,
    pub account_number: String,
    pub first_name: String,
    pub last_name: String,
    pub balance: f64,
}