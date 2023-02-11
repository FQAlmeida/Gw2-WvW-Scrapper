use gw2_api_wrapper::Gw2ApiWrapper;
use gw2_info_persistence::{
    file_system_persistence::FileSystemPersistence, persistence_system_interface::PersistenceSystem,
};
use std::env;
use tokio_cron_scheduler::{Job, JobScheduler};

#[tokio::main]
async fn main() {
    let scheduler = JobScheduler::new().await.unwrap();

    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let default_basepath = String::from(".");
    let basepath = args.get(1).unwrap_or(&default_basepath).clone();
    let file_persistence = FileSystemPersistence::new(basepath);

    let job = Job::new_async("0 1/15 * * * *", move |_, _| {
        let this_file_persistence = file_persistence.clone();
        Box::pin(async move {
            dbg!("Running Job");
            let api = Gw2ApiWrapper::create();
            let ids = api.get_matchup_ids().await.unwrap();
            dbg!(&ids);
            let info = api.get_matchup_info(ids).await.unwrap();
            // dbg!(&info);
            this_file_persistence.save(&info).await.unwrap();
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
}
