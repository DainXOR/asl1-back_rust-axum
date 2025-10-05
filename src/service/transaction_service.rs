use sea_orm::DbErr;
use crate::dto::transaction_dto::TransactionDTO;
use crate::dto::transaction_request_dto::TransactionRequestDTO;
use crate::repository::customer_repository::CustomerRepository;
use crate::repository::transaction_repository::TransactionRepository;

pub struct TransactionService {
    transaction_repo: TransactionRepository,
    customer_repo: CustomerRepository,
}

impl TransactionService {
    pub fn new(transaction_repo: TransactionRepository, customer_repo: CustomerRepository) -> Self {
        Self { transaction_repo: transaction_repo, customer_repo: customer_repo }
    }

    pub async fn create(&self, transaction_request: TransactionRequestDTO) -> Result<TransactionDTO, DbErr> {
        let mut sender_customer = self.customer_repo.find_by_account_number(&transaction_request.sender_account_number).await
            .ok_or(DbErr::Custom("From account not found".to_string()))?;

        let mut receiver_customer = self.customer_repo.find_by_account_number(&transaction_request.receiver_account_number).await
            .ok_or(DbErr::Custom("To account not found".to_string()))?;

        if sender_customer.balance < transaction_request.amount {
            return Err(DbErr::Custom("Insufficient balance".to_string()));
        }

        sender_customer.balance -= transaction_request.amount;
        receiver_customer.balance += transaction_request.amount;

        self.customer_repo.create(sender_customer).await?;
        self.customer_repo.create(receiver_customer).await?;

        let transaction = crate::entity::transaction::Model {
            id: 0,
            sender_account_number: transaction_request.sender_account_number,
            receiver_account_number: transaction_request.receiver_account_number,
            amount: transaction_request.amount,
            timestamp: chrono::Utc::now(),
        };

        let created = self.transaction_repo.create(transaction).await?;

        Ok(created.into())
    }
}

