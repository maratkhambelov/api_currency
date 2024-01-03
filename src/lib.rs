pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use crate::models::{NewRate, Rate};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

//TODO: delete
pub fn create_rate(conn: &mut SqliteConnection, base_currency: &str, target_currency: &str, last_update: &str) -> Rate {
    use crate::schema::exchanges_rates;

    let new_rate = NewRate { base_currency, target_currency, rate: 1f32, last_update };

    diesel::insert_into(exchanges_rates::table)
        .values(&new_rate)
        .returning(Rate::as_returning())
        .get_result(conn)
        .expect("Error saving new rate")
}