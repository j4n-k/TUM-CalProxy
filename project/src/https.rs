use std::convert::Infallible;
use std::future::{Ready, ready};
use std::ops::Deref;
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use hyper::Client as HyperClient;
use hyper::client::HttpConnector;
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};

#[derive(Clone)]
pub struct Client {
    inner: HyperClient<HttpsConnector<HttpConnector>>,
}

impl Client {
    pub fn new() -> Self {
        let mut connector = HttpConnector::new();
        connector.enforce_http(false);

        let connector = HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .wrap_connector(connector);

        Self {
            inner: HyperClient::builder().build(connector)
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Client {
    type Target = HyperClient<HttpsConnector<HttpConnector>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl FromRequest for Client {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let client = req.app_data::<Client>();

        if let Some(client) = client {
            ready(Ok(client.clone()))
        } else {
            panic!("Client is not configured. Configure with App::app_data(client)");
        }
    }
}