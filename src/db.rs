use actix_web::web;
use deadpool_postgres::Client;
use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::types::{Json, Type};

use crate::{
    constants,
    errors::CustomError,
    models::{LeaderboardMultipleEntries, LeaderboardSingleEntry, MaterialEntry, ScoreEntry},
};

pub async fn add_score_entry(
    client: &Client,
    item: web::Json<ScoreEntry>,
) -> Result<Vec<LeaderboardSingleEntry>, CustomError> {
    // Map text/string/bpchar/varchar to integer since integer-based
    // operations/comparisons are generally much faster.
    let mapped_difficulty = constants::DIFFICULTY_MAP
        .get::<str>(&item.difficulty.clone())
        .expect("error mapping difficulty string to integer");

    let lowercased_gender: String = item.gender.clone().to_ascii_lowercase();

    let statement = client
        .prepare_typed(
            "INSERT INTO leaderboard (name, gender, email, difficulty, score, materials) VALUES \
             ($1, $2, $3, $4, $5, $6) RETURNING id, name, gender, difficulty, score",
            &[
                Type::BPCHAR,
                Type::BPCHAR,
                Type::TEXT,
                Type::INT4,
                Type::INT4,
                Type::JSONB,
            ],
        )
        .await
        .map_err(|_err| CustomError::DbError)?;

    // Setting the inputs as parameters for the query statement this way (SQL query
    // parameterization) prevents SQL injection.
    let score = client
        .query(
            &statement,
            &[
                &item.name.clone(),
                &lowercased_gender.clone(),
                &item.email.clone(),
                &mapped_difficulty.clone(),
                &item.score.clone(),
                &Json(&item.materials.clone()),
            ],
        )
        .await
        .map_err(|_err| CustomError::DbError)?;

    let id: i32 = score[0].get("id");

    let rank_statement = client
        .prepare(
            "SELECT subquery.rank FROM (SELECT id, dense_rank() OVER (PARTITION BY difficulty \
             ORDER BY score DESC) rank FROM leaderboard) subquery WHERE subquery.id = $1",
        )
        .await
        .map_err(|_err| CustomError::DbError)?;

    let rank = client
        .query(&rank_statement, &[&id.clone()])
        .await
        .map_err(|_err| CustomError::DbError)?;

    let remapped_difficulty: String = constants::FLIPPED_DIFFICULTY_MAP
        .get::<i32>(&score[0].get::<_, i32>("difficulty"))
        .expect("error mapping difficulty integer to string")
        .to_string();

    let return_value = vec![LeaderboardSingleEntry {
        name: score[0].get("name"),
        gender: score[0].get("gender"),
        difficulty: remapped_difficulty,
        score: score[0].get("score"),
        rank: rank[0].get("rank"),
    }];

    Ok(return_value)
}

pub async fn get_score_entries(
    client: &Client,
    limit: i64,
    difficulty: i32,
) -> Result<Vec<LeaderboardMultipleEntries>, CustomError> {
    // If multiple score entries have the same rank due to same score, order them by
    // the time they are added to the database (while still retaining the same
    // rank).
    let statement = client
        .prepare(
            "SELECT name, gender, score, dense_rank() OVER (PARTITION BY difficulty ORDER BY \
             score DESC) rank FROM leaderboard WHERE difficulty = $1 ORDER BY rank ASC, id ASC \
             FETCH FIRST $2 ROWS ONLY",
        )
        .await
        .map_err(|_err| CustomError::DbError)?;

    // Setting `limit` and `difficulty` as parameters for the query statement this
    // way (SQL query parameterization) prevents SQL injection.
    let scores = client
        .query(&statement, &[&difficulty, &limit])
        .await
        .map_err(|_err| CustomError::DbError)?
        .iter()
        .map(|row| {
            LeaderboardMultipleEntries::from_row_ref(row)
                .expect("error mapping leaderboard score entries")
        })
        .collect::<Vec<LeaderboardMultipleEntries>>();

    Ok(scores)
}

pub async fn get_total_materials(client: &Client) -> Result<Vec<MaterialEntry>, CustomError> {
    let statement = client
        .prepare("SELECT name, quantity FROM material ORDER BY id ASC")
        .await
        .map_err(|_err| CustomError::DbError)?;

    let materials = client
        .query(&statement, &[])
        .await
        .map_err(|_err| CustomError::DbError)?
        .iter()
        .map(|row| MaterialEntry::from_row_ref(row).expect("error mapping material entries"))
        .collect::<Vec<MaterialEntry>>();

    Ok(materials)
}
