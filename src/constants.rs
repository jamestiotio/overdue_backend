use lazy_static::lazy_static;
use std::collections::HashMap;
use regex::Regex;
use std::sync::atomic::AtomicUsize;
use slog::Level;

// Define non-confidential constants here
pub const MAX_SCORE: i32 = 999999;
pub const MIN_SCORE: i32 = 0;
pub const MAX_MATERIALS: i32 = 10;
pub const MIN_MATERIALS: i32 = 0;
pub const MAX_LEADERBOARD_LENGTH: u32 = 100;
pub const MIN_LEADERBOARD_LENGTH: u32 = 1;
pub const MAX_BONUS_VALUE: i32 = 10;
pub const MIN_BONUS_VALUE: i32 = 0;
pub const INCOMING_SCORE_PAYLOAD_LIMIT: usize = 1024;
pub const INCOMING_LEADERBOARD_PAYLOAD_LIMIT: usize = 0;
pub const INCOMING_MATERIALS_PAYLOAD_LIMIT: usize = 0;
pub const RATE_LIMIT_INTERVAL_DURATION: u64 = 60;
pub const RATE_LIMIT_MAX_REQUESTS: usize = 60;
pub const KEEP_ALIVE_DURATION: usize = 150;
pub const CORS_MAX_AGE_DURATION: usize = 150;
pub const GAME_CLIENT_URL_DOMAIN_ORIGIN: &str = "https://sutd-fablab-game.netlify.app";
pub const PUBLIC_FACING_GAME_CLIENT_URL: &str = "https://overdue.sutd.edu.sg";
pub const FRONT_DOMAIN: &str = "https://openhouse.sutd.edu.sg";
pub const SERVER_HOST_URL: &str = "sutdoverdue.dev";

lazy_static! {
    pub static ref NAME_REGEX: Regex = Regex::new(r"^[a-zA-Z]{3}$").expect("error creating the name regex");
    pub static ref DIFFICULTY_REGEX: Regex = Regex::new(r"^easy|normal|hard$").expect("error creating the difficulty regex");
    pub static ref GENDER_REGEX: Regex = Regex::new(r"^[mMfF]{1}$").expect("error creating the gender regex");
    pub static ref DIFFICULTY_MAP: HashMap<&'static str, i32> = [("easy", 0), ("normal", 1), ("hard", 2)].iter().cloned().collect();
    pub static ref FLIPPED_DIFFICULTY_MAP: HashMap<i32, &'static str> = [(0, "easy"), (1, "normal"), (2, "hard")].iter().cloned().collect();
    pub static ref LEVEL: AtomicUsize = AtomicUsize::new(Level::Info.as_usize());
}