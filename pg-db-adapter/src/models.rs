use chrono::{DateTime, Utc};
use gw2_api_models::models::matchup_overview::MatchupOverview;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchupOverviewPG {
    pub matchup_id: String,
    pub initial_date_matchup: DateTime<Utc>,
    pub end_date_matchup: DateTime<Utc>,
    pub info: MatchupOverview,
}
