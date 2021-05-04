use diesel::Queryable;

#[derive(Queryable, Debug)]
pub struct Pet {
    pub pet_id: i32,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub species: Option<String>,
}
