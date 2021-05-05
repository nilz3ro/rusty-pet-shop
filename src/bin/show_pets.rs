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

    results.iter().for_each(|result| {
        println!(
            "name: {:?}, species: {:?}, owner: {:?}",
            result.name, result.species, result.owner
        );
    });
}
