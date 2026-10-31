use rust_rent::{
    database,
    repository::{customer::CustomerRepository, vehicle::VehicleRepository},
};
use tempfile::tempdir;

#[tokio::test]
async fn vehicles_and_customers_survive_new_pool_connections() {
    let directory = tempdir().expect("temp directory");
    let path = directory.path().join("rent-test.db");
    let url = format!("sqlite://{}", path.display());

    let pool = database::connect(&url)
        .await
        .expect("database and migrations");
    let vehicles = VehicleRepository::new(pool.clone());
    let customers = CustomerRepository::new(pool.clone());

    let created_vehicle = vehicles
        .create("Toyota".into(), "Camry".into(), 2025, 6500)
        .await
        .expect("vehicle insert");
    let created_customer = customers
        .create("Avery".into(), "avery@example.com".into())
        .await
        .expect("customer insert");

    assert_eq!(created_vehicle.make(), "Toyota");
    assert_eq!(created_customer.email(), "avery@example.com");
    pool.close().await;

    let reopened = database::connect(&url).await.expect("reopen database");
    let saved_vehicles = VehicleRepository::new(reopened.clone())
        .list()
        .await
        .expect("vehicle list");
    let saved_customers = CustomerRepository::new(reopened)
        .list()
        .await
        .expect("customer list");

    assert_eq!(saved_vehicles.len(), 1);
    assert_eq!(saved_customers.len(), 1);
}
