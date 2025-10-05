use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait};
use crate::entity::transaction;

pub struct TransactionRepository {
    db: DatabaseConnection,
}

impl TransactionRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, transaction: transaction::Model) -> Result<transaction::Model, DbErr> {
        let active_model: transaction::ActiveModel = transaction.into();
        let res = active_model.insert(&self.db).await?;
        Ok(res)
    }

    pub async fn find_by_id(&self, id: u32) -> Option<transaction::Model> {
        transaction::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .ok()
            .flatten()
    }

    // Puedes agregar más métodos según tus necesidades
}

