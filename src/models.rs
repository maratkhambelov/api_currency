use super::schema::exchanges_rates;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = exchanges_rates)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Rate {
    pub id: Option<i32>,
    pub base_currency: String,
    pub target_currency: String,
    pub rate: f32,
    pub timestamp: Option<String>,
}


//TODO: delete timestamp
#[derive(Insertable, Debug)]
#[diesel(table_name = exchanges_rates)]
pub struct NewRate<'a> {
    pub base_currency: &'a str,
    pub target_currency: &'a str,
    pub rate: f32,
    pub timestamp: &'a str,
}