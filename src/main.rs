pub mod api;
pub mod services;

use crate::api::account::login;
use actix_web::{web, App, HttpServer};
use sqlx::{Pool, Sqlite};

struct AppData {
    pool: Pool<Sqlite>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_uri = "";

    let data = web::Data::new(AppData {
        pool: Pool::connect(database_uri).await.unwrap(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(web::scope("/api").service(web::scope("/account").service(login)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
