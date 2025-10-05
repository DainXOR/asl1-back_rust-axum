use chrono::{DateTime};
use crate::dto::transaction_dto::TransactionDTO;
use crate::entity::transaction;

impl From<TransactionDTO> for transaction::Model {
    fn from(dto: TransactionDTO) -> Self {
        transaction::Model {
            id: dto.id,
            sender_account_number: dto.sender_account_number,
            receiver_account_number: dto.receiver_account_number,
            amount: dto.amount,
            timestamp: DateTime::parse_from_str(&dto.timestamp, "%Y-%m-%d %H:%M:%S")
                .unwrap_or_else(|_| DateTime::from_timestamp(0, 0).expect("Imposible").into()).into(),
        }
    }
}

impl From<transaction::Model> for TransactionDTO {
    fn from(model: transaction::Model) -> Self {
        TransactionDTO {
            id: model.id,
            sender_account_number: model.sender_account_number,
            receiver_account_number: model.receiver_account_number,
            amount: model.amount,
            timestamp: model.timestamp.to_string(),
        }
    }
}