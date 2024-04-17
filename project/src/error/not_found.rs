use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use serde::ser::SerializeMap;
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub struct NotFound;

impl fmt::Display for NotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Not Found")
    }
}

impl std::error::Error for NotFound {}

impl Serialize for NotFound {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_map(Some(2))?;
        s.serialize_entry("status", &404)?;
        s.serialize_entry("message", "Not Found")?;
        s.end()
    }
}

impl ResponseError for NotFound {
    fn status_code(&self) -> StatusCode {
        StatusCode::NOT_FOUND
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::NotFound().json(self)
    }
}

impl Responder for NotFound {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        self.error_response()
    }
}
