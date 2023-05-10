use gw2_api_wrapper::Gw2ApiWrapper;
use gw2_info_persistence::{
    dynamo_persistence::DynamoPersistence, persistence_system_interface::PersistenceSystem,
};
use std::{env, error::Error};
use tokio_cron_scheduler::{Job, JobScheduler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let scheduler = JobScheduler::new().await.unwrap();

    // let default_basepath = String::from(".");
    // let basepath = args.get(1).unwrap_or(&default_basepath).clone();
    // let file_persistence = FileSystemPersistence::new(basepath);

    // let aws_access_key_id: &str = &env::var("aws_access_key_id")
    //     .expect("aws_access_key_id must be set.")
    //     .to_owned();
    // let aws_secret_access_key: &str = &env::var("aws_secret_access_key")
    //     .expect("aws_secret_access_key must be set.")
    //     .to_owned();
    // let region: &str = &env::var("region").expect("region must be set.").to_owned();

    // let oracle_persistence = OraclePersistence::new(host, user, password);
    let dynamo_persistence = DynamoPersistence::new().await;

    let job = Job::new_async("0 1/2 * * * *", move |_, _| {
        // let this_file_persistence = file_persistence.clone();
        // let this_pg_persistence = pg_persistence.clone();
        // let this_oracle_persistence = oracle_persistence.clone();

        let this_dynamo_persistence = dynamo_persistence.clone();

        Box::pin(async move {
            dbg!("Running Job");
            let api = Gw2ApiWrapper::create();
            let ids = api.get_matchup_ids().await.unwrap();
            dbg!(&ids);
            let info = api.get_matchup_info(ids).await.unwrap();

            // dbg!(&info);
            // this_file_persistence.save(&info).await.unwrap();
            // this_pg_persistence.save(&info).await.unwrap();
            this_dynamo_persistence.save(&info).await.unwrap();

            dbg!("Saved");
        })
    })
    .unwrap();

    scheduler.add(job).await.unwrap();

    // TODO(Otavio): Wrap in tokio thread, aka: tokio::spawn
    scheduler.start().await.unwrap();

    // Wait a while so that the jobs actually run
    dbg!("All done, sleeping...");
    tokio::time::sleep(core::time::Duration::from_secs(7 * 24 * 60 * 60)).await;
    dbg!("Sleep done");

    Ok(())
}
