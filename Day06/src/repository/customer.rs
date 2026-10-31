use crate::{models::customer::Customer, repository::repo_tools::*};
use sqlx::{FromRow, SqlitePool};

#[derive(FromRow)]
struct CustomerRow {
    id: i64,
    name: String,
    email: String,
}

fn customer_from_row(row: CustomerRow) -> RepoResult<Customer> {
    let id = u32::try_from(row.id)?;
    Ok(Customer::reconstitute(id, row.name, row.email))
}

pub struct CustomerRepository {
    pool: SqlitePool,
}

impl CustomerRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> RepoResult<Vec<Customer>> {
        let rows: Vec<CustomerRow> =
            sqlx::query_as("SELECT id, name, email FROM customers ORDER BY id;")
                .fetch_all(&self.pool)
                .await?;

        rows.into_iter().map(customer_from_row).collect()
    }

    pub async fn find(&self, id: u32) -> RepoResult<Option<Customer>> {
        let row: Option<CustomerRow> =
            sqlx::query_as("SELECT id, name, email FROM customers where id = ?;")
                .bind(i64::from(id))
                .fetch_optional(&self.pool)
                .await?;

        row.map(customer_from_row).transpose()
    }

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

    pub async fn email_exists(&self, email: &str) -> RepoResult<bool> {
        let found: Option<(i64,)> = sqlx::query_as("SELECT id FROM customers WHERE email = ?;")
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;
        Ok(found.is_some())
    }
}
