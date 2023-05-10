use chrono::Utc;
use getset::Getters;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Score {
    red: u64,
    blue: u64,
    green: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct KillScore {
    red: u64,
    blue: u64,
    green: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct World {
    red: u64,
    blue: u64,
    green: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Team {
    red: Vec<u64>,
    blue: Vec<u64>,
    green: Vec<u64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MapScore {
    r#type: String,
    scores: Score,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Skirmish {
    id: u64,
    scores: Score,
    map_scores: Vec<MapScore>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MapBonus {
    r#type: String,
    owner: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MapInfo {
    id: u64,
    r#type: String,
    scores: Score,
    bonuses: Vec<MapBonus>,
    objectives: Vec<Objective>,
    deaths: KillScore,
    kills: KillScore,
}

#[serde_as]
#[derive(Getters, Serialize, Deserialize, Debug, Clone)]
#[getset(get = "pub")]
pub struct MatchupOverview {
    id: String,
    #[serde(with = "my_date_format")]
    start_time: chrono::DateTime<Utc>, // ISO-8601
    #[serde(with = "my_date_format")]
    end_time: chrono::DateTime<Utc>, // ISO-8601
    scores: Score,
    worlds: World,
    all_worlds: Team,
    deaths: KillScore,
    kills: KillScore,
    victory_points: Score,
    skirmishes: Vec<Skirmish>,
    maps: Vec<MapInfo>,
}

mod my_date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%SZ";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

pub mod mock {
    use chrono::Utc;

    use super::{KillScore, MatchupOverview, Score, Team, World};

    pub fn get_naive_mock() -> MatchupOverview {
        MatchupOverview {
            id: String::from("1-1"),
            start_time: Utc::now(),
            end_time: Utc::now(),
            scores: Score {
                red: 0,
                blue: 0,
                green: 0,
            },
            worlds: World {
                red: 0,
                blue: 0,
                green: 0,
            },
            all_worlds: Team {
                red: vec![0],
                blue: vec![0],
                green: vec![0],
            },
            deaths: KillScore {
                red: 0,
                blue: 0,
                green: 0,
            },
            kills: KillScore {
                red: 0,
                blue: 0,
                green: 0,
            },
            victory_points: Score {
                red: 0,
                blue: 0,
                green: 0,
            },
            skirmishes: vec![],
            maps: vec![],
        }
    }
}
