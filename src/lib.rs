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

pub mod data {
    use crate::models::{NewPet, Pet};
    use diesel::prelude::*;
    use diesel::MysqlConnection;
    use std::error::Error;

    pub fn get_pets(conn: &MysqlConnection) -> Result<Vec<Pet>, diesel::result::Error> {
        use super::schema::pets::dsl::*;

        pets.load::<Pet>(conn)
    }

    pub fn create_pet<'a>(
        conn: &MysqlConnection,
        name: &'a str,
        owner: &'a str,
        species: &'a str,
    ) -> Result<(), Box<dyn Error>> {
        use super::schema::pets;

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
}

pub mod services {
    type DbPool = diesel::r2d2::Pool<ConnectionManager<MysqlConnection>>;
    use super::data;
    use actix_web::{get, web, HttpResponse, Responder};
    use diesel::r2d2::ConnectionManager;
    use diesel::MysqlConnection;

    #[get("/")]
    pub async fn hello(db: web::Data<DbPool>) -> impl Responder {
        let conn = db.get().expect("could not get pool connection");

        let pets = web::block(move || data::get_pets(&conn).unwrap())
            .await
            .unwrap();

        HttpResponse::Ok().json(pets)
    }
}
