use crate::dto::customer_dto::CustomerDTO;
use crate::entity::customer;

impl From<CustomerDTO> for customer::Model {
    fn from(dto: CustomerDTO) -> Self {
        customer::Model {
            id: dto.id,
            account_number: dto.account_number,
            first_name: dto.first_name,
            last_name: dto.last_name,
            balance: dto.balance,
        }
    }
}

impl From<customer::Model> for CustomerDTO {
    fn from(model: customer::Model) -> Self {
        CustomerDTO {
            id: model.id,
            account_number: model.account_number,
            first_name: model.first_name,
            last_name: model.last_name,
            balance: model.balance,
        }
    }
}