use diesel::RunQueryDsl;
use self::models::*;
use sqlite_test_v2::*;
use reqwest::{ Client, Error};
use serde_json::Value;
use tokio::runtime::Runtime;


async fn fetch_rates(currency: &str) -> Result<(), Error> {
    use self::schema::exchanges_rates::dsl::*;


    let url = dotenvy::var("API_URL").unwrap();
    let api_key = dotenvy::var("API_KEY").unwrap();

    let client = Client::new();


    let response = client
        .get(url.to_owned() + "/latest")
        .query(&[("apikey", api_key), ("base_currency",  currency.to_string())])
        .send()
        .await?;


    if response.status().is_success() {

        let body = response.text().await?;


        let response: Value = serde_json::from_str(&body).unwrap();


        let parsed_timestamp = "1704204843"; //TODO: delete

        let data = response["data"].as_object().unwrap();

        for (currency_code, rate_value) in data.iter() {
            let parsed_rate = rate_value.as_object().unwrap();

            let exchange_rate = NewRate {
                base_currency: currency,
                target_currency: &*currency_code.to_string(),
                rate: parsed_rate["value"].as_f64().unwrap() as f32,
                timestamp: &*parsed_timestamp.to_string(),
            };
            println!("exchange_rate {:?}", exchange_rate);

            diesel::insert_into(exchanges_rates)
                .values(&exchange_rate)
                .execute(&mut establish_connection())
                .expect("Failed to insert data into exchange_rates table");
        }

    } else {
        println!("Request failed with status code: {}", response.status());
    }



    Ok(())

}


fn main() {


    let mut rt = Runtime::new().unwrap();
    match rt.block_on(fetch_rates("RUB")) {
        Ok(_) => println!("Currency rates fetched successfully!"),
        Err(e) => eprintln!("Error: {}", e),
    }
    match rt.block_on(fetch_rates("GBP")) {
        Ok(_) => println!("Currency rates fetched successfully!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

// use sqlite_test_v2::*;
// use std::io::{stdin, Read};
//
// fn main() {
//     let connection = &mut establish_connection();
//
//     println!("What would you like your base_currency to be?");
//     let mut base_currency = String::new();
//     stdin().read_line(&mut base_currency).unwrap();
//     let base_currency = &base_currency[..(base_currency.len() - 1)]; // Drop the newline character
//     println!("\nOk! Let's write {base_currency} (Press {EOF} when finished)\n");
//
//     let mut target_currency = String::new();
//     stdin().read_to_string(&mut target_currency).unwrap();
//
//     let mut timestamp = String::new();
//     stdin().read_to_string(&mut timestamp).unwrap();
//
//
//     let rate = create_rate(connection, base_currency, &target_currency, &timestamp);
//     println!("\nSaved draft {base_currency} with id {}", rate.base_currency);
// }
//
// #[cfg(windows)]
// const EOF: &str = "CTRL+Z";

//use self::models::*;
// use diesel::prelude::*;
// use sqlite_test_v2::*;
//
// fn main() {
//     use self::schema::exchanges_rates::dsl::*;
//
//     let connection = &mut establish_connection();
//     let results = exchanges_rates
//         // .filter(id.eq(1))
//         .limit(5)
//         .select(Rate::as_select())
//         .load(connection)
//         .expect("Error loading posts");
//
//     //.filter(published.eq(true))
//     println!("Displaying {:?}", results[4]);
//     // for rate in results {
//     //     // println!("{}", rate.base_currency);
//     //     // println!("----------\n");
//     //     // println!("{}", rate.target_currency);
//     // }
// }