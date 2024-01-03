use diesel::RunQueryDsl;
use self::models::*;
use sqlite_test_v2::*;
use reqwest::{ Client, Error};
use serde_json::Value;
use tokio::runtime::Runtime;
use chrono::{Local};


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


        let data = response["data"].as_object().unwrap();

        for (currency_code, rate_value) in data.iter() {
            let parsed_rate = rate_value.as_object().unwrap();
            let current_date = Local::now();
            let formatted_date = current_date.format("%Y-%m-%d %H:%M:%S").to_string();

            println!("formatted_date {:?}", formatted_date);


            let exchange_rate = NewRate {
                base_currency: currency,
                target_currency: &*currency_code.to_string(),
                rate: parsed_rate["value"].as_f64().unwrap() as f32,
                last_update: &*formatted_date,
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
