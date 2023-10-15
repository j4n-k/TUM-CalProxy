use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use hyper::header::ALLOW;
use hyper::Method;
use serde::Serialize;

#[derive(Debug)]
pub struct MethodNotAvailable {
    allowed_methods: &'static [&'static Method],
}

impl MethodNotAvailable {
    pub fn new(allowed_methods: &'static [&'static Method]) -> Self {
        Self { allowed_methods }
    }

    fn allowed_methods_str(&self) -> String {
        self.allowed_methods.iter()
            .map(|m| m.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn allowed_methods_str_vec(&self) -> Vec<&str> {
        self.allowed_methods.iter()
            .map(|m| m.as_str())
            .collect::<Vec<_>>()
    }
}

impl std::fmt::Display for MethodNotAvailable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Method Not Allowed, allowed methods: {}", self.allowed_methods_str())
    }
}

impl std::error::Error for MethodNotAvailable {}

impl Serialize for MethodNotAvailable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        use serde::ser::SerializeMap;
        let mut s = serializer.serialize_map(Some(3))?;
        s.serialize_entry("status", &405)?;
        s.serialize_entry("message", "Method Not Allowed")?;
        s.serialize_entry("allowed_methods", &self.allowed_methods_str_vec())?;
        s.end()
    }
}

impl ResponseError for MethodNotAvailable {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::METHOD_NOT_ALLOWED
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::MethodNotAllowed()
            .append_header((
                ALLOW,
                self.allowed_methods_str()
            ))
            .json(self)
    }
}

impl Responder for MethodNotAvailable {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        self.error_response()
    }
}