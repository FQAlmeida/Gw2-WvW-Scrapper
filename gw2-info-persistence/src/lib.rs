use std::{fs::File, io::Write};

use gw2_api_wrapper::models::matchup_overview::MatchupOverview;

pub fn save(basepath: &String, obj: Vec<MatchupOverview>) -> Result<(), std::io::Error> {
    for wvw_match in obj.iter() {
        let id = wvw_match.id();
        let start_time = wvw_match.start_time();

        // save to file named match_{id}_{start_time}.json
        let filename = gen_filename(id, start_time);
        let mut fp = basepath.clone(); 
        fp.push('/');
        fp.push_str(&filename);

        save_json(&fp, serde_json::to_vec_pretty(wvw_match).unwrap().as_ref())?;
    }
    Ok(())
}

fn save_json(filepath: &String, content: &[u8]) -> Result<(), std::io::Error> {
    let fp = std::path::Path::new(filepath);
    let parent_dir = fp.parent();
    if let Some(pd) = parent_dir {
        if !pd.exists(){
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

#[cfg(test)]
mod tests {
}
