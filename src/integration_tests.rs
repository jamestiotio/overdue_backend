use actix_rt;
use actix_web::{test, web, App, guard, FromRequest};
use slog::Level;
use dotenv::dotenv;
use lazy_static::lazy_static;
use serde_json::json;

use crate::models;
use crate::config;
use crate::logging;
use crate::constants;
use crate::handlers;
use crate::errors;

// TODO: Add integration tests for each of the 3 main handlers

lazy_static! {
    static ref APP_STATE: models::AppState = {
        dotenv().ok();

        let config = config::Config::from_env().expect("error getting configuration from environment");

        let pool = config.configure_pool();

        let logger = logging::configure_log();
        logging::set_global_level(Level::Trace);

        models::AppState {
            pool: pool.clone(),
            log: logger.clone()
        }
    };
}

#[actix_rt::test]
async fn test_get_materials_with_no_host_header() {
    let app = App::new()
        .data(APP_STATE.clone())
        .service(
            web::resource("/get_materials{_:/?}")
            .guard(guard::Host(constants::SERVER_HOST_URL))
            .route(web::get().to(handlers::get_materials)));

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .uri("/get_materials")
        .to_request();

    let res = test::call_service(&mut app, req).await;

    assert_eq!(res.status(), 404, "GET /get_materials with no Host header should return status 404");
}

#[actix_rt::test]
async fn test_get_materials_with_wrong_host_header() {
    let app = App::new()
        .data(APP_STATE.clone())
        .service(
            web::resource("/get_materials{_:/?}")
            .guard(guard::Host(constants::SERVER_HOST_URL))
            .route(web::get().to(handlers::get_materials)));

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .header("Host", "overdue.sutd.edu.sg")
        .uri("/get_materials")
        .to_request();

    let res = test::call_service(&mut app, req).await;

    assert_eq!(res.status(), 404, "GET /get_materials with wrong Host header should return status 404");
}

#[actix_rt::test]
async fn test_get_materials_with_correct_header() {
    let app = App::new()
        .data(APP_STATE.clone())
        .service(
            web::resource("/get_materials{_:/?}")
            .guard(guard::Host(constants::SERVER_HOST_URL))
            .route(web::get().to(handlers::get_materials)));

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .header("Host", constants::SERVER_HOST_URL)
        .uri("/get_materials")
        .to_request();

    let res = test::call_service(&mut app, req).await;

    assert_eq!(res.status(), 200, "GET /get_materials with correct Host header should return status 200");
}

#[actix_rt::test]
async fn test_get_leaderboard_with_no_host_header() {
    let app = App::new()
        .data(APP_STATE.clone())
        .service(
            web::resource("/get_leaderboard{_:/?}")
            .guard(guard::Host(constants::SERVER_HOST_URL))
            .route(web::get().to(handlers::get_leaderboard)))
            .app_data(
                web::Query::<models::LeaderboardQueryRequest>::configure(|cfg| {
                    cfg.error_handler(errors::query_error_handler)
                })
            );

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .uri("/get_leaderboard")
        .to_request();

    let res = test::call_service(&mut app, req).await;

    assert_eq!(res.status(), 404, "GET /get_leaderboard with no Host header should return status 404");
}

#[actix_rt::test]
async fn test_get_leaderboard_with_wrong_host_header() {
    let app = App::new()
        .data(APP_STATE.clone())
        .service(
            web::resource("/get_leaderboard{_:/?}")
            .guard(guard::Host(constants::SERVER_HOST_URL))
            .route(web::get().to(handlers::get_leaderboard)))
            .app_data(
                web::Query::<models::LeaderboardQueryRequest>::configure(|cfg| {
                    cfg.error_handler(errors::query_error_handler)
                })
            );

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .header("Host", "overdue.sutd.edu.sg")
        .uri("/get_leaderboard")
        .to_request();

    let res = test::call_service(&mut app, req).await;

    assert_eq!(res.status(), 404, "GET /get_leaderboard with wrong Host header should return status 404");
}

#[actix_rt::test]
async fn test_get_leaderboard_with_correct_host_header() {
    let app = App::new()
        .data(APP_STATE.clone())
        .service(
            web::resource("/get_leaderboard{_:/?}")
            .guard(guard::Host(constants::SERVER_HOST_URL))
            .route(web::get().to(handlers::get_leaderboard)))
            .app_data(
                web::Query::<models::LeaderboardQueryRequest>::configure(|cfg| {
                    cfg.error_handler(errors::query_error_handler)
                })
            );

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .header("Host", constants::SERVER_HOST_URL)
        .uri("/get_leaderboard")
        .to_request();

    let res = test::call_service(&mut app, req).await;

    assert_eq!(res.status(), 200, "GET /get_leaderboard with correct Host header should return status 200");
}

