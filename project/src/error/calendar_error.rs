use std::fmt;
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use actix_web::body::BoxBody;
use serde::ser::SerializeMap;
use serde::Serialize;

#[derive(Debug)]
pub struct CalendarError {
    _p: (),
}

impl CalendarError {
    pub fn new() -> CalendarError {
        CalendarError { _p: () }
    }
}

impl fmt::Display for CalendarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid token and/or Student/Person ID was provided")
    }
}

impl std::error::Error for CalendarError {}

impl Serialize for CalendarError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        let mut s = serializer.serialize_map(Some(2))?;
        s.serialize_entry("status", &400)?;
        s.serialize_entry("message", "Invalid token and/or Student/Person ID was provided")?;
        s.end()
    }
}

impl ResponseError for CalendarError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::BadRequest()
            .json(self)
    }
}

impl Responder for CalendarError {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse {
        self.error_response()
    }
}