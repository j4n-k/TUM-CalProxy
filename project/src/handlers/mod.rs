use actix_web::{web, Route};

use crate::error;

pub mod cal;
mod health;
mod static_files;

pub use cal::service as cal;
pub use health::service as health;
pub use static_files::configure as static_files;

pub fn default() -> Route {
    web::route().to(|| async { error::NotFound })
}
