use crate::init::AppInit;
use actix_web::HttpServer;

mod calendar;
mod error;
mod handlers;
mod https;
mod init;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let init = AppInit::init();
    let addr = (init.listen_ip.clone(), init.port);

    HttpServer::new(move || {
        init.create_app()
            .service(handlers::cal())
            .service(handlers::health())
            .configure(handlers::static_files)
            .default_service(handlers::default())
    })
    .bind(addr)?
    .run()
    .await
}
