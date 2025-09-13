use std::env;

use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;

use crate::routes::{create_link, list_urls, visit};

mod models;
mod db;
mod schema;
mod routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::establish_connection(&database_url);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(list_urls)
            .service(create_link)
            .service(visit)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
