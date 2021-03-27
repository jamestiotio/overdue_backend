// NOTE: These integration tests are designed to be run
// consecutively/separately/sequentially without any parallelism since they all
// share the same PostgreSQL database state (remember to run these tests using
// only a single one test thread).
use actix_rt;
use actix_web::{dev::Body, guard, test, web, App, FromRequest};
use dotenv::dotenv;
use lazy_static::lazy_static;
use serde_json::json;
use slog::Level;

use crate::{config, constants, errors, handlers, logging, models};

lazy_static! {
    static ref APP_TEST_STATE: models::AppState = {
        dotenv().ok();

        let config =
            config::Config::from_env().expect("error getting configuration from environment");

        let pool = config.configure_pool();

        let logger = logging::configure_log();
        logging::set_global_level(Level::Trace);

        models::AppState {
            pool: pool.clone(),
            log: logger.clone(),
        }
    };
}

#[actix_rt::test]
async fn test_fortune_cookie_handler() {
    let app = App::new().data(APP_TEST_STATE.clone()).service(
        web::resource("/fortune{_:/?}")
            .guard(guard::Host(constants::SERVER_HOST_URL))
            .route(web::get().to(handlers::fortune_cookie_handler)),
    );

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .header("Host", constants::SERVER_HOST_URL)
        .uri("/fortune")
        .to_request();

    let res = test::call_service(&mut app, req).await;

    assert_eq!(
        res.status(),
        200,
        "GET /fortune with correct Host header should return status 200"
    );
}

#[actix_rt::test]
async fn test_vsauce_handler() {
    let app = App::new().data(APP_TEST_STATE.clone()).service(
        web::resource("/vsauce{_:/?}")
            .guard(guard::Host(constants::SERVER_HOST_URL))
            .route(web::get().to(handlers::vsauce_handler)),
    );

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .header("Host", constants::SERVER_HOST_URL)
        .uri("/vsauce")
        .to_request();

    let res = test::call_service(&mut app, req).await;

    assert_eq!(
        res.status(),
        200,
        "GET /vsauce with correct Host header should return status 200"
    );
}

#[actix_rt::test]
async fn test_get_materials_with_no_host_header() {
    let app = App::new().data(APP_TEST_STATE.clone()).service(
        web::resource("/get_materials{_:/?}")
            .guard(guard::Host(constants::SERVER_HOST_URL))
            .route(web::get().to(handlers::get_materials)),
    );

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get().uri("/get_materials").to_request();

    let res = test::call_service(&mut app, req).await;

    assert_eq!(
        res.status(),
        404,
        "GET /get_materials with no Host header should return status 404"
    );
}

#[actix_rt::test]
async fn test_get_materials_with_wrong_host_header() {
    let app = App::new().data(APP_TEST_STATE.clone()).service(
        web::resource("/get_materials{_:/?}")
            .guard(guard::Host(constants::SERVER_HOST_URL))
            .route(web::get().to(handlers::get_materials)),
    );

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .header("Host", "overdue.sutd.edu.sg")
        .uri("/get_materials")
        .to_request();

    let res = test::call_service(&mut app, req).await;

    assert_eq!(
        res.status(),
        404,
        "GET /get_materials with wrong Host header should return status 404"
    );
}

#[actix_rt::test]
async fn test_get_materials_with_correct_header() {
    let app = App::new().data(APP_TEST_STATE.clone()).service(
        web::resource("/get_materials{_:/?}")
            .guard(guard::Host(constants::SERVER_HOST_URL))
            .route(web::get().to(handlers::get_materials)),
    );

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .header("Host", constants::SERVER_HOST_URL)
        .uri("/get_materials")
        .to_request();

    let mut res = test::call_service(&mut app, req).await;

    let body = res.take_body();

    let body = body.as_ref().unwrap();

    assert_eq!(
        res.status(),
        200,
        "GET /get_materials with correct Host header should return status 200"
    );

    // The order is the same as the order of the material entries/rows added to the
    // material SQL table in the setup script.
    assert_eq!(
        &Body::from(
            json!([{"name":"jigsawAcrylic","quantity":0},{"name":"jigsawMetal","quantity":0},{"name":"jigsawWood","quantity":0},{"name":"drilledAcrylic","quantity":0},{"name":"drilledMetal","quantity":0},{"name":"drilledWood","quantity":0},{"name":"acrylicStrips","quantity":0},{"name":"woodStrips","quantity":0},{"name":"threeDPrint","quantity":0},{"name":"solderedPcb","quantity":0}])
        ),
        body
    );
}

