use actix_web::{
    web, HttpResponse, Responder, http::StatusCode
};
use deadpool_postgres::{Client, Pool};
use validator::Validate;
use actix_files as fs;
use slog::{o, crit, error, Logger};
use std::{io, process::Command};

use crate::models;
use crate::db;
use crate::utils;
use crate::constants;
use crate::errors::CustomError;

pub async fn get_client(pool: Pool, log: Logger) -> Result<Client, CustomError> {
    pool.get().await
        .map_err(|err| {
            let sublog = log.new(o!("cause" => err.to_string()));
            crit!(sublog, "Error creating database client");

            CustomError::DbError
        })
}

pub fn log_error(log: Logger) -> impl Fn(CustomError) -> CustomError {
    move |err| {
        let sublog = log.new(o!("cause" => err.to_string()));
        error!(sublog, "{}", err.name());
        err
    }
}

// This handler uses JSON extractor with limit
pub async fn submit_score(state: web::Data<models::AppState>, item: web::Json<models::ScoreEntry>) -> Result<impl Responder, CustomError> {
    // Validate JSON input payload
    match item.validate() {
        Ok(_) => (),
        Err(_e) => return Err(CustomError::ValidationError)
    }

    // Validate each material
    if !item.materials.is_empty() {
        for material in item.materials.clone().iter() {
            match material.validate() {
                Ok(_) => (),
                Err(_e) => return Err(CustomError::ValidationError)
            }
        }
    }

    let log = state.log.new(o!("handler" => "submit_score"));

    let client: Client = get_client(state.pool.clone(), log.clone()).await?;

    // By default, return error
    let mut result: Result<Vec<models::LeaderboardSingleEntry>, CustomError> = Err(CustomError::Internal);

    let values = utils::get_values_of_materials(&client).await.map_err(|_err| CustomError::DbError)?;

    // Pass the fetched material names from database for input verification purposes
    let allowed_to_add_score: bool = utils::check_if_materials_total_value_sum_up_to_score(item.score, item.materials.clone(), item.bonus, values.clone()).await.map_err(|_err| CustomError::ValidationError)?;

    if allowed_to_add_score {
        // Do not need to add any materials to aggregate if material vector is empty
        let added_to_materials: bool = if !item.materials.is_empty() { utils::add_materials_to_aggregate(&client, item.materials.clone()).await.map_err(|_err| CustomError::DbError)? } else { true };

        // Only add score entry if there are materials added to the material table
        if added_to_materials {
            result = Ok(db::add_score_entry(&client, item).await.map_err(|_err| CustomError::DbError)?);
        }
    } else {
        return Err(CustomError::ValidationError);
    }

    // Echo JSON response partially back if everything is okay (follow standard military communication procedure, protocol & etiquette)
    result.map(|score| HttpResponse::Ok().header("Content-Security-Policy", "default-src 'self'").header("Strict-Transport-Security", "max-age=3600").header("X-XSS-Protection", "1; mode=block").json(score.get(0)))
        .map_err(log_error(log))
}

pub async fn get_leaderboard(state: web::Data<models::AppState>, web::Query(query): web::Query<models::LeaderboardQueryRequest>) -> Result<impl Responder, CustomError> {
    match query.validate() {
        Ok(_) => (),
        Err(_e) => return Err(CustomError::ValidationError)
    }

    let log = state.log.new(o!("handler" => "get_leaderboard")); 

    let client: Client = get_client(state.pool.clone(), log.clone()).await?;

    // Map text/string/bpchar/varchar to integer since integer-based operations/comparisons are generally much faster
    let mapped_difficulty = constants::DIFFICULTY_MAP.get::<str>(&query.difficulty.clone()).expect("error mapping difficulty string to integer");

    let result = db::get_score_entries(&client, query.max_entries, *mapped_difficulty).await;

    result.map(|scores| HttpResponse::Ok().header("Content-Security-Policy", "default-src 'self'").header("Strict-Transport-Security", "max-age=3600").header("X-XSS-Protection", "1; mode=block").json(scores))
        .map_err(log_error(log))
}

pub async fn get_materials(state: web::Data<models::AppState>) -> Result<impl Responder, CustomError> {
    let log = state.log.new(o!("handler" => "get_materials")); 

    let client: Client = get_client(state.pool.clone(), log.clone()).await?;

    let result = db::get_total_materials(&client).await;

    result.map(|materials| HttpResponse::Ok().header("Content-Security-Policy", "default-src 'self'").header("Strict-Transport-Security", "max-age=3600").header("X-XSS-Protection", "1; mode=block").json(materials))
        .map_err(log_error(log))
}

pub async fn vsauce_handler() -> Result<impl Responder, std::io::Error> {
    Ok(HttpResponse::Ok().header("Content-Security-Policy", "default-src 'self'").header("Strict-Transport-Security", "max-age=3600").header("X-XSS-Protection", "1; mode=block").body("This resource does not exist... Or does it? *VSauce music plays*"))
}

pub async fn fortune_cookie_handler() -> Result<impl Responder, std::io::Error> {
    let fortune: String;
    
    if cfg!(target_os = "windows") {
        fortune = "No fortune cookies on Windows. 😔".to_string();
    } else {
        // Need to run `sudo apt install fortune` first since this command is not built-in on Ubuntu
        let output = Command::new("/usr/games/fortune")
        .arg("-s")
        .output()
        .and_then(|r| match r.status.success() {
            true => Ok(r),
            false => Err(io::Error::new(io::ErrorKind::InvalidData, "some error caused the child process to not run properly"))
        });

        fortune = match output {
            Ok(_) => std::str::from_utf8(&output.unwrap().stdout).unwrap().to_string(),
            Err(_e) => "Unfortunately, some internal server error that has occurred prevents us from giving a fortune cookie to you. Apologies! 😔".to_string()
        }
    }

    Ok(HttpResponse::Ok().header("Content-Security-Policy", "default-src 'self'").header("Strict-Transport-Security", "max-age=3600").header("X-XSS-Protection", "1; mode=block").body(fortune))
}

pub async fn favicon_handler() -> Result<fs::NamedFile, std::io::Error> {
    // Development mode
    #[cfg(debug_assertions)]
    return Ok(fs::NamedFile::open("static/favicon.ico")?.set_status_code(StatusCode::OK));

    // Production mode
    #[cfg(not(debug_assertions))]
    return Ok(fs::NamedFile::open("/root/overdue_backend/static/favicon.ico")?.set_status_code(StatusCode::OK));
}

pub async fn default_handler() -> Result<fs::NamedFile, std::io::Error> {
    // Development mode
    #[cfg(debug_assertions)]
    return Ok(fs::NamedFile::open("static/detected_cheater.png")?.set_status_code(StatusCode::NOT_FOUND));

    // Production mode
    #[cfg(not(debug_assertions))]
    return Ok(fs::NamedFile::open("/root/overdue_backend/static/detected_cheater.png")?.set_status_code(StatusCode::NOT_FOUND));
}