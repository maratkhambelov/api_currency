use self::models::*;
use diesel::prelude::*;
use reqwest::{ Error };
use tokio::runtime::Runtime;
use sqlite_test_v2::*;

async fn convert_currency(from: &str, to: &str, amount: f32) -> Result<(), Error> {
    use self::schema::exchanges_rates::dsl::*;

    let connection = &mut establish_connection();
    let pair = exchanges_rates
        .filter(base_currency.eq(from))
        .filter(target_currency.eq(to))
        .select(Rate::as_select())
        .load(connection)
        .expect("Error loading currencies");

    println!("Displaying {:?}", pair[0]);

    let result: f32 = pair[0].rate * amount;
    println!("result {}", result);

    Ok(())

}
fn main() {

    let mut rt = Runtime::new().unwrap();
    match rt.block_on(convert_currency("RUB", "USD", 2000 as f32)) {
        Ok(_) => println!("Currency rates fetched successfully!"),
        Err(e) => eprintln!("Error: {}", e),
    }

}