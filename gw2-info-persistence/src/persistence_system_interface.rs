use std::error::Error;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use gw2_api_models::models::matchup_overview::MatchupOverview;

#[async_trait]
pub trait PersistenceSystem {
    async fn save<'life>(&self, obj: &'life [MatchupOverview]) -> Result<(), Box<dyn Error>>;
    async fn select_by_date_range(
        &self,
        start_date: &DateTime<Utc>,
        end_date: &DateTime<Utc>,
    ) -> Result<Vec<MatchupOverview>, Box<dyn Error>>;
}