#[actix_rt::test]
async fn test_get_leaderboard_with_wrong_query_type() {
    let app = App::new()
        .data(APP_STATE.clone())
        .service(
            web::resource("/get_leaderboard{_:/?}")
            .guard(guard::Host(constants::SERVER_HOST_URL))
            .route(web::get().to(handlers::get_leaderboard)))
            .app_data(
                web::Query::<models::LeaderboardQueryRequest>::configure(|cfg| {
                    cfg.error_handler(errors::query_error_handler)
                })
            );

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .header("Host", constants::SERVER_HOST_URL)
        .uri("/get_leaderboard?difficulty=1&max_entries=normal")
        .to_request();

    let res = test::call_service(&mut app, req).await;

    assert_eq!(res.status(), 400, "GET /get_leaderboard with wrong query types should return status 400");
}

#[actix_rt::test]
async fn test_get_leaderboard_with_wrong_difficulty_query_content() {
    let app = App::new()
        .data(APP_STATE.clone())
        .service(
            web::resource("/get_leaderboard{_:/?}")
            .guard(guard::Host(constants::SERVER_HOST_URL))
            .route(web::get().to(handlers::get_leaderboard)))
            .app_data(
                web::Query::<models::LeaderboardQueryRequest>::configure(|cfg| {
                    cfg.error_handler(errors::query_error_handler)
                })
            );

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .header("Host", constants::SERVER_HOST_URL)
        .uri("/get_leaderboard?difficulty=medium&max_entries=20")
        .to_request();

    let res = test::call_service(&mut app, req).await;

    assert_eq!(res.status(), 400, "GET /get_leaderboard with wrong difficulty query content should return status 400");
}

#[actix_rt::test]
async fn test_get_leaderboard_with_wrong_max_entries_query_content() {
    let app = App::new()
        .data(APP_STATE.clone())
        .service(
            web::resource("/get_leaderboard{_:/?}")
            .guard(guard::Host(constants::SERVER_HOST_URL))
            .route(web::get().to(handlers::get_leaderboard)))
            .app_data(
                web::Query::<models::LeaderboardQueryRequest>::configure(|cfg| {
                    cfg.error_handler(errors::query_error_handler)
                })
            );

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .header("Host", constants::SERVER_HOST_URL)
        .uri("/get_leaderboard?difficulty=normal&max_entries=120")
        .to_request();

    let res = test::call_service(&mut app, req).await;

    assert_eq!(res.status(), 400, "GET /get_leaderboard with wrong max_entries query content should return status 400");

    let req = test::TestRequest::get()
        .header("Host", constants::SERVER_HOST_URL)
        .uri("/get_leaderboard?difficulty=normal&max_entries=-20")
        .to_request();

    let res = test::call_service(&mut app, req).await;

    assert_eq!(res.status(), 400, "GET /get_leaderboard with wrong max_entries query content should return status 400");

    let req = test::TestRequest::get()
        .header("Host", constants::SERVER_HOST_URL)
        .uri("/get_leaderboard?difficulty=normal&max_entries=0")
        .to_request();

    let res = test::call_service(&mut app, req).await;

    assert_eq!(res.status(), 400, "GET /get_leaderboard with wrong max_entries query content should return status 400");
}

#[actix_rt::test]
async fn test_get_leaderboard_with_correct_query() {
    let app = App::new()
        .data(APP_STATE.clone())
        .service(
            web::resource("/get_leaderboard{_:/?}")
            .guard(guard::Host(constants::SERVER_HOST_URL))
            .route(web::get().to(handlers::get_leaderboard)))
            .app_data(
                web::Query::<models::LeaderboardQueryRequest>::configure(|cfg| {
                    cfg.error_handler(errors::query_error_handler)
                })
            );

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .header("Host", constants::SERVER_HOST_URL)
        .uri("/get_leaderboard?difficulty=normal&max_entries=20")
        .to_request();

    let res = test::call_service(&mut app, req).await;

    assert_eq!(res.status(), 200, "GET /get_leaderboard with correct query should return status 200");

    let req = test::TestRequest::get()
        .header("Host", constants::SERVER_HOST_URL)
        .uri("/get_leaderboard?difficulty=normal&max_entries=002")
        .to_request();

    let res = test::call_service(&mut app, req).await;

    assert_eq!(res.status(), 200, "GET /get_leaderboard with correct query should return status 200");
}