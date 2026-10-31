use crate::models::customer::Customer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateCustomer {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct CustomerResponse {
    pub id: u32,
    pub name: String,
    pub email: String,
}

impl From<&Customer> for CustomerResponse {
    fn from(customer: &Customer) -> Self {
        Self {
            id: customer.id(),
            name: customer.name().to_owned(),
            email: customer.email().to_owned(),
        }
    }
}
