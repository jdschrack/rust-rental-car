use tokio::time::{Duration, sleep};

async fn greet() {
    println!("Before waiting");
    sleep(Duration::from_millis(100)).await;
    println!("After waiting");
}

#[tokio::main]
async fn main() {
    //let future = greet();
    //println!("Future created");
    //future.await;

    let started = std::time::Instant::now();
    greet().await;
    greet().await;
    println!("Sequential: {:?}", started.elapsed());

    let started = std::time::Instant::now();
    tokio::join!(greet(), greet());
    println!("Concurrent: {:?}", started.elapsed());
}
