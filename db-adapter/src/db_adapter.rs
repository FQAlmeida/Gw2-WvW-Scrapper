use async_trait::async_trait;
use chrono::{DateTime, Utc};
use gw2_api_models::models::matchup_overview::MatchupOverview;
use std::error::Error;

#[async_trait]
pub trait DbAdapter {
    async fn insert(&self, obj: &MatchupOverview) -> Result<(), Box<dyn Error>>;
    async fn select_by_date_range(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<MatchupOverview>, Box<dyn Error>>;
}
