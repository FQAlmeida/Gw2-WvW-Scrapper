use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use gw2_api_models::models::matchup_overview::MatchupOverview;
use mongodb::{
    bson::{self, oid::ObjectId},
    options::{ClientOptions, FindOptions, ServerApi, ServerApiVersion, UpdateOptions},
    Client,
};
use std::error::Error;

use crate::db_adapter;

use self::models::MatchupOverviewMongo;
pub mod models;

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

impl MongoClientAdapter {
    async fn check_exists(
        &self,
        data: &MatchupOverview,
    ) -> Result<Option<ObjectId>, Box<dyn Error>> {
        let filter = bson::doc! {
            "id": data.id(),
            "initial_date_matchup": bson::DateTime::from_chrono(*data.start_time())
        };
        let result = self
            .client
            .database("gw2-wvw-scrapper")
            .collection::<MatchupOverviewMongo>("gw2-wvw-scrapper")
            .find_one(filter, None)
            // .find(bson::doc! {}, None)
            .await?;
        match result {
            Some(matchup) => Ok(Some(matchup.inner_id)),
            None => Ok(None),
        }
    }

    async fn update(&self, data: &MatchupOverview, id: ObjectId) -> Result<(), Box<dyn Error>> {
        self.client
            .database("gw2-wvw-scrapper")
            .collection::<MatchupOverviewMongo>("gw2-wvw-scrapper")
            .update_one(
                bson::doc! {
                  "_id": id,
                },
                bson::doc! {
                    "$set": bson::doc!{
                        "info": bson::to_bson(data).expect("Could not convert data to bson")
                    }
                },
                UpdateOptions::builder().upsert(true).build(),
            )
            .await?;
        Ok(())
    }
}

#[async_trait]
impl db_adapter::DbAdapter for MongoClientAdapter {
    async fn insert(&self, data: &MatchupOverview) -> Result<(), Box<dyn Error>> {
        let existent_id = self.check_exists(data).await?;
        if let Some(id) = existent_id {
            dbg!(format!("Found {}", &id));
            return self.update(data, id).await;
        }
        dbg!(format!("Did not found, inserting"));
        self.client
            .database("gw2-wvw-scrapper")
            .collection::<MatchupOverviewMongo>("gw2-wvw-scrapper")
            .insert_one(
                MatchupOverviewMongo {
                    inner_id: ObjectId::new(),
                    id: data.id().clone(),
                    initial_date_matchup: bson::DateTime::from_chrono(*data.start_time()),
                    end_date_matchup: bson::DateTime::from_chrono(*data.end_time()),
                    info: data.clone(),
                },
                None,
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
            "initial_date_matchup": {
                "$gte": bson::DateTime::from_chrono(*start_date)
            },
            "end_date_matchup": {
                "$lte": bson::DateTime::from_chrono(*end_date)
            }
        };
        let find_options = FindOptions::builder()
            .sort(bson::doc! { "id": 1, "initial_date_matchup": 1, "_id": 1 })
            .build();
        let mut cursor = self
            .client
            .database("gw2-wvw-scrapper")
            .collection::<MatchupOverviewMongo>("gw2-wvw-scrapper")
            .find(filter, find_options)
            // .find(bson::doc! {}, None)
            .await?;

        let mut matchups: Vec<MatchupOverview> = vec![];
        // Iterate over the results of the cursor.
        while let Some(matchup) = cursor.try_next().await? {
            matchups.push(matchup.info);
        }
        return Ok(matchups);
    }
}

#[cfg(test)]
mod tests {}
