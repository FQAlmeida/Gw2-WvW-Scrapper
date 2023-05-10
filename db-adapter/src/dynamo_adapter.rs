use std::{error::Error, time::Duration};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use dynamodb::types::AttributeValue;
use gw2_api_models::models::matchup_overview::MatchupOverview;

use aws_config;
use aws_sdk_dynamodb as dynamodb;
use serde_json;

use crate::db_adapter;

#[derive(Debug, Clone)]
pub struct DynamoAdapterConfig {
    aws_config: aws_config::SdkConfig,
}

#[derive(Debug, Clone)]
pub struct DynamoAdapter {
    time_to_sleep: Duration,
    config: DynamoAdapterConfig,
}

impl DynamoAdapter {
    pub async fn new() -> Self {
        let aws_config = aws_config::load_from_env().await;

        let config = DynamoAdapterConfig { aws_config };
        Self {
            time_to_sleep: Duration::from_secs(10),
            config,
        }
    }

    pub async fn get_connection(&self) -> Result<DynamoClientAdapter, aws_sdk_dynamodb::Error> {
        let client = dynamodb::Client::new(&self.config.aws_config);
        Ok(DynamoClientAdapter {
            time_to_sleep: self.time_to_sleep,
            client,
        })
    }
}

pub struct DynamoClientAdapter {
    time_to_sleep: Duration,
    client: dynamodb::Client,
}

impl DynamoClientAdapter {
    pub fn new(time_to_sleep: Duration, client: dynamodb::Client) -> Self {
        Self {
            time_to_sleep,
            client,
        }
    }
}

#[async_trait]
impl db_adapter::DbAdapter for DynamoClientAdapter {
    async fn insert(&self, data: &MatchupOverview) -> Result<(), Box<dyn Error>> {
        let matchup_key = format!("{} {}", data.id(), data.start_time());
        let start_time = data.start_time().to_rfc3339();
        let end_time = data.end_time().to_rfc3339();
        let content =
            serde_json::to_string(data).expect("Can serialize MatchupOverview data to string");

        let matchup_key_value = AttributeValue::S(matchup_key);
        let start_time_value = AttributeValue::S(start_time);
        let end_time_value = AttributeValue::S(end_time);
        let content_value = AttributeValue::S(content);

        let _ = self
            .client
            .put_item()
            .table_name("gw2-wvw-scrapper")
            .item("matchup_key", matchup_key_value)
            .item("matchup_start_date", start_time_value)
            .item("matchup_end_date", end_time_value)
            .item("content", content_value)
            .send()
            .await?;
        // Sleep so the cota doesn't exceed
        tokio::time::sleep(self.time_to_sleep).await;
        Ok(())
    }

    async fn select_by_date_range(
        &self,
        start_date: &DateTime<Utc>,
        end_date: &DateTime<Utc>,
    ) -> Result<Vec<MatchupOverview>, Box<dyn Error>> {
        let start_time = start_date.to_rfc3339();
        let end_time = end_date.to_rfc3339();

        let start_time_value = AttributeValue::S(start_time);
        let end_time_value = AttributeValue::S(end_time);

        let query = self.client.query();
        let table = query.table_name("gw2-wvw-scrapper");
        let filter = table
            .filter_expression("matchup_start_date >= :date_1 AND matchup_end_date <= :date_2")
            .expression_attribute_values(":date_1", start_time_value)
            .expression_attribute_values(":date_2", end_time_value);

        let result = filter.send().await;
        dbg!(&start_date);
        match result {
            Ok(data) => {
                dbg!(data);
            }
            Err(err) => {
                dbg!(err);
            }
        }

        // if let Some(items) = result?.items {
        //     let matchups: Vec<MatchupOverview> = items
        //         .iter()
        //         .map(|data| {
        //             let content_value = data.get("content").expect("Could not retrieve content");
        //             let content = content_value
        //                 .as_s()
        //                 .expect("Could not convert content value to String");
        //             let matchup_data: MatchupOverview = serde_json::from_str(content)
        //                 .expect("Could not convert content String to Struct MatchupOverview");
        //             matchup_data
        //         })
        //         .collect();
        //     return Ok(matchups);
        // }
        return Ok(vec![]);
    }
}

#[cfg(test)]
mod tests {}
