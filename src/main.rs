// <main>
use actix_web::{App, HttpServer, guard, middleware, web};
use actix_cors::Cors;
use actix_ratelimit::{RateLimiter, MemoryStore, MemoryStoreActor};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use slog::{Level, slog_info};
use tokio_postgres::NoTls;
use dotenv::dotenv;
use std::time::Duration;

mod config;
mod constants;
mod handlers;
mod models;
mod db;
mod errors;
mod utils;
mod defaults;
mod logging;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = config::Config::from_env().expect("error getting configuration from environment");

    let pool = config.pg.create_pool(NoTls).expect("error creating deadpool postgres database pool");

    // Initialize store
    let store = MemoryStore::new();

    // Set environment variables for logging (consider using slog_envlogger: https://crates.io/crates/slog-envlogger)
    // Might need https://crates.io/crates/slog-scope and https://crates.io/crates/slog-stdlog as additional dependencies
    /*
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    std::env::set_var("RUST_BACKTRACE", "FULL");
    */

    let logger = logging::configure_log();
    logging::set_global_level(Level::Trace);

    slog_info!(logger, "Starting server at https://{}:{}/", config.server.host, config.server.port);

    let mut builder: SslAcceptorBuilder;

    // Development mode
    if cfg!(debug_assertions) {
        builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
        .set_private_key_file("tls/privkey.pem", SslFiletype::PEM)
        .unwrap();
        builder.set_certificate_chain_file("tls/cert.pem").unwrap();
    }
    // Production mode
    else {
        builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
        .set_private_key_file("/root/overdue_backend/tls/privkey.pem", SslFiletype::PEM)
        .unwrap();
        builder.set_certificate_chain_file("/root/overdue_backend/tls/fullchain.pem").unwrap();
    }

    HttpServer::new(move || {
        // TODO: Define Cross-Origin Resource Sharing policy
        /*
        let cors = Cors::default()
              .allowed_origin(constants::GAME_CLIENT_URL_DOMAIN_ORIGIN)
              .allowed_origin_fn(|origin, _req_head| {
                  origin.as_bytes().ends_with(constants::FRONT_DOMAIN)
              })
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::ACCEPT, http::header::CONTENT_TYPE, http::header::CONTENT_LENGTH, http::header::HOST])
              .max_age(3600);
        */

        // Use this permissive policy for debugging phase/development mode
        let cors = Cors::permissive();

        App::new()
            .wrap(middleware::Compress::default())
            // Enable logging
            // .wrap(middleware::Logger::default())
            .wrap(cors)
            // Register the middleware which allows for a maximum of 60 requests per minute per client based on IP address
            .wrap(
                RateLimiter::new(
                MemoryStoreActor::from(store.clone()).start())
                    .with_interval(Duration::from_secs(constants::RATE_LIMIT_INTERVAL_DURATION))
                    .with_max_requests(constants::RATE_LIMIT_MAX_REQUESTS)
            )
            .data(models::AppState {
                pool: pool.clone(),
                log: logger.clone()
            })
            // Define all of the available endpoints
            .service(
                web::resource("/submit_score{_:/?}")
                // Limit size of the payload
                .data(web::JsonConfig::default().limit(constants::INCOMING_SCORE_PAYLOAD_LIMIT))
                .guard(guard::Host(constants::SERVER_HOST_URL))
                .route(web::post().to(handlers::submit_score)))
            .service(
                web::resource("/get_leaderboard{_:/?}")
                // Limit size of the payload
                .data(web::JsonConfig::default().limit(constants::INCOMING_LEADERBOARD_PAYLOAD_LIMIT))
                .guard(guard::Host(constants::SERVER_HOST_URL))
                .route(web::get().to(handlers::get_leaderboard)))
            .service(
                web::resource("/get_materials{_:/?}")
                // Limit size of the payload
                .data(web::JsonConfig::default().limit(constants::INCOMING_MATERIALS_PAYLOAD_LIMIT))
                .guard(guard::Host(constants::SERVER_HOST_URL))
                .route(web::get().to(handlers::get_materials)))
            // Define easter egg endpoints
            .service(
                web::resource("/vsauce{_:/?}")
                .guard(guard::Host(constants::SERVER_HOST_URL))
                .route(web::get().to(handlers::vsauce_handler)))
            .service(
                web::resource("/fortune{_:/?}")
                .guard(guard::Host(constants::SERVER_HOST_URL))
                .route(web::get().to(handlers::fortune_cookie_handler)))
            // Default 404 handler
            .default_service(web::route().to(handlers::default_handler))
    })
    .keep_alive(constants::KEEP_ALIVE_DURATION)
    .bind_openssl(format!("{}:{}", config.server.host, config.server.port), builder)?
    .run()
    .await
}
// </main>

#[cfg(test)]
#[cfg(feature = "integration")]
mod tests;