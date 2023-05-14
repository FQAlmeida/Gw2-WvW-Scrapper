use std::env;

use chrono::{NaiveDateTime, ParseError, TimeZone, Utc};
use gw2_api_models::models::matchup_overview::MatchupOverview;
use gw2_info_persistence::{
    mongo_persistence::MongoPersistence, persistence_system_interface::PersistenceSystem,
};
use rocket::{get, http::Status, launch, request::FromParam, routes, serde::json::Json, State};
// use rocket_okapi::{openapi, openapi_get_routes};

struct ServerState {
    persistence: MongoPersistence,
}

pub struct NaiveDateForm(pub NaiveDateTime);

impl<'a> FromParam<'a> for NaiveDateForm {
    type Error = ParseError;

    fn from_param(param: &'a str) -> std::result::Result<Self, Self::Error> {
        match NaiveDateTime::parse_from_str(param, "%Y-%m-%d-%H-%M-%S") {
            Ok(date) => Ok(NaiveDateForm(date)),
            Err(e) => Err(e),
        }
    }
}

#[get("/<start_date>/<end_date>")]
async fn index(
    start_date: NaiveDateForm,
    end_date: NaiveDateForm,
    server_state: &State<ServerState>,
) -> Result<Json<Vec<MatchupOverview>>, Status> {
    let start_date = Utc.from_utc_datetime(&start_date.0);
    let end_date = Utc.from_utc_datetime(&end_date.0);

    let result_promise = server_state
        .persistence
        .select_by_date_range(&start_date, &end_date);
    let result = result_promise.await;

    match result {
        Ok(data) => Ok(Json(data.to_vec())),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();

    let host: &str = &env::var("MONGO_HOST")
        .expect("MONGO_HOST must be set.")
        .to_owned();
    let user: &str = &env::var("MONGO_USERNAME")
        .expect("MONGO_USERNAME must be set.")
        .to_owned();
    let password: &str = &env::var("MONGO_PASSWORD")
        .expect("MONGO_PASSWORD must be set.")
        .to_owned();

    let persistence = MongoPersistence::new(host, user, password).await;

    let routes = routes![index];

    rocket::build()
        .manage(ServerState { persistence })
        .mount("/", routes)
}
