use std::collections::HashMap;
use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;

#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../models/mod.rs"]
mod models;
#[path = "../router.rs"]
mod routers;
#[path = "../state.rs"]
mod state;
#[path = "../errors.rs"]
mod errors;

use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
   let shared_data = web::Data::new(AppState{
        db: Mutex::new(HashMap::new())
    });
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .app_data(web::PayloadConfig::new(1000000 * 250))
            .configure(audio_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}