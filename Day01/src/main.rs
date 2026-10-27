fn main() {
    let make = "Toyota";
    let model = String::from("Camry");
    let year: u16 = 2025;
    let daily_rate_cents: i64 = 6500;
    let available = true;

    println!("{year} {make} {model}");
    println!("Rate: {daily_rate_cents} cents");
    println!("Available: {available}");
}
