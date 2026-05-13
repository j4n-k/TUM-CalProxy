use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use std::convert::Infallible;
use std::future::{ready, Ready};

#[derive(Debug)]
pub struct AppData<T>(pub T);

impl<T: Clone + 'static> FromRequest for AppData<T> {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let client = req.app_data::<T>();

        if let Some(client) = client {
            ready(Ok(AppData(client.clone())))
        } else {
            panic!(
                "Data of type {} not found in app data",
                std::any::type_name::<T>()
            );
        }
    }
}
