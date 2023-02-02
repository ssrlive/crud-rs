use crate::schema;
use diesel::{AsChangeset, Queryable};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Queryable, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = schema::artists)]
pub struct Artist {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub description: String,
}
