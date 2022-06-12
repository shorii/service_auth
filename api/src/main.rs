use actix_web::{web, App, HttpServer};
use state::AppState;

mod controller;
mod domain;
mod infra;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // FIXME
    let conn = sea_orm::Database::connect("postgresql://postgres:postgres@localhost/talk_system")
        .await
        .unwrap();
    let state = AppState { conn };
    HttpServer::new(move || {
        App::new().service(
            web::scope("/user")
                .app_data(web::Data::new(state.clone()))
                .configure(controller::init_routes),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
