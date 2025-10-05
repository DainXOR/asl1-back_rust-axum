use crate::dto::customer_dto::CustomerDTO;
use crate::entity::customer;
use crate::repository::customer_repository::CustomerRepository;
use sea_orm::DbErr;

pub struct CustomerService {
    customerRepo: CustomerRepository,
}

impl CustomerService {
    pub fn new(customerRepo: CustomerRepository) -> Self {
        Self { customerRepo }
    }

    pub async fn create(&self, customer: CustomerDTO) -> Result<CustomerDTO, DbErr> {
        let mut entity: customer::Model = customer.into();
        entity.id = 0;
        entity.account_number = String::new();
        entity.balance = 0.0;

        let created = self.customerRepo.create(entity).await?;

        Ok(created.into())
    }

    pub async fn find_by_id(&self, id: u32) -> Option<CustomerDTO> {
        let customer = self.customerRepo.find_by_id(id).await?;
        Some(customer.into())
    }

    pub async fn find_by_account_number(&self, account_number: &str) -> Option<CustomerDTO> {
        let customer = self.customerRepo.find_by_account_number(account_number).await?;
        Some(customer.into())
    }
}