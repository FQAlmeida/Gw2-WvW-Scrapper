use models::matchup_overview::MatchupOverview;
use reqwest::{Client, ClientBuilder};

pub mod models;

pub struct Gw2ApiWrapper {
    client: Client,
}

impl Gw2ApiWrapper {
    fn build_client() -> Client {
        ClientBuilder::new().build().unwrap()
    }
}

impl Gw2ApiWrapper {
    pub fn create() -> Self {
        Self {
            client: Self::build_client(),
        }
    }
    pub async fn get_matchup_ids(&self) -> Result<Vec<String>, reqwest::Error> {
        let response = self
            .client
            .get("https://api.guildwars2.com/v2/wvw/matches")
            .send()
            .await?;
        let data: Vec<String> = response.json().await?;
        return Ok(data);
    }

    pub async fn get_matchup_info(
        &self,
        ids: Vec<String>,
    ) -> Result<Vec<MatchupOverview>, reqwest::Error> {
        let mut uri = "https://api.guildwars2.com/v2/wvw/matches?ids=".to_owned();
        uri.push_str(&ids.join(","));
        let response = self.client.get(uri).send().await?;
        let data: Vec<MatchupOverview> = response.json().await?;
        return Ok(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn can_get_list_of_matchups() {
        let api = Gw2ApiWrapper::create();
        let matchup_ids: Vec<String> = api.get_matchup_ids().await.unwrap();
        dbg!(&matchup_ids);
        assert_eq!(matchup_ids.len(), 9);
    }

    #[tokio::test]
    async fn can_get_a_matchup_overview() {
        let api = Gw2ApiWrapper::create();
        let matchup_ids: Vec<String> = vec!["1-1".to_string()];
        let matchup_overview = api.get_matchup_info(matchup_ids).await.unwrap();
        assert_eq!(matchup_overview[0].id(), "1-1");
    }

    #[tokio::test]
    async fn can_get_all_matches_info(){
        let api = Gw2ApiWrapper::create();
        let ids = api.get_matchup_ids().await.unwrap();
        let qtd_matches =ids.len();
        let matchup_overview = api.get_matchup_info(ids).await.unwrap();
        assert_eq!(matchup_overview.len(), qtd_matches)
    }
}
