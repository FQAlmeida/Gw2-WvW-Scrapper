use rustis::{
    bb8::{Pool, PooledConnection},
    client::PooledClientManager,
};

pub use rustis::{bb8::RunError, client::IntoConfig, Error};
pub use rustis::commands::{JsonCommands, SearchCommands};

#[derive(Debug, Clone)]
pub struct RedisAdapter {
    conn_pool: rustis::bb8::Pool<rustis::client::PooledClientManager>,
}

impl RedisAdapter {
    pub async fn new(config: impl IntoConfig) -> Result<Self, rustis::Error> {
        let manager = PooledClientManager::new(config)?;
        Ok(Self {
            conn_pool: Pool::builder().max_size(4).build(manager).await?,
        })
    }
}

impl RedisAdapter {
    pub async fn get_client(
        &self,
    ) -> Result<PooledConnection<PooledClientManager>, RunError<rustis::Error>> {
        Ok(self.conn_pool.get().await?)
    }
}
