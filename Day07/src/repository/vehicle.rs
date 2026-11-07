use crate::{
    models::vehicle::{Vehicle, VehicleStatus},
    repository::repo_tools::*,
};
use sqlx::{FromRow, SqlitePool};
use tracing::instrument;

#[derive(FromRow)]
struct VehicleRow {
    id: i64,
    make: String,
    model: String,
    year: i64,
    daily_rate_cents: i64,
    status: String,
}

impl TryFrom<VehicleRow> for Vehicle {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(row: VehicleRow) -> RepoResult<Self> {
        let status = VehicleStatus::parse(&row.status)
            .ok_or_else(|| invalid_row("unknown vehicle status"))?;
        let id = u32::try_from(row.id)?;
        let year = u16::try_from(row.year)?;
        Ok(Vehicle::reconstitute(
            id,
            row.make,
            row.model,
            year,
            row.daily_rate_cents,
            status,
        ))
    }
}

pub struct VehicleRepository {
    pool: SqlitePool,
}

impl VehicleRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    #[instrument(skip_all, name="vehicle.count", fields(operation="vehicle_count", db.system="sqlite"))]
    pub async fn count(&self) -> RepoResult<usize> {
        let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM vehicles")
            .fetch_one(&self.pool)
            .await?;
        Ok(usize::try_from(count)?)
    }

    #[instrument(skip_all, name="vehicle.list", fields(operation="vehicle_list", db.system="sqlite"))]
    pub async fn list(&self) -> RepoResult<Vec<Vehicle>> {
        let rows: Vec<VehicleRow> = sqlx::query_as(
            "SELECT id, make, model, year, daily_rate_cents, status \
            FROM vehicles ORDER BY id;",
        )
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(Vehicle::try_from).collect()
    }

    #[instrument(skip_all, name="vehicle.find", fields(operation="vehicle_find", db.system="sqlite"))]
    pub async fn find(&self, id: u32) -> RepoResult<Option<Vehicle>> {
        let row: Option<VehicleRow> = sqlx::query_as(
            "SELECT id, make, model, year, daily_rate_cents, status \
             FROM vehicles WHERE id = ?",
        )
        .bind(i64::from(id))
        .fetch_optional(&self.pool)
        .await?;
        row.map(Vehicle::try_from).transpose()
    }

    #[instrument(skip_all, name="vehicle.create", fields(operation="vehicle_create", db.system="sqlite"))]
    pub async fn create(
        &self,
        make: String,
        model: String,
        year: u16,
        rate: i64,
    ) -> RepoResult<Vehicle> {
        let result = sqlx::query(
            "INSERT INTO vehicles (make, model, year, daily_rate_cents, status) \
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&make)
        .bind(&model)
        .bind(i64::from(year))
        .bind(rate)
        .bind(VehicleStatus::Ready.as_str())
        .execute(&self.pool)
        .await?;
        let id = u32::try_from(result.last_insert_rowid())?;
        self.find(id)
            .await?
            .ok_or_else(|| invalid_row("inserted vehicle not found").into())
    }

    #[instrument(skip_all, name="vehicle.change_rate", fields(operation="vehicle_change_rate", db.system="sqlite"))]
    pub async fn change_rate(&self, id: u32, rate: i64) -> RepoResult<Option<Vehicle>> {
        let result = sqlx::query("UPDATE vehicles SET daily_rate_cents = ? WHERE id = ?")
            .bind(rate)
            .bind(i64::from(id))
            .execute(&self.pool)
            .await?;
        if result.rows_affected() == 0 {
            return Ok(None);
        }
        self.find(id).await
    }

    #[instrument(skip_all, name="vehicle.set_status", fields(operation="vehicle_set_status", db.system="sqlite"))]
    pub async fn set_status(&self, id: u32, status: VehicleStatus) -> RepoResult<Option<Vehicle>> {
        let result = sqlx::query("UPDATE vehicles SET status = ? WHERE id = ?")
            .bind(status.as_str())
            .bind(i64::from(id))
            .execute(&self.pool)
            .await?;
        if result.rows_affected() == 0 {
            return Ok(None);
        }
        self.find(id).await
    }

    #[instrument(skip_all, name="vehicle.delete", fields(operation="vehicle_delete", db.system="sqlite"))]
    pub async fn delete(&self, id: u32) -> RepoResult<bool> {
        let result = sqlx::query("DELETE FROM vehicles WHERE id = ?")
            .bind(i64::from(id))
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
