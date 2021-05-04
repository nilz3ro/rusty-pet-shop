#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("missing env var: DATABASE_URL");
    println!("db url? {}", db_url);

    MysqlConnection::establish(&db_url).expect(&format!("could not connect to db at : {}", db_url))
}
