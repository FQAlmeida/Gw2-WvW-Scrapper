use std::error::Error;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use db_adapter::{db_adapter::DbAdapter, mongo_adapter::MongoAdapter};
use gw2_api_models::models::matchup_overview::MatchupOverview;

use crate::persistence_system_interface::PersistenceSystem;

#[derive(Debug, Clone)]
pub struct MongoPersistence {
    adapter: MongoAdapter,
}

impl MongoPersistence {
    pub async fn new(host: &str, user: &str, password: &str) -> Self {
        Self {
            adapter: MongoAdapter::new(host, user, password).await,
        }
    }
}

#[async_trait]
impl PersistenceSystem for MongoPersistence {
    async fn save<'life>(&self, obj: &'life [MatchupOverview]) -> Result<(), Box<dyn Error>> {
        let client = self.adapter.get_connection().await?;
        for o in obj {
            client.insert(o).await?;
        }

        Ok(())
    }

    async fn select_by_date_range(
        &self,
        start_date: &DateTime<Utc>,
        end_date: &DateTime<Utc>,
    ) -> Result<Vec<MatchupOverview>, Box<dyn Error>> {
        let client = self.adapter.get_connection().await?;
        let result = client.select_by_date_range(start_date, end_date).await?;

        Ok(result)
    }
}
