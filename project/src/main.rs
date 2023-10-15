use actix_web::HttpServer;
use crate::init::AppInit;

mod handlers;
mod https;
mod init;
mod calendar;
mod error;

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