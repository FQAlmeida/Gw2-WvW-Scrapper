use std::error::Error;

use async_trait::async_trait;
use gw2_api_models::models::matchup_overview::MatchupOverview;
use pg_db_adapter::PostgresAdapter;

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
    async fn save(&self, obj: &Vec<MatchupOverview>) -> Result<(), Box<dyn Error>> {
        let (client, conn) = self.adapter.get_connection().await?;

        tokio::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("connection error: {}", e);
            }
        });
        for o in obj {
            client.insert(&o).await?;
        }

        Ok(())
    }
}
