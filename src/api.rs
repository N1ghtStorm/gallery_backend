use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use crate::images;

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
    let serialized_names = serde_json::to_string(&galleries).unwrap();
    HttpResponse::Ok().body(serialized_names)
}

#[get("/api/galleries/{gallery_name}")]
async fn get_all_images_names(web::Path(gallery_name): web::Path<String>) -> impl Responder {
    let images_names = images::get_all_gallery_image_names(gallery_name);
    let serialized_names = serde_json::to_string(&images_names).unwrap();
    HttpResponse::Ok().body(serialized_names)
}