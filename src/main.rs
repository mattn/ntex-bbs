#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use dotenvy::dotenv;
use ntex::web::{self, Error, HttpResponse};
use ntex_files as fs;
use serde::Deserialize;

use std::collections::HashMap;
use std::env;

mod entry;
mod schema;
use crate::entry::Entry;

embed_migrations!();

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

async fn entries(pool: web::types::State<SqlitePool>) -> Result<HttpResponse, Error> {
    let pool = (*pool).clone();
    let mut context = HashMap::new();
    let entries = web::block(move || Entry::all(pool.get().as_deref().unwrap())).await?;
    context.insert("entries", entries);
    Ok(HttpResponse::Ok().json(&context))
}

#[derive(Deserialize)]
pub struct InsertJSON {
    body: String,
}

async fn insert(
    params: web::types::Json<InsertJSON>,
    pool: web::types::State<SqlitePool>,
) -> Result<HttpResponse, Error> {
    let pool = (*pool).clone();
    let entry = Entry {
        id: None,
        body: params.body.clone(),
    };
    web::block(move || Entry::add(entry, pool.get().as_deref().unwrap())).await?;
    Ok(HttpResponse::Created().body(""))
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    web::HttpServer::new(move || {
        web::App::new().state(pool.clone()).service((
            web::resource("/").route(web::get().to(|| async {
                HttpResponse::Found()
                    .header("LOCATION", "/static/index.html")
                    .finish()
            })),
            web::resource("/entries")
                .route(web::get().to(entries))
                .route(web::post().to(insert)),
            fs::Files::new("/static", "static/"),
        ))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