#[actix_rt::test]
async fn test_get_leaderboard_with_no_host_header() {
    let app = App::new()
        .data(APP_TEST_STATE.clone())
        .service(
            web::resource("/get_leaderboard{_:/?}")
                .guard(guard::Host(constants::SERVER_HOST_URL))
                .route(web::get().to(handlers::get_leaderboard)),
        )
        .app_data(web::Query::<models::LeaderboardQueryRequest>::configure(
            |cfg| cfg.error_handler(errors::query_error_handler),
        ));

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .uri("/get_leaderboard")
        .to_request();

    let res = test::call_service(&mut app, req).await;

    assert_eq!(
        res.status(),
        404,
        "GET /get_leaderboard with no Host header should return status 404"
    );
}

#[actix_rt::test]
async fn test_get_leaderboard_with_wrong_host_header() {
    let app = App::new()
        .data(APP_TEST_STATE.clone())
        .service(
            web::resource("/get_leaderboard{_:/?}")
                .guard(guard::Host(constants::SERVER_HOST_URL))
                .route(web::get().to(handlers::get_leaderboard)),
        )
        .app_data(web::Query::<models::LeaderboardQueryRequest>::configure(
            |cfg| cfg.error_handler(errors::query_error_handler),
        ));

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .header("Host", "overdue.sutd.edu.sg")
        .uri("/get_leaderboard")
        .to_request();

    let res = test::call_service(&mut app, req).await;

    assert_eq!(
        res.status(),
        404,
        "GET /get_leaderboard with wrong Host header should return status 404"
    );
}

#[actix_rt::test]
async fn test_get_leaderboard_with_correct_host_header() {
    let app = App::new()
        .data(APP_TEST_STATE.clone())
        .service(
            web::resource("/get_leaderboard{_:/?}")
                .guard(guard::Host(constants::SERVER_HOST_URL))
                .route(web::get().to(handlers::get_leaderboard)),
        )
        .app_data(web::Query::<models::LeaderboardQueryRequest>::configure(
            |cfg| cfg.error_handler(errors::query_error_handler),
        ));

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .header("Host", constants::SERVER_HOST_URL)
        .uri("/get_leaderboard")
        .to_request();

    let mut res = test::call_service(&mut app, req).await;

    let body = res.take_body();

    let body = body.as_ref().unwrap();

    assert_eq!(
        res.status(),
        200,
        "GET /get_leaderboard with correct Host header should return status 200"
    );

    assert_eq!(&Body::from(json!([])), body);
}

#[actix_rt::test]
async fn test_get_leaderboard_with_wrong_query_type() {
    let app = App::new()
        .data(APP_TEST_STATE.clone())
        .service(
            web::resource("/get_leaderboard{_:/?}")
                .guard(guard::Host(constants::SERVER_HOST_URL))
                .route(web::get().to(handlers::get_leaderboard)),
        )
        .app_data(web::Query::<models::LeaderboardQueryRequest>::configure(
            |cfg| cfg.error_handler(errors::query_error_handler),
        ));

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .header("Host", constants::SERVER_HOST_URL)
        .uri("/get_leaderboard?difficulty=1&max_entries=normal")
        .to_request();

    let res = test::call_service(&mut app, req).await;

    assert_eq!(
        res.status(),
        400,
        "GET /get_leaderboard with wrong query types should return status 400"
    );
}

