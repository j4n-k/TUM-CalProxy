mod not_found;
mod method_not_available;
mod internal_server_error;
mod query_error;
mod calendar_error;

pub use not_found::NotFound;
pub use method_not_available::MethodNotAvailable;
pub use internal_server_error::InternalServerError;
pub use query_error::QueryError;
pub use calendar_error::CalendarError;