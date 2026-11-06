use crate::{models::customer::Customer, repository::repo_tools::*};
use sqlx::{FromRow, SqlitePool};
use tracing::{Instrument, info_span, instrument};
#[derive(FromRow)]
struct CustomerRow {
    id: i64,
    name: String,
    email: String,
}

impl TryFrom<CustomerRow> for Customer {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(row: CustomerRow) -> RepoResult<Self> {
        let id = u32::try_from(row.id)?;
        Ok(Customer::reconstitute(id, row.name, row.email))
    }
}

pub struct CustomerRepository {
    pool: SqlitePool,
}

impl CustomerRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    #[instrument(skip_all, name="customer.list", fields(operation="customer_get", db.system="sqlite", customer.count = tracing::field::Empty))]
    pub async fn list(&self) -> RepoResult<Vec<Customer>> {
        let rows: Vec<CustomerRow> =
            sqlx::query_as("SELECT id, name, email FROM customers ORDER BY id;")
                .fetch_all(&self.pool)
                .await?;

        tracing::Span::current().record("customer.count", rows.len());

        rows.into_iter().map(Customer::try_from).collect()
    }

    #[instrument(skip_all, name="customer.find", fields(operation="customer_find", db.system="sqlite"))]
    pub async fn find(&self, id: u32) -> RepoResult<Option<Customer>> {
        let row: Option<CustomerRow> =
            sqlx::query_as("SELECT id, name, email FROM customers where id = ?;")
                .bind(i64::from(id))
                .fetch_optional(&self.pool)
                .await?;

        row.map(Customer::try_from).transpose()
    }

    #[instrument(skip_all, name="customer.create", fields(operation="customer_create", db.system="sqlite"))]
    pub async fn create(&self, name: String, email: String) -> RepoResult<Customer> {
        let result = sqlx::query("INSERT INTO customers (name, email) VALUES (?, ?);")
            .bind(&name)
            .bind(&email)
            .execute(&self.pool)
            .await?;
        let id = u32::try_from(result.last_insert_rowid())?;
        self.find(id)
            .await?
            .ok_or_else(|| invalid_row("inserted customer not found").into())
    }

    #[instrument(skip_all, name="customer.email_exists", fields(operation="customer_email_exists", db.system="sqlite"))]
    pub async fn email_exists(&self, email: &str) -> RepoResult<bool> {
        let found: Option<(i64,)> = sqlx::query_as("SELECT id FROM customers WHERE email = ?;")
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;
        Ok(found.is_some())
    }
}
