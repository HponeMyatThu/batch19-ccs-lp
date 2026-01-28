use actix_web::{App, HttpServer};
mod db;
mod models;
mod routes;

use crate::db::sqlite::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let conn = db::sqlite::init().expect("DB init failed");

    let state = actix_web::web::Data::new(AppState {
        conn: std::sync::Mutex::new(conn),
    });

    println!("Server running at http://0.0.0.0:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(routes::master_content::init_routes)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
