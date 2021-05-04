#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::error::Error;

pub mod models;
pub mod schema;

use self::models::NewPet;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("missing env var: DATABASE_URL");
    println!("db url? {}", db_url);

    MysqlConnection::establish(&db_url).expect(&format!("could not connect to db at : {}", db_url))
}

pub fn create_pet<'a>(
    conn: &MysqlConnection,
    name: &'a str,
    owner: &'a str,
    species: &'a str,
) -> Result<(), Box<dyn Error>> {
    use schema::pets;

    let new_pet = NewPet {
        name,
        owner,
        species,
    };

    diesel::insert_into(pets::table)
        .values(&new_pet)
        .execute(conn)?;

    Ok(())
}
