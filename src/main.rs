mod images;
mod api;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::sync::{Arc, RwLock, Mutex};
use serde_json::{Result, Value};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = "0.0.0.0:8002";

    // START HTTP SERVER WITH GLOBAL STATE
    HttpServer::new( move || {  
        App::new()
            .service(api::hi)
            .service(api::get_all_galleries)
            .service(api::get_all_images_names)
            .service(api::get_image)
    })
    .bind(url)?
    .run()
    .await
}