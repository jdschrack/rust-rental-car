use crate::models::{
    reservation::{Reservation, ReservationError, ReservationStatus},
    vehicle::VehicleStatus,
};
use chrono::NaiveDate;
use sqlx::{FromRow, SqlitePool};
use tracing::{Instrument, info_span, instrument};

#[derive(Debug)]
pub enum RentalError {
    VehicleNotFound,
    CustomerNotFound,
    ReservationNotFound,
    VehicleUnavailable,
    OverlappingReservation,
    InvalidDateRange,
    InvalidRate,
    PriceOverflow,
    Database(sqlx::Error),
    InvalidStoredData,
}

impl From<sqlx::Error> for RentalError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

impl From<ReservationError> for RentalError {
    fn from(error: ReservationError) -> Self {
        match error {
            ReservationError::InvalidDateRange => Self::InvalidDateRange,
            ReservationError::InvalidRate => Self::InvalidRate,
            ReservationError::PriceOverflow => Self::PriceOverflow,
        }
    }
}

#[derive(FromRow)]
struct ReservationRow {
    id: i64,
    vehicle_id: i64,
    customer_id: i64,
    start_date: String,
    end_date: String,
    total_cents: i64,
    status: String,
}

impl TryFrom<ReservationRow> for Reservation {
    type Error = RentalError;

    fn try_from(row: ReservationRow) -> Result<Self, Self::Error> {
        let status = ReservationStatus::parse(&row.status).ok_or(RentalError::InvalidStoredData)?;
        let id = u32::try_from(row.id).map_err(|_| RentalError::InvalidStoredData)?;
        let vehicle_id =
            u32::try_from(row.vehicle_id).map_err(|_| RentalError::InvalidStoredData)?;
        let customer_id =
            u32::try_from(row.customer_id).map_err(|_| RentalError::InvalidStoredData)?;
        let start = NaiveDate::parse_from_str(&row.start_date, "%Y-%m-%d")
            .map_err(|_| RentalError::InvalidStoredData)?;
        let end = NaiveDate::parse_from_str(&row.end_date, "%Y-%m-%d")
            .map_err(|_| RentalError::InvalidStoredData)?;
        Ok(Reservation::reconstitute(
            id,
            vehicle_id,
            customer_id,
            start,
            end,
            row.total_cents,
            status,
        ))
    }
}

pub struct ReservationRepository {
    pool: SqlitePool,
}

impl ReservationRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    #[instrument(skip_all, name="reservations.find", fields(operation="reservations_find", db.system="sqlite"))]
    pub async fn find(&self, id: u32) -> Result<Option<Reservation>, RentalError> {
        let row: Option<ReservationRow> = sqlx::query_as(
            "SELECT id, vehicle_id, customer_id, start_date, 
            end_date, total_cents, status 
            FROM reservations 
            WHERE id  = ?;",
        )
        .bind(i64::from(id))
        .fetch_optional(&self.pool)
        .await?;

        row.map(Reservation::try_from).transpose()
    }

    #[instrument(skip_all, name="reservations.create", fields(operation="reservations_create", db.system="sqlite"))]
    pub async fn create(
        &self,
        vehicle_id: u32,
        customer_id: u32,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Reservation, RentalError> {
        if end_date <= start_date {
            return Err(RentalError::InvalidDateRange);
        }

        let mut tx = self
            .pool
            .begin_with("BEGIN IMMEDIATE")
            .instrument(info_span!("db.acquire_write_lock"))
            .await?;

        let vehicle: Option<(i64, String)> =
            sqlx::query_as("SELECT daily_rate_cents, status FROM vehicles WHERE id = ?")
                .bind(i64::from(vehicle_id))
                .fetch_optional(&mut *tx)
                .await?;
        let (rate, status) = vehicle.ok_or(RentalError::VehicleNotFound)?;
        if VehicleStatus::parse(&status) != Some(VehicleStatus::Ready) {
            return Err(RentalError::VehicleUnavailable);
        }

        let customer: Option<(i64,)> = sqlx::query_as("SELECT id FROM customers WHERE id = ?")
            .bind(i64::from(customer_id))
            .fetch_optional(&mut *tx)
            .await?;
        if customer.is_none() {
            return Err(RentalError::CustomerNotFound);
        }

        let end = end_date.format("%Y-%m-%d").to_string();
        let start = start_date.format("%Y-%m-%d").to_string();
        let overlap: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM reservations \
             WHERE vehicle_id = ? AND status = ? \
             AND start_date < ? AND end_date > ?",
        )
        .bind(i64::from(vehicle_id))
        .bind(ReservationStatus::Confirmed.as_str())
        .bind(&end)
        .bind(&start)
        .fetch_one(&mut *tx)
        .await?;
        if overlap.0 > 0 {
            return Err(RentalError::OverlappingReservation);
        }

        let reservation = Reservation::new(0, vehicle_id, customer_id, start_date, end_date, rate)?;
        let inserted = sqlx::query(
            "INSERT INTO reservations \
             (vehicle_id, customer_id, start_date, end_date, total_cents, status) \
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(i64::from(vehicle_id))
        .bind(i64::from(customer_id))
        .bind(&start)
        .bind(&end)
        .bind(reservation.total_cents())
        .bind(ReservationStatus::Confirmed.as_str())
        .execute(&mut *tx)
        .await?;
        let id = u32::try_from(inserted.last_insert_rowid())
            .map_err(|_| RentalError::InvalidStoredData)?;

        tx.commit().await?;
        Reservation::new(id, vehicle_id, customer_id, start_date, end_date, rate)
            .map_err(RentalError::from)
    }

    #[instrument(skip_all, name="reservations.cancel", fields(operation="reservations_cancel", db.system="sqlite"))]
    pub async fn cancel(&self, id: u32) -> Result<Reservation, RentalError> {
        let result = sqlx::query("UPDATE reservations SET status = ? WHERE id = ?")
            .bind(ReservationStatus::Cancelled.as_str())
            .bind(i64::from(id))
            .execute(&self.pool)
            .await?;
        if result.rows_affected() == 0 {
            return Err(RentalError::ReservationNotFound);
        }
        self.find(id).await?.ok_or(RentalError::ReservationNotFound)
    }
}
