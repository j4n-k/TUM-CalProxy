use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::web::QueryConfig;
use actix_web::App;
use std::env;
use tracing::error;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

use crate::error::{InternalServerError, QueryError};

use crate::https::Client;

#[derive(Clone)]
pub struct AppInit {
    client: Client,
    query_config: QueryConfig,
    pub port: u16,
    pub listen_ip: String,
}

impl AppInit {
    pub fn init() -> Self {
        init_logging();

        let port = env::var("ACTIX_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8080);

        let listen_ip = env::var("ACTIX_LISTEN")
            .ok()
            .unwrap_or("0.0.0.0".to_string());

        Self {
            client: Client::new(),
            query_config: query_config(),
            port,
            listen_ip,
        }
    }

    pub fn create_app(
        &self,
    ) -> App<
        impl ServiceFactory<
            ServiceRequest,
            Config = (),
            Response = ServiceResponse<impl MessageBody>,
            Error = actix_web::Error,
            InitError = (),
        >,
    > {
        App::new()
            .app_data(self.client.clone())
            .app_data(self.query_config.clone())
    }
}

fn init_logging() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(tracing::Level::INFO.into())
                .with_env_var("RUST_LOG")
                .from_env_lossy(),
        )
        .init();
}

fn query_config() -> QueryConfig {
    QueryConfig::default().error_handler(|err, _| {
        let e = err.to_string().replace("Query deserialize error: ", "");
        if e.starts_with("missing field `") {
            let missing_parameter = e
                .trim_start_matches("missing field `")
                .trim_end_matches("`");

            return QueryError::new(missing_parameter.to_string()).into();
        }

        error!("Unknown error happened during query deserialization: {}", e);
        InternalServerError::new().into()
    })
}
