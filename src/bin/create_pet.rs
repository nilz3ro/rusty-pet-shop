use diesel::prelude::*;
use gasoline::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("What is the pet's name?");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    println!("What is the pets species?");
    let mut species = String::new();
    stdin().read_line(&mut species).unwrap();
    let species = species.trim();

    println!("Who is the species owner?");
    let mut owner = String::new();
    stdin().read_line(&mut owner).unwrap();
    let owner = owner.trim();

    data::create_pet(&connection, name, owner, species).expect("could not create pet.");
    println!("Created a new pet!");

    #[cfg(not(windows))]
    const EOF: &'static str = "CTRL+D";

    #[cfg(windows)]
    const EOF: &'static str = "CTRL+Z";
}
