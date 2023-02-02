use crud_rs::{models::Artist, schema::artists, ApiError, DbConn};
use diesel::prelude::*;
use rocket::{
    response::status::{Created, NoContent, NotFound},
    serde::json::Json,
};

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        // State
        .attach(DbConn::fairing())
        // Routes
        .mount(
            "/artists",
            rocket::routes![list, retrieve, create, update, destroy],
        )
}

#[rocket::get("/")]
async fn list(connection: DbConn) -> Json<Vec<Artist>> {
    connection
        .run(|c| artists::table.load(c))
        .await
        .map(Json)
        .expect("Failed to fetch artists")
}

#[rocket::get("/<id>")]
async fn retrieve(connection: DbConn, id: i32) -> Result<Json<Artist>, NotFound<Json<ApiError>>> {
    connection
        .run(move |c| artists::table.filter(artists::id.eq(id)).first(c))
        .await
        .map(Json)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}

#[rocket::post("/", data = "<artist>")]
async fn create(
    connection: DbConn,
    artist: Json<Artist>,
) -> Result<Created<Json<Artist>>, Json<ApiError>> {
    #[derive(Insertable)]
    #[diesel(table_name = artists)]
    pub struct NewArtist {
        pub name: String,
        pub description: String,
    }

    connection
        .run(move |c| {
            let artist = NewArtist {
                name: artist.name.clone(),
                description: artist.description.clone(),
            };
            diesel::insert_into(artists::table)
                .values(&artist)
                .get_result(c)
        })
        .await
        .map(|a| Created::new("/").body(Json(a)))
        .map_err(|e| {
            Json(ApiError {
                details: e.to_string(),
            })
        })
}

#[rocket::patch("/<id>", data = "<artist>")]
async fn update(
    connection: DbConn,
    id: i32,
    artist: Json<Artist>,
) -> Result<Json<Artist>, NotFound<Json<ApiError>>> {
    connection
        .run(move |c| {
            let mut artist = artist.into_inner();
            artist.id = id;
            diesel::update(artists::table.find(id))
                .set(&artist)
                .get_result(c)
        })
        .await
        .map(Json)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}

#[rocket::delete("/<id>")]
async fn destroy(connection: DbConn, id: i32) -> Result<NoContent, NotFound<Json<ApiError>>> {
    connection
        .run(move |c| {
            let affected = diesel::delete(artists::table.filter(artists::id.eq(id)))
                .execute(c)
                .expect("Connection is broken");
            match affected {
                1 => Ok(()),
                0 => Err("NotFound"),
                _ => Err("???"),
            }
        })
        .await
        .map(|_| NoContent)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}
