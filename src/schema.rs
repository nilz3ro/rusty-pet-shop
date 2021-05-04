table! {
    pet (pet_id) {
        pet_id -> Integer,
        name -> Nullable<Varchar>,
        owner -> Nullable<Varchar>,
        species -> Nullable<Varchar>,
        sex -> Nullable<Char>,
        birth -> Nullable<Date>,
        death -> Nullable<Date>,
    }
}
