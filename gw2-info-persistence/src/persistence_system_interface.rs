use std::error::Error;

use async_trait::async_trait;
use gw2_api_models::models::matchup_overview::MatchupOverview;

#[async_trait]
pub trait PersistenceSystem {
   async fn save<'life>(&self, obj: &'life [MatchupOverview]) -> Result<(), Box<dyn Error>>;
}


