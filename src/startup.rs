use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    // wrap connection in a smart pointer
    let connection = web::Data::new(connection);
    // Capture `connection` from the surrounding enviroment
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // Get a pointer copy and attach it to the app state
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    // No .await here!
    Ok(server)
}
