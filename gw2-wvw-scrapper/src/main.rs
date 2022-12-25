use gw2_api_wrapper::Gw2ApiWrapper;
use tokio_cron_scheduler::{Job, JobScheduler};

#[tokio::main]
async fn main() {
    let scheduler = JobScheduler::new().await.unwrap();

    // let job = Job::new_async("0 0 2,18 * * Friday,Saturday", |_, _| {
    let job = Job::new_async("* 1/15 * * * *", |_, _| {
        Box::pin(async move{
            dbg!("Running Job");
            let api = Gw2ApiWrapper::create();
            let ids = api.get_matchup_ids().await.unwrap();
            dbg!(&ids);
            let info = api.get_matchup_info(ids).await.unwrap();
            dbg!(&info);
            gw2_info_persistence::save(info).unwrap();
            dbg!("Saved");
        })
    })
    .unwrap();

    scheduler.add(job).await.unwrap();
    scheduler.start().await.unwrap();

    // Wait a while so that the jobs actually run
    tokio::time::sleep(core::time::Duration::from_secs(7 * 24 * 60 * 60)).await;
    dbg!("Sleep done");
}
