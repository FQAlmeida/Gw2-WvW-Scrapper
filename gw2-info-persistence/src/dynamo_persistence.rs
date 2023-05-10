use std::error::Error;

use async_trait::async_trait;
use db_adapter::{db_adapter::DbAdapter, dynamo_adapter::DynamoAdapter};
use gw2_api_models::models::matchup_overview::MatchupOverview;


use crate::persistence_system_interface::PersistenceSystem;

#[derive(Debug, Clone)]
pub struct DynamoPersistence {
    adapter: DynamoAdapter,
}

impl DynamoPersistence {
    pub async fn new() -> Self {
        Self {
            adapter: DynamoAdapter::new().await,
        }
    }
}

#[async_trait]
impl PersistenceSystem for DynamoPersistence {
    async fn save<'life>(&self, obj: &'life [MatchupOverview]) -> Result<(), Box<dyn Error>> {
        let client = self.adapter.get_connection().await?;
        for o in obj {
            client.insert(o).await?;
        }

        Ok(())
    }
}
