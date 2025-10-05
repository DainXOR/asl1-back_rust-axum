use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait};
use crate::entity::customer;

pub struct CustomerRepository {
    db: DatabaseConnection
}

impl CustomerRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, customer: customer::Model) -> Result<customer::Model, DbErr> {
        let active_model: customer::ActiveModel = customer.into();
        let res = active_model.insert(&self.db).await?;
        Ok(res)
    }

    pub async fn find_by_account_number(&self, account_number: &str) -> Option<customer::Model> {
        let customer = customer::Entity::find()
            .filter(customer::Column::AccountNumber.eq(account_number))
            .one(&self.db)
            .await
            .ok()
            .flatten();

        customer
    }
}