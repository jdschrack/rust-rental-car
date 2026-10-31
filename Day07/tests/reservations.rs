use chrono::NaiveDate;
use rust_rent::{
    database,
    models::{reservation::ReservationStatus, vehicle::VehicleStatus},
    repository::{
        customer::CustomerRepository,
        reservation::{RentalError, ReservationRepository},
        vehicle::VehicleRepository,
    },
};
use tempfile::tempdir;

fn date(day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 11, day).expect("valid date")
}

#[tokio::test]
async fn create_overlap_adjacent_cancel_and_persist() {
    let directory = tempdir().unwrap();
    let url = format!("sqlite://{}", directory.path().join("test.db").display());
    let pool = database::connect(&url).await.unwrap();
    let vehicle = VehicleRepository::new(pool.clone())
        .create("Toyota".into(), "Camry".into(), 2025, 6500)
        .await
        .unwrap();
    let customer = CustomerRepository::new(pool.clone())
        .create("Avery".into(), "avery@example.com".into())
        .await
        .unwrap();
    let service = ReservationRepository::new(pool.clone());

    let first = service
        .create(vehicle.id(), customer.id(), date(1), date(4))
        .await
        .unwrap();
    assert_eq!(first.total_cents(), 19500);

    let overlap = service
        .create(vehicle.id(), customer.id(), date(2), date(5))
        .await;
    assert!(matches!(overlap, Err(RentalError::OverlappingReservation)));

    let adjacent = service
        .create(vehicle.id(), customer.id(), date(4), date(7))
        .await
        .unwrap();
    assert_ne!(first.id(), adjacent.id());

    let cancelled = service.cancel(first.id()).await.unwrap();
    assert_eq!(cancelled.status(), ReservationStatus::Cancelled);
    let replacement = service
        .create(vehicle.id(), customer.id(), date(1), date(4))
        .await
        .unwrap();
    assert_ne!(replacement.id(), first.id());

    pool.close().await;
    let reopened = database::connect(&url).await.unwrap();
    let persisted = ReservationRepository::new(reopened)
        .find(replacement.id())
        .await
        .unwrap()
        .expect("persisted booking");
    assert_eq!(persisted.total_cents(), 19500);
}

#[tokio::test]
async fn simultaneous_conflicting_requests_cannot_both_commit() {
    let directory = tempdir().unwrap();
    let url = format!(
        "sqlite://{}",
        directory.path().join("concurrent.db").display()
    );
    let pool = database::connect(&url).await.unwrap();
    let vehicle = VehicleRepository::new(pool.clone())
        .create("Honda".into(), "Civic".into(), 2025, 5000)
        .await
        .unwrap();
    let customer = CustomerRepository::new(pool.clone())
        .create("Taylor".into(), "taylor@example.com".into())
        .await
        .unwrap();

    let a = ReservationRepository::new(pool.clone());
    let b = ReservationRepository::new(pool.clone());
    let (left, right) = tokio::join!(
        a.create(vehicle.id(), customer.id(), date(1), date(4)),
        b.create(vehicle.id(), customer.id(), date(1), date(4))
    );
    let results = [left, right];
    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
    assert_eq!(
        results
            .iter()
            .filter(|result| matches!(result, Err(RentalError::OverlappingReservation)))
            .count(),
        1,
        "the loser must be a domain overlap, not a database failure"
    );
}

#[tokio::test]
async fn vehicle_in_maintenance_cannot_be_reserved() {
    let directory = tempdir().unwrap();
    let url = format!(
        "sqlite://{}",
        directory.path().join("maintenance.db").display()
    );
    let pool = database::connect(&url).await.unwrap();
    let vehicles = VehicleRepository::new(pool.clone());
    let vehicle = vehicles
        .create("Ford".into(), "Focus".into(), 2025, 4500)
        .await
        .unwrap();
    let customer = CustomerRepository::new(pool.clone())
        .create("Jordan".into(), "jordan@example.com".into())
        .await
        .unwrap();
    let service = ReservationRepository::new(pool.clone());

    let updated = vehicles
        .set_status(vehicle.id(), VehicleStatus::Maintenance)
        .await
        .unwrap()
        .expect("vehicle exists");
    assert!(!updated.is_rentable());

    let blocked = service
        .create(vehicle.id(), customer.id(), date(1), date(4))
        .await;
    assert!(matches!(blocked, Err(RentalError::VehicleUnavailable)));

    vehicles
        .set_status(vehicle.id(), VehicleStatus::Ready)
        .await
        .unwrap()
        .expect("vehicle exists");
    service
        .create(vehicle.id(), customer.id(), date(1), date(4))
        .await
        .unwrap();

    let missing = vehicles
        .set_status(9999, VehicleStatus::Maintenance)
        .await
        .unwrap();
    assert!(missing.is_none());
}