#[actix_rt::test]
async fn test_get_leaderboard_with_wrong_difficulty_query_content() {
    let app = App::new()
        .data(APP_TEST_STATE.clone())
        .service(
            web::resource("/get_leaderboard{_:/?}")
                .guard(guard::Host(constants::SERVER_HOST_URL))
                .route(web::get().to(handlers::get_leaderboard)),
        )
        .app_data(web::Query::<models::LeaderboardQueryRequest>::configure(
            |cfg| cfg.error_handler(errors::query_error_handler),
        ));

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .header("Host", constants::SERVER_HOST_URL)
        .uri("/get_leaderboard?difficulty=medium&max_entries=20")
        .to_request();

    let mut res = test::call_service(&mut app, req).await;

    let body = res.take_body();

    let body = body.as_ref().unwrap();

    assert_eq!(
        res.status(),
        400,
        "GET /get_leaderboard with wrong difficulty query content should return status 400"
    );

    assert_eq!(
        &Body::from(
            json!({"code":400,"error":"Validation Error","message":"A validation error has occurred."})
        ),
        body
    );

    let req = test::TestRequest::get()
        .header("Host", constants::SERVER_HOST_URL)
        .uri("/get_leaderboard?difficulty=mediums&max_entries=20")
        .to_request();

    let mut res = test::call_service(&mut app, req).await;

    let body = res.take_body();

    let body = body.as_ref().unwrap();

    assert_eq!(
        res.status(),
        400,
        "GET /get_leaderboard with wrong difficulty query content should return status 400"
    );

    assert_eq!(
        &Body::from(
            json!({"code":400,"error":"Validation Error","message":"A validation error has occurred."})
        ),
        body
    );

    let req = test::TestRequest::get()
        .header("Host", constants::SERVER_HOST_URL)
        .uri("/get_leaderboard?difficulty=20&max_entries=20")
        .to_request();

    let mut res = test::call_service(&mut app, req).await;

    let body = res.take_body();

    let body = body.as_ref().unwrap();

    assert_eq!(
        res.status(),
        400,
        "GET /get_leaderboard with wrong difficulty query content should return status 400"
    );

    assert_eq!(
        &Body::from(
            json!({"code":400,"error":"Validation Error","message":"A validation error has occurred."})
        ),
        body
    );
}

#[actix_rt::test]
async fn test_get_leaderboard_with_wrong_max_entries_query_content() {
    let app = App::new()
        .data(APP_TEST_STATE.clone())
        .service(
            web::resource("/get_leaderboard{_:/?}")
                .guard(guard::Host(constants::SERVER_HOST_URL))
                .route(web::get().to(handlers::get_leaderboard)),
        )
        .app_data(web::Query::<models::LeaderboardQueryRequest>::configure(
            |cfg| cfg.error_handler(errors::query_error_handler),
        ));

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .header("Host", constants::SERVER_HOST_URL)
        .uri("/get_leaderboard?difficulty=normal&max_entries=120")
        .to_request();

    let mut res = test::call_service(&mut app, req).await;

    let body = res.take_body();

    let body = body.as_ref().unwrap();

    assert_eq!(
        res.status(),
        400,
        "GET /get_leaderboard with wrong max_entries query content should return status 400"
    );

    assert_eq!(
        &Body::from(
            json!({"code":400,"error":"Validation Error","message":"A validation error has occurred."})
        ),
        body
    );

    let req = test::TestRequest::get()
        .header("Host", constants::SERVER_HOST_URL)
        .uri("/get_leaderboard?difficulty=normal&max_entries=-20")
        .to_request();

    let mut res = test::call_service(&mut app, req).await;

    let body = res.take_body();

    let body = body.as_ref().unwrap();

    assert_eq!(
        res.status(),
        400,
        "GET /get_leaderboard with wrong max_entries query content should return status 400"
    );

    assert_eq!(
        &Body::from(
            json!({"code":400,"error":"Validation Error","message":"A validation error has occurred."})
        ),
        body
    );

    let req = test::TestRequest::get()
        .header("Host", constants::SERVER_HOST_URL)
        .uri("/get_leaderboard?difficulty=normal&max_entries=0")
        .to_request();

    let mut res = test::call_service(&mut app, req).await;

    let body = res.take_body();

    let body = body.as_ref().unwrap();

    assert_eq!(
        res.status(),
        400,
        "GET /get_leaderboard with wrong max_entries query content should return status 400"
    );

    assert_eq!(
        &Body::from(
            json!({"code":400,"error":"Validation Error","message":"A validation error has occurred."})
        ),
        body
    );
}

