use diesel::dsl::*;
use diesel::{Insertable, Queryable};

#[derive(Queryable, Debug, Insertable)]
#[diesel(table_name = crate::schema::client)]
pub struct Client {
    id: uuid::Uuid,
    first_name: Option<String>,
    last_name: Option<String>,
    email: String,
    hash_password: Option<String>,
}
