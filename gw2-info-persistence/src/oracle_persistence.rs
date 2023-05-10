use std::error::Error;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use gw2_api_models::models::matchup_overview::MatchupOverview;
use oracle_db_adapter::OracleAdapter;

use crate::persistence_system_interface::PersistenceSystem;

#[derive(Debug, Clone)]
pub struct OraclePersistence {
    adapter: OracleAdapter,
}

impl OraclePersistence {
    pub fn new(host: &str, user: &str, password: &str) -> Self {
        Self {
            adapter: OracleAdapter::new(host, user, password),
        }
    }
}

#[async_trait]
impl PersistenceSystem for OraclePersistence {
    async fn save<'life>(&self, obj: &'life [MatchupOverview]) -> Result<(), Box<dyn Error>> {
        let client = self.adapter.get_connection().await?;

        for o in obj {
            client.insert(o).await?;
        }

        Ok(())
    }
    async fn select_by_date_range(
        &self,
        _start_date: &DateTime<Utc>,
        _end_date: &DateTime<Utc>,
    ) -> Result<Vec<MatchupOverview>, Box<dyn Error>> {
        todo!();
    }
}
