use gw2_api_wrapper::Gw2ApiWrapper;
use tokio_cron_scheduler::{Job, JobScheduler};
use std::env;

#[tokio::main]
async fn main() {
    let scheduler = JobScheduler::new().await.unwrap();

    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let default_basepath = String::from(".");
    let basepath = args.get(1).unwrap_or(&default_basepath).clone();

    let job = Job::new_async("0 1/1 * * * *", move |_, _| {
        let this_basepath = basepath.clone();
        Box::pin(async move {
            dbg!("Running Job");
            let api = Gw2ApiWrapper::create();
            let ids = api.get_matchup_ids().await.unwrap();
            dbg!(&ids);
            let info = api.get_matchup_info(ids).await.unwrap();
            // dbg!(&info);
            gw2_info_persistence::save(&this_basepath, info).unwrap();
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
