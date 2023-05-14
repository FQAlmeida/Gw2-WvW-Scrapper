use gw2_api_models::models::matchup_overview::MatchupOverview;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchupOverviewMongo {
    #[serde(rename = "_id")]
    pub inner_id: ObjectId,
    pub id: String,
    pub initial_date_matchup: DateTime,
    pub end_date_matchup: DateTime,
    pub info: MatchupOverview,
}
