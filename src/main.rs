mod images;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::sync::{Arc, RwLock, Mutex};
use serde_json::{Result, Value};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = "0.0.0.0:8002";

    // START HTTP SERVER WITH GLOBAL STATE
    HttpServer::new( move || {  
        App::new()
            .service(hi)
            .service(get_all_galleries)
    })
    .bind(url)?
    .run()
    .await
}

/// Healthcheck endpoint
#[get("/info")]
async fn hi() -> impl Responder {
    HttpResponse::Ok().body("
        IMAGE GALLERY READY
    ")
}

#[get("/api/galleries")]
async fn get_all_galleries() -> impl Responder {
    let galleries = images::get_all_galleries();
    let serialized = serde_json::to_string(&galleries).unwrap();
    HttpResponse::Ok().body(serialized)
}

#[get("/api/image_names/{gallery_name}")]
async fn get_all_images_name(web::Path(gallery_name): web::Path<String>) -> impl Responder {


    HttpResponse::Ok().body("
        IMAGE GALLERY READY
    ")
}