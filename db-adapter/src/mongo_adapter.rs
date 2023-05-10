use std::error::Error;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use gw2_api_models::models::matchup_overview::MatchupOverview;
use mongodb::{
    bson,
    options::{ClientOptions, FindOptions, ServerApi, ServerApiVersion, UpdateOptions},
    Client,
};

use crate::db_adapter;

#[derive(Debug, Clone)]
pub struct MongoAdapterConfig {
    mongo_config: ClientOptions,
}

#[derive(Debug, Clone)]
pub struct MongoAdapter {
    config: MongoAdapterConfig,
}

impl MongoAdapter {
    pub async fn new(host: &str, user: &str, password: &str) -> Self {
        let conn_string = format!(
            "mongodb+srv://{}:{}@{}/?retryWrites=true&w=majority",
            user, password, host
        );

        let mut client_options = ClientOptions::parse(conn_string)
            .await
            .expect("Could not parse connection string");

        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        // Manually set an option.
        client_options.app_name = Some("Gw2 WvW Scrapper".to_string());

        // Get a handle to the deployment.
        let config = MongoAdapterConfig {
            mongo_config: client_options,
        };
        Self { config }
    }

    pub async fn get_connection(&self) -> Result<MongoClientAdapter, Box<dyn Error>> {
        let client = Client::with_options(self.config.mongo_config.clone())?;
        Ok(MongoClientAdapter::new(client))
    }
}

pub struct MongoClientAdapter {
    client: Client,
}

impl MongoClientAdapter {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl db_adapter::DbAdapter for MongoClientAdapter {
    async fn insert(&self, data: &MatchupOverview) -> Result<(), Box<dyn Error>> {
        self.client
            .database("gw2-wvw-scrapper")
            .collection::<MatchupOverview>("gw2-wvw-scrapper")
            .update_one(
                bson::doc! { // TODO (Otavio): bug, insert mult times same doc, comp is wrong
                  "id": data.id(),
                  "start_time" : format!("ISO_DATE(\"{}\")", data.start_time())
                },
                bson::doc! {
                    "$set": bson::to_bson(data).expect("Could not convert data to bson")
                },
                UpdateOptions::builder().upsert(true).build(),
            )
            .await?;
        Ok(())
    }

    async fn select_by_date_range(
        &self,
        start_date: &DateTime<Utc>,
        end_date: &DateTime<Utc>,
    ) -> Result<Vec<MatchupOverview>, Box<dyn Error>> {
        dbg!(&start_date);
        dbg!(&end_date);
        let filter = bson::doc! {
            "start_time": {// TODO (Otavio): bug, gte start date not returning data
                "$gte": format!("ISO_DATE(\"{}\")", start_date)
            },
            "end_time": {
                "$lte": format!("ISO_DATE(\"{}\")", end_date)
            }
        };
        let find_options = FindOptions::builder()
            .sort(bson::doc! { "id": 1, "start_time": 1, "_id": 1 })
            .build();
        let mut cursor = self
            .client
            .database("gw2-wvw-scrapper")
            .collection::<MatchupOverview>("gw2-wvw-scrapper")
            .find(filter, find_options)
            // .find(bson::doc! {}, None)
            .await
            .unwrap();

        let mut matchups: Vec<MatchupOverview> = vec![];
        // Iterate over the results of the cursor.
        while let Some(book) = cursor.try_next().await? {
            println!("title: {}", book.id());
            matchups.push(book);
        }
        return Ok(matchups);
    }
}

#[cfg(test)]
mod tests {}
