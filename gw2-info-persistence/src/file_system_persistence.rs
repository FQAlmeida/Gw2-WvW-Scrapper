use std::{fs::File, io::Write};

use crate::persistence_system_interface::PersistenceSystem;

#[derive(Debug, Clone)]
pub struct FileSystemPersistence {
    basepath: String,
}

impl PersistenceSystem for FileSystemPersistence {
    fn save(
        &self,
        obj: &Vec<gw2_api_models::models::matchup_overview::MatchupOverview>,
    ) -> Result<(), std::io::Error> {
        for wvw_match in obj.iter() {
            let id = wvw_match.id();
            let start_time = wvw_match.start_time();

            // save to file named match_{id}_{start_time}.json
            let filename = Self::gen_filename(id, start_time);
            let mut fp = self.basepath.clone();
            fp.push('/');
            fp.push_str(&filename);

            Self::save_json(&fp, serde_json::to_vec_pretty(wvw_match).unwrap().as_ref())?;
        }
        Ok(())
    }
}

impl FileSystemPersistence {
    pub fn new(basepath: String) -> Self {
        Self { basepath }
    }
}

impl FileSystemPersistence {
    fn save_json(filepath: &String, content: &[u8]) -> Result<(), std::io::Error> {
        let fp = std::path::Path::new(filepath);
        let parent_dir = fp.parent();
        if let Some(pd) = parent_dir {
            if !pd.exists() {
                std::fs::create_dir_all(pd)?;
            }
            dbg!(std::fs::canonicalize(&pd)?);
        }
        dbg!(&fp);
        let mut fd = File::create(&fp)?;
        fd.write_all(content)?;
        Ok(())
    }

    fn gen_filename(id: &String, start_time: &String) -> String {
        let mut filename = String::from("data/match_");
        filename.push_str(id);
        filename.push('_');
        filename.push_str(start_time);
        filename.push_str(".json");
        return filename;
    }
}