#[actix_rt::test]
async fn test_get_leaderboard_with_correct_query() {
    let app = App::new()
        .data(APP_TEST_STATE.clone())
        .service(
            web::resource("/get_leaderboard{_:/?}")
                .guard(guard::Host(constants::SERVER_HOST_URL))
                .route(web::get().to(handlers::get_leaderboard)),
        )
        .app_data(web::Query::<models::LeaderboardQueryRequest>::configure(
            |cfg| cfg.error_handler(errors::query_error_handler),
        ));

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .header("Host", constants::SERVER_HOST_URL)
        .uri("/get_leaderboard?difficulty=normal&max_entries=20")
        .to_request();

    let res = test::call_service(&mut app, req).await;

    assert_eq!(
        res.status(),
        200,
        "GET /get_leaderboard with correct query should return status 200"
    );

    let req = test::TestRequest::get()
        .header("Host", constants::SERVER_HOST_URL)
        .uri("/get_leaderboard?difficulty=normal&max_entries=002")
        .to_request();

    let mut res = test::call_service(&mut app, req).await;

    let body = res.take_body();

    let body = body.as_ref().unwrap();

    assert_eq!(
        res.status(),
        200,
        "GET /get_leaderboard with correct query should return status 200"
    );

    assert_eq!(&Body::from(json!([])), body);
}

#[actix_rt::test]
async fn test_submit_score_with_no_host_header() {
    let app = App::new()
        .data(APP_TEST_STATE.clone())
        .service(
            web::resource("/submit_score{_:/?}")
                .data(web::JsonConfig::default().limit(constants::INCOMING_SCORE_PAYLOAD_LIMIT))
                .guard(guard::Host(constants::SERVER_HOST_URL))
                .route(web::post().to(handlers::submit_score)),
        )
        .app_data(web::Json::<models::ScoreEntry>::configure(|cfg| {
            // Limit size of the payload.
            cfg.limit(constants::INCOMING_SCORE_PAYLOAD_LIMIT)
                .error_handler(errors::json_error_handler)
        }));

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::post()
        .header("Content-Type", "application/json")
        .uri("/submit_score")
        .set_payload(json!({}).to_string())
        .to_request();

    let res = test::call_service(&mut app, req).await;

    assert_eq!(
        res.status(),
        404,
        "POST /submit_score with no Host header should return status 404"
    );
}

#[actix_rt::test]
async fn test_submit_score_with_wrong_host_header() {
    let app = App::new()
        .data(APP_TEST_STATE.clone())
        .service(
            web::resource("/submit_score{_:/?}")
                .data(web::JsonConfig::default().limit(constants::INCOMING_SCORE_PAYLOAD_LIMIT))
                .guard(guard::Host(constants::SERVER_HOST_URL))
                .route(web::post().to(handlers::submit_score)),
        )
        .app_data(web::Json::<models::ScoreEntry>::configure(|cfg| {
            // Limit size of the payload.
            cfg.limit(constants::INCOMING_SCORE_PAYLOAD_LIMIT)
                .error_handler(errors::json_error_handler)
        }));

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::post()
        .header("Host", "overdue.sutd.edu.sg")
        .header("Content-Type", "application/json")
        .uri("/submit_score")
        .set_payload(json!({}).to_string())
        .to_request();

    let res = test::call_service(&mut app, req).await;

    assert_eq!(
        res.status(),
        404,
        "POST /submit_score with wrong Host header should return status 404"
    );
}

