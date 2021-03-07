// <main>
use actix_web::{App, FromRequest, HttpServer, guard, http, middleware, web};
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
        // Define Cross-Origin Resource Sharing policy
        let cors = Cors::default()
              .allowed_origin(constants::GAME_CLIENT_URL_DOMAIN_ORIGIN)
              .allowed_origin(constants::PUBLIC_FACING_GAME_CLIENT_URL)
              .allowed_origin(constants::FRONT_DOMAIN)
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::CONTENT_LENGTH, http::header::HOST, http::header::USER_AGENT, http::header::ORIGIN, http::header::CONNECTION, http::header::ACCEPT, http::header::ACCEPT_ENCODING, http::header::ACCEPT_LANGUAGE, http::header::ACCEPT_CHARSET, http::header::DNT, http::header::REFERER, http::header::UPGRADE, http::header::UPGRADE_INSECURE_REQUESTS, http::header::STRICT_TRANSPORT_SECURITY, http::header::CONTENT_SECURITY_POLICY, http::header::X_XSS_PROTECTION])
              .max_age(constants::CORS_MAX_AGE_DURATION);

        // Use this permissive policy for debugging phase/development mode
        // let cors = Cors::permissive();

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
                .data(web::JsonConfig::default().limit(constants::INCOMING_SCORE_PAYLOAD_LIMIT))
                .guard(guard::Host(constants::SERVER_HOST_URL))
                .route(web::post().to(handlers::submit_score)))
                .app_data(
                    web::Json::<models::ScoreEntry>::configure(|cfg| {
                        // Limit size of the payload
                        cfg.limit(constants::INCOMING_SCORE_PAYLOAD_LIMIT)
                           .error_handler(errors::json_error_handler)
                    })
                )
            .service(
                web::resource("/get_leaderboard{_:/?}")
                .guard(guard::Host(constants::SERVER_HOST_URL))
                .route(web::get().to(handlers::get_leaderboard)))
                .app_data(
                    web::Query::<models::LeaderboardQueryRequest>::configure(|cfg| {
                        cfg.error_handler(errors::query_error_handler)
                    })
                )
            .service(
                web::resource("/get_materials{_:/?}")
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
            // Serve favicon image
            .service(
                web::resource("/favicon.ico")
                .guard(guard::Host(constants::SERVER_HOST_URL))
                .route(web::get().to(handlers::favicon_handler)))
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