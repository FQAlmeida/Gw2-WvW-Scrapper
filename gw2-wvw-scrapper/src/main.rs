use gw2_api_wrapper::Gw2ApiWrapper;
use gw2_info_persistence::{
    file_system_persistence::FileSystemPersistence, persistence_system_interface::PersistenceSystem, postgres_persistence::PostgresPersistence,
};
use std::{env, error::Error};
use tokio_cron_scheduler::{Job, JobScheduler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    
    let scheduler = JobScheduler::new().await.unwrap();

    let default_basepath = String::from(".");
    let basepath = args.get(1).unwrap_or(&default_basepath).clone();
    let file_persistence = FileSystemPersistence::new(basepath);
    
    let host: &str = &env::var("PG_HOST").expect("PG_HOST must be set.").to_owned();
    let user: &str = &env::var("PG_USER").expect("PG_USER must be set.").to_owned();
    let password: &str = &env::var("PG_PASSWORD").expect("PG_PASSWORD must be set.").to_owned();
    let pg_persistence = PostgresPersistence::new(host, user, password);

    let job = Job::new_async("0 1/1 * * * *", move |_, _| {
        let this_file_persistence = file_persistence.clone();
        let this_pg_persistence = pg_persistence.clone();
        Box::pin(async move {
            dbg!("Running Job");
            let api = Gw2ApiWrapper::create();
            let ids = api.get_matchup_ids().await.unwrap();
            dbg!(&ids);
            let info = api.get_matchup_info(ids).await.unwrap();
            // dbg!(&info);
            this_file_persistence.save(&info).await.unwrap();
            this_pg_persistence.save(&info).await.unwrap();

            dbg!("Saved");
        })
    })
    .unwrap();

    scheduler.add(job).await.unwrap();
    scheduler.start().await.unwrap();

    // Wait a while so that the jobs actually run
    dbg!("All done, sleeping...");
    tokio::time::sleep(core::time::Duration::from_secs(7 * 24 * 60 * 60)).await;
    dbg!("Sleep done");
    Ok(())
}
