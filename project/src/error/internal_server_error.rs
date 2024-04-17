use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use serde::ser::SerializeMap;
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub struct InternalServerError {
    _p: (),
}

impl InternalServerError {
    pub fn new() -> Self {
        Self { _p: () }
    }
}

impl fmt::Display for InternalServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Internal Server Error")
    }
}

impl std::error::Error for InternalServerError {}

impl Serialize for InternalServerError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_map(Some(2))?;
        s.serialize_entry("status", &500)?;
        s.serialize_entry("message", "Internal Server Error")?;
        s.end()
    }
}

impl ResponseError for InternalServerError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().json(self)
    }
}

impl Responder for InternalServerError {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        self.error_response()
    }
}
