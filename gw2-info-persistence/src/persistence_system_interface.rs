use gw2_api_models::models::matchup_overview::MatchupOverview;

pub trait PersistenceSystem {
    fn save(&self, obj: &Vec<MatchupOverview>) -> Result<(), std::io::Error>;
}