#[actix_rt::test]
async fn test_submit_score_with_empty_json() {
    let app = App::new()
        .data(APP_TEST_STATE.clone())
        .service(
            web::resource("/submit_score{_:/?}")
                .data(web::JsonConfig::default().limit(constants::INCOMING_SCORE_PAYLOAD_LIMIT))
                .guard(guard::Host(constants::SERVER_HOST_URL))
                .route(web::post().to(handlers::submit_score)),
        )
        .app_data(web::Json::<models::ScoreEntry>::configure(|cfg| {
            // Limit size of the payload.
            cfg.limit(constants::INCOMING_SCORE_PAYLOAD_LIMIT)
                .error_handler(errors::json_error_handler)
        }));

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::post()
        .header("Host", constants::SERVER_HOST_URL)
        .header("Content-Type", "application/json")
        .uri("/submit_score")
        .set_payload(json!({}).to_string())
        .to_request();

    let res = test::call_service(&mut app, req).await;

    assert_eq!(
        res.status(),
        400,
        "POST /submit_score with empty JSON payload should return status 400"
    );
}

#[actix_rt::test]
async fn test_submit_score_with_malformed_json() {
    let app = App::new()
        .data(APP_TEST_STATE.clone())
        .service(
            web::resource("/submit_score{_:/?}")
                .data(web::JsonConfig::default().limit(constants::INCOMING_SCORE_PAYLOAD_LIMIT))
                .guard(guard::Host(constants::SERVER_HOST_URL))
                .route(web::post().to(handlers::submit_score)),
        )
        .app_data(web::Json::<models::ScoreEntry>::configure(|cfg| {
            // Limit size of the payload.
            cfg.limit(constants::INCOMING_SCORE_PAYLOAD_LIMIT)
                .error_handler(errors::json_error_handler)
        }));

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::post()
        .header("Host", constants::SERVER_HOST_URL)
        .header("Content-Type", "application/json")
        .uri("/submit_score")
        .set_payload(json!({"name":"JRT","gender":"M","email":"james_raphael@mymail.sutd.edu.sg","difficulty":1,"score":0,"bonus":0,"materials":[]}).to_string())
        .to_request();

    let res = test::call_service(&mut app, req).await;

    assert_eq!(
        res.status(),
        400,
        "POST /submit_score with malformed JSON payload should return status 400"
    );
}

#[actix_rt::test]
async fn test_submit_score_with_invalid_json() {
    let app = App::new()
        .data(APP_TEST_STATE.clone())
        .service(
            web::resource("/submit_score{_:/?}")
                .data(web::JsonConfig::default().limit(constants::INCOMING_SCORE_PAYLOAD_LIMIT))
                .guard(guard::Host(constants::SERVER_HOST_URL))
                .route(web::post().to(handlers::submit_score)),
        )
        .app_data(web::Json::<models::ScoreEntry>::configure(|cfg| {
            // Limit size of the payload.
            cfg.limit(constants::INCOMING_SCORE_PAYLOAD_LIMIT)
                .error_handler(errors::json_error_handler)
        }));

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::post()
        .header("Host", constants::SERVER_HOST_URL)
        .header("Content-Type", "application/json")
        .uri("/submit_score")
        .set_payload(json!({"name":"JRT","gender":"M","email":"james_raphael@mymail.sutd.edu.sg","difficulty":"easy","score":1000,"bonus":5,"materials":[{"name":"drilledMetal","quantity":7},{"name":"acrylicStrips","quantity":5},{"name":"woodStrips","quantity":3}]}).to_string())
        .to_request();

    let mut res = test::call_service(&mut app, req).await;

    let body = res.take_body();

    let body = body.as_ref().unwrap();

    assert_eq!(
        res.status(),
        400,
        "POST /submit_score with invalid JSON payload should return status 400"
    );

    assert_eq!(
        &Body::from(
            json!({"code":400,"error":"Validation Error","message":"A validation error has occurred."})
        ),
        body
    );
}
