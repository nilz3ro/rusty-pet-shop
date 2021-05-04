use super::schema::pets;
use diesel::Queryable;

#[derive(Queryable, Debug)]
pub struct Pet {
    pub pet_id: i32,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub species: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "pets"]
pub struct NewPet<'a> {
    pub name: &'a str,
    pub owner: &'a str,
    pub species: &'a str,
}
