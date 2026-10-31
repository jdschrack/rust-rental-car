use crate::{
    api::{errors, router::AppState},
    dto::customer_dto::*,
    models::customer::Customer,
    repository::customer::*,
};

use axum::{Json, extract::State, http::StatusCode};

pub(crate) async fn list_customers(
    State(state): State<AppState>,
) -> Result<Json<Vec<CustomerResponse>>, StatusCode> {
    let repository = CustomerRepository::new(state.pool.clone());
    let customers = repository.list().await.map_err(errors::internal_error)?;
    Ok(Json(customers.iter().map(CustomerResponse::from).collect()))
}

pub(crate) async fn create_customer(
    State(state): State<AppState>,
    Json(input): Json<CreateCustomer>,
) -> Result<(StatusCode, Json<CustomerResponse>), StatusCode> {
    Customer::new(0, input.name.clone(), input.email.clone())
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let repository = CustomerRepository::new(state.pool.clone());
    // Database uniqueness protects against concurrent duplicate requests.
    let email_for_conflict_check = input.email.clone();
    match repository.create(input.name, input.email).await {
        Ok(customer) => Ok((StatusCode::CREATED, Json(CustomerResponse::from(&customer)))),
        Err(error) => {
            if let Ok(true) = repository.email_exists(&email_for_conflict_check).await {
                return Err(StatusCode::CONFLICT);
            }
            eprintln!("Customer insertion failed: {error}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
