use self::models::*;
use diesel::prelude::*;
use reqwest::{ Error };
use tokio::runtime::Runtime;
use sqlite_test_v2::*;

async fn get_last_update_rates() -> Result<(), Error> {
    use self::schema::exchanges_rates::dsl::*;

    let connection = &mut establish_connection();
    let result = exchanges_rates
        .filter(timestamp.is_not_null())
        .limit(1)
        .select(Rate::as_select())
        .load(connection)
        .expect("Error loading currencies");


    println!("result {:?}", result[0]);

    Ok(())

}
fn main() {

    let mut rt = Runtime::new().unwrap();
    match rt.block_on(get_last_update_rates()) {
        Ok(_) => println!("Currency rates fetched successfully!"),
        Err(e) => eprintln!("Error: {}", e),
    }

}