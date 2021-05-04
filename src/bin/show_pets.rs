// extern crate diesel;
// extern crate gasoline;

use diesel::prelude::*;
use gasoline::*;
use models::*;

fn main() {
    use gasoline::schema::pets::dsl::*;

    let connection = establish_connection();
    let results = pets
        .filter(species.eq_all("dog"))
        .limit(5)
        .load::<Pet>(&connection)
        .expect("cant get pets");

    println!("got pets! {:?}", results);
}
