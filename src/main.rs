#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use diesel::{r2d2::ConnectionManager, MysqlConnection};
use dotenv::dotenv;
use gasoline::services;
use std::env;
use std::io;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("missing DATABASE_URL env var.");
    let connection_manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = diesel::r2d2::Pool::builder()
        .max_size(8)
        .build(connection_manager)
        .expect("could not create db connection pool.");

    HttpServer::new(move || App::new().service(services::hello).data(pool.clone()))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
