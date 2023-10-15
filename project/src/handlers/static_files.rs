use actix_web::{HttpResponse, web};
use actix_web::http::{header, Method};
use actix_web::web::ServiceConfig;

use crate::error;

macro_rules! handler {
    ($name:literal, $mime:literal) => {
        web::get().to(|| async {
            const FILE: &'static [u8] = include_bytes!(concat!("../../../static/", $name));

            HttpResponse::Ok()
                .append_header((header::CONTENT_TYPE, $mime))
                .body(FILE)
        })
    }
}

macro_rules! default_handler {
    () => {
        web::route().to(|| async {
            error::MethodNotAvailable::new(&[&Method::GET])
        })
    }
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(handler!("index.html", "text/html"))
            .default_service(default_handler!())
    ).service(
        web::resource("favicon.ico")
            .route(handler!("favicon.ico", "image/x-icon"))
            .default_service(default_handler!())
    ).service(
        web::resource("favicon-16x16.png")
            .route(handler!("favicon-16x16.png", "image/png"))
            .default_service(default_handler!())
    ).service(
        web::resource("favicon-32x32.png")
            .route(handler!("favicon-32x32.png", "image/png"))
            .default_service(default_handler!())
    );
}