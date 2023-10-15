use std::fmt;
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use actix_web::body::BoxBody;
use serde::ser::SerializeMap;
use serde::Serialize;

#[derive(Debug)]
pub struct QueryError {
    missing_parameter: String,
}

impl QueryError {
    pub fn new(missing_parameter: String) -> Self {
        Self { missing_parameter }
    }
}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "missing required query parameter: {}", self.missing_parameter)
    }
}

impl std::error::Error for QueryError {}

impl Serialize for QueryError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        let mut s = serializer.serialize_map(Some(3))?;
        s.serialize_entry("status", &400)?;
        s.serialize_entry("message", "Missing Required Query Parameter")?;
        s.serialize_entry("missing_parameter", &self.missing_parameter)?;
        s.end()
    }
}

impl ResponseError for QueryError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::BadRequest()
            .json(self)
    }
}

impl Responder for QueryError {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse {
        self.error_response()
    }
}