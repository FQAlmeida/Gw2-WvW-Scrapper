use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Score {
    red: u64,
    blue: u64,
    green: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KillScore {
    red: u64,
    blue: u64,
    green: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct World {
    red: u64,
    blue: u64,
    green: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Team {
    red: Vec<u64>,
    blue: Vec<u64>,
    green: Vec<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MapScore {
    r#type: String,
    scores: Score,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Skirmish {
    id: u64,
    scores: Score,
    map_scores: Vec<MapScore>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Objective {
    id: String,
    r#type: String,
    owner: String,
    last_flipped: String, // ISO-8601,
    claimed_by: Option<String>,
    claimed_at: Option<String>, // ISO-8601
    points_tick: u64,
    points_capture: u64,
    guild_upgrades: Option<Vec<u64>>,
    yaks_delivered: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MapBonus {
    r#type: String,
    owner: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MapInfo {
    id: u64,
    r#type: String,
    scores: Score,
    bonuses: Vec<MapBonus>,
    objectives: Vec<Objective>,
    deaths: KillScore,
    kills: KillScore,
}

use getset::Getters;

#[derive(Getters, Serialize, Deserialize, Debug)]
#[getset(get = "pub")]
pub struct MatchupOverview {
    id: String,
    start_time: String, // ISO-8601
    end_time: String,   // ISO-8601
    scores: Score,
    worlds: World,
    all_worlds: Team,
    deaths: KillScore,
    kills: KillScore,
    victory_points: Score,
    skirmishes: Vec<Skirmish>,
    maps: Vec<MapInfo>,
}
