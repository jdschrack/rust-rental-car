use rust_rent::api::{AppState, router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = router(AppState::default());

    let listener_address: String = "0.0.0.0:3000".into();

    let listener = tokio::net::TcpListener::bind(listener_address.clone()).await?;

    println!("RestRent: http://{}", listener_address);

    axum::serve(listener, app).await?;

    Ok(())
}
