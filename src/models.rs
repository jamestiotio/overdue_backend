use deadpool_postgres::Pool;
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use slog::Logger;
use tokio_pg_mapper_derive::PostgresMapper;
use validator::Validate;

use crate::{constants, defaults};

#[derive(Debug, Validate, Serialize, Deserialize, Clone, PostgresMapper)]
#[pg_mapper(table = "material")]
pub struct MaterialValueEntry {
    pub name: String,
    pub value: i32,
}

#[derive(Debug, Validate, Serialize, Deserialize, PostgresMapper, ToSql, FromSql, Clone)]
#[pg_mapper(table = "material")]
pub struct MaterialEntry {
    pub name: String,
    #[validate(range(min = "constants::MIN_MATERIALS", max = "constants::MAX_MATERIALS"))]
    pub quantity: i32,
}

#[derive(Debug, Validate, Serialize, Deserialize, PostgresMapper, ToSql, FromSql)]
#[pg_mapper(table = "leaderboard")]
pub struct ScoreEntry {
    #[validate(length(equal = 3), regex = "constants::NAME_REGEX")]
    pub name: String,
    #[validate(regex = "constants::GENDER_REGEX")]
    pub gender: String,
    // Only emails that satisfy the HTML5 regex specs standard will be accepted (some esoteric
    // valid emails will not be accepted).
    #[validate(email)]
    pub email: String,
    #[validate(regex = "constants::DIFFICULTY_REGEX")]
    pub difficulty: String,
    #[validate(range(min = "constants::MIN_SCORE", max = "constants::MAX_SCORE"))]
    pub score: i32,
    #[validate(range(min = "constants::MIN_BONUS_VALUE", max = "constants::MAX_BONUS_VALUE"))]
    #[serde(default = "defaults::default_bonus")]
    pub bonus: i32,
    pub materials: Vec<MaterialEntry>,
}

#[derive(Debug, Validate, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct LeaderboardQueryRequest {
    #[validate(range(
        min = "constants::MIN_LEADERBOARD_LENGTH",
        max = "constants::MAX_LEADERBOARD_LENGTH"
    ))]
    #[serde(default = "defaults::default_max_entries")]
    pub max_entries: i64,
    #[validate(regex = "constants::DIFFICULTY_REGEX")]
    #[serde(default = "defaults::default_difficulty")]
    pub difficulty: String,
}

#[derive(Debug, Validate, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "leaderboard")]
pub struct LeaderboardSingleEntry {
    #[validate(length(equal = 3), regex = "constants::NAME_REGEX")]
    pub name: String,
    #[validate(length(equal = 1), regex = "constants::GENDER_REGEX")]
    pub gender: String,
    #[validate(regex = "constants::DIFFICULTY_REGEX")]
    pub difficulty: String,
    #[validate(range(min = "constants::MIN_SCORE", max = "constants::MAX_SCORE"))]
    pub score: i32,
    #[validate(range(min = 1))]
    pub rank: i64,
}

#[derive(Debug, Validate, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "leaderboard")]
pub struct LeaderboardMultipleEntries {
    #[validate(length(equal = 3), regex = "constants::NAME_REGEX")]
    pub name: String,
    #[validate(length(equal = 1), regex = "constants::GENDER_REGEX")]
    pub gender: String,
    #[validate(range(min = "constants::MIN_SCORE", max = "constants::MAX_SCORE"))]
    pub score: i32,
    #[validate(range(min = 1))]
    pub rank: i64,
}

// Model for error message.
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub error: String,
    pub message: String,
}

// Model for logging.
#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
    pub log: Logger,
}
