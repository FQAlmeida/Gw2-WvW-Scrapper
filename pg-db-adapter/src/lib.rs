use chrono::Utc;
use gw2_api_models::models::matchup_overview::MatchupOverview;
use models::MatchupOverviewPG;
use tokio_postgres::{tls::NoTlsStream, Config, NoTls, Socket, types::Json};

pub mod models;

#[derive(Debug, Clone)]
pub struct PostgresAdapter {
    config: tokio_postgres::Config,
}

impl PostgresAdapter {
    pub fn new(host: &str, user: &str, password: &str) -> Self {
        let mut config = Config::new();
        config.dbname("gw2_wvw_matchups");
        config.application_name("gw2-wvw-matchups");
        config.host(host);
        config.user(user);
        config.password(password);
        Self { config }
    }

    pub async fn get_connection(
        &self,
    ) -> Result<
        (
            PostgresClientAdapter,
            tokio_postgres::Connection<Socket, NoTlsStream>,
        ),
        tokio_postgres::Error,
    > {
        let (client, conn) = self.config.connect(NoTls).await?;
        Ok((PostgresClientAdapter { client }, conn))
    }
}

pub struct PostgresClientAdapter {
    client: tokio_postgres::Client,
}

impl PostgresClientAdapter {
    pub fn new(client: tokio_postgres::Client) -> Self {
        Self { client }
    }

    async fn match_exists_statement(
        &self,
    ) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
        self.client.prepare_typed("SELECT EXISTS(SELECT 1 FROM \"MatchupInfos\" WHERE \"MatchupInfos\".id_matchup = $1 AND \"MatchupInfos\".initial_date_matchup = $2);", &[tokio_postgres::types::Type::VARCHAR, tokio_postgres::types::Type::TIMESTAMPTZ]).await
    }

    async fn match_exists(&self, data: &MatchupOverview) -> Result<bool, tokio_postgres::Error> {
        let prepared = self.match_exists_statement().await?;
        let result = self
            .client
            .query_one(&prepared, &[data.id(), data.start_time()])
            .await?;
        let exists = result.get(0);
        Ok(exists)
    }

    async fn insert_prepared_statement(
        &self,
    ) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
        self.client.prepare_typed(
                "INSERT INTO \"MatchupInfos\" (id_matchup, initial_date_matchup, end_date_matchup, info) VALUES ($1, $2, $3, $4);", 
                &[tokio_postgres::types::Type::VARCHAR, tokio_postgres::types::Type::TIMESTAMPTZ, tokio_postgres::types::Type::TIMESTAMPTZ, tokio_postgres::types::Type::JSONB]
            ).await
    }

    pub async fn insert(&self, data: &MatchupOverview) -> Result<u64, tokio_postgres::Error> {
        if self.match_exists(data).await? {
            return self.update(data).await;
        }
        let statement = self.insert_prepared_statement().await?;
        self.client
            .execute(
                &statement,
                &[
                    data.id(),
                    data.start_time(),
                    data.end_time(),
                    &tokio_postgres::types::Json::<MatchupOverview>(data.clone()),
                ],
            )
            .await
    }

    async fn update_statement(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
        self.client.prepare_typed("UPDATE \"MatchupInfos\" SET info = $1 WHERE id_matchup = $2 AND initial_date_matchup = $3;", &[tokio_postgres::types::Type::JSONB, tokio_postgres::types::Type::VARCHAR, tokio_postgres::types::Type::TIMESTAMPTZ]).await
    }

    async fn update(&self, data: &MatchupOverview) -> Result<u64, tokio_postgres::Error> {
        let prepared = self.update_statement().await?;
        self.client
            .execute(
                &prepared,
                &[
                    &tokio_postgres::types::Json::<MatchupOverview>(data.clone()),
                    data.id(),
                    data.start_time(),
                ],
            )
            .await
    }

    async fn select_by_date_range_statement(
        &self,
    ) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
        self.client.prepare_typed("SELECT id_matchup, initial_date_matchup, end_date_matchup, info FROM \"MatchupInfos\" WHERE initial_date_matchup >= $1 AND end_date_matchup <= $2;", &[tokio_postgres::types::Type::TIMESTAMPTZ, tokio_postgres::types::Type::TIMESTAMPTZ]).await
    }

    pub async fn select_by_date_range(
        &self,
        initial_date: &chrono::DateTime<Utc>,
        end_date: &chrono::DateTime<Utc>,
    ) -> Result<Vec<MatchupOverviewPG>, tokio_postgres::Error> {
        let prepared = self.select_by_date_range_statement().await?;
        let rows = self.client.query(&prepared, &[initial_date, end_date]).await?;
        let result: Vec<MatchupOverviewPG> = rows.iter().map(|value|{
            let info: Json<MatchupOverview> = value.get(3);
            MatchupOverviewPG{
                matchup_id: value.get(0),
                initial_date_matchup: value.get(1),
                end_date_matchup: value.get(2),
                info: info.0,
            }
        }).collect();
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::PostgresAdapter;

    #[tokio::test]
    async fn can_connect() -> Result<(), tokio_postgres::Error> {
        let adapter = PostgresAdapter::new("192.168.0.11", "postgres", "<passwd_here>");
        let (_, conn) = adapter.get_connection().await?;

        tokio::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("connection error: {}", e);
            }
        });

        // let rows = client.query("SELECT $1::TEXT", &[&"hello world"]).await?;

        // And then check that we got back the same string we sent over.
        // let value: &str = rows[0].get(0);
        // assert_eq!(value, "hello world");

        Ok(())
    }
}
