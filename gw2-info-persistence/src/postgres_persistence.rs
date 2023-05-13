use std::error::Error;

use async_trait::async_trait;
use chrono::{Utc, DateTime};
use db_adapter::{postgres_adapter::PostgresAdapter, db_adapter::DbAdapter};
use gw2_api_models::models::matchup_overview::MatchupOverview;

use tokio;

use crate::persistence_system_interface::PersistenceSystem;

#[derive(Debug, Clone)]
pub struct PostgresPersistence {
    adapter: PostgresAdapter,
}

impl PostgresPersistence {
    pub fn new(host: &str, user: &str, password: &str) -> Self {
        Self {
            adapter: PostgresAdapter::new(host, user, password),
        }
    }
}

#[async_trait]
impl PersistenceSystem for PostgresPersistence {
    async fn save<'life>(&self, obj: &'life [MatchupOverview]) -> Result<(), Box<dyn Error>> {
        let (client, conn) = self.adapter.get_connection().await?;

        tokio::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("connection error: {}", e);
            }
        });
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
