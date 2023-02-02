pub mod models;

pub mod schema {
    table! {
        artists (id) {
            id -> Int4,
            name -> Varchar,
            description -> Text,
        }
    }
}

#[macro_use]
extern crate diesel;
use rocket::serde::{Deserialize, Serialize};
use rocket_sync_db_pools::database;

#[database("db")]
pub struct DbConn(diesel::PgConnection);

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
    pub details: String,
}
