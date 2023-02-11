use std::error::Error;

use crate::persistence_system_interface::PersistenceSystem;
use async_trait::async_trait;
use redis_conn_adapter::{RedisAdapter, IntoConfig};

#[derive(Debug, Clone)]
pub struct RedisPersistence {
    redis_adapter: RedisAdapter,
}

#[async_trait]
impl PersistenceSystem for RedisPersistence {
    async fn save(
        &self,
        obj: &Vec<gw2_api_models::models::matchup_overview::MatchupOverview>,
    ) -> Result<(), Box<dyn Error>> {
        let client = self.redis_adapter.get_client().await?;
        // TODO: Define Redis Query to insert/update obj in key Gw2Matchups
        // TODO: Define if one query, or one per matchup info in obj
        // client.json_set();
        Ok(())
    }
}

impl RedisPersistence {
    pub async fn new(config: impl IntoConfig) -> Result<Self, redis_conn_adapter::Error> {
        Ok(Self { redis_adapter: RedisAdapter::new(config).await? })
    }
}
