use rust_rent::{
    api::router::{AppState, router},
    database,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://rust_rent.db".to_owned());
    let pool = database::connect(&database_url).await?;

    let app = router(AppState { pool });

    let listener_address: String = "0.0.0.0:3000".into();

    let listener = tokio::net::TcpListener::bind(listener_address.clone()).await?;

    println!("RestRent: http://{}", listener_address);

    axum::serve(listener, app).await?;

    Ok(())
}
