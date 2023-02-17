use chrono::NaiveDateTime;
use chrono::ParseError;
use chrono::TimeZone;
use chrono::Utc;
use pg_db_adapter::models::MatchupOverviewPG;
use pg_db_adapter::PostgresAdapter;
use rocket::State;
use rocket::get;
use rocket::http;
use rocket::http::Status;
use rocket::launch;
use rocket::request::FromParam;
use rocket::routes;
use rocket::serde::json::Json;
use std::env;

struct ServerState {
    adapter: PostgresAdapter,
}

pub struct NaiveDateForm(pub NaiveDateTime);

impl<'a> FromParam<'a> for NaiveDateForm {
    type Error = ParseError;

    fn from_param(param: &'a str) -> std::result::Result<Self, Self::Error> {
        match NaiveDateTime::parse_from_str(&param, "%Y-%m-%d-%H-%M-%S") {
            Ok(date) => Ok(NaiveDateForm(date)),
            Err(e) => Err(e),
        }
    }
}
#[get("/<_start_date>/<_end_date>")]
async fn index(
    _start_date: NaiveDateForm,
    _end_date: NaiveDateForm,
    server_state: &State<ServerState>
) -> Result<Json<Vec<MatchupOverviewPG>>, Status> {
    let adapter = server_state.adapter.clone();
    let get_conn = adapter.get_connection().await;

    let (client, conn) = match get_conn {
        Ok((client, conn)) => (client, conn),
        Err(_) => return Err(rocket::http::Status::InternalServerError),
    };

    // let (client, conn) = adapter.get_connection().await.unwrap();
    rocket::tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("connection error: {}", e);
        }
    });

    let start_date = Utc.from_utc_datetime(&_start_date.0);
    let end_date = Utc.from_utc_datetime(&_end_date.0);

    let result = client.select_by_date_range(&start_date, &end_date).await;

    return match result {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(http::Status::InternalServerError),
    };
}

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();

    let host: &str = &env::var("PG_HOST")
        .expect("PG_HOST must be set.")
        .to_owned();
    let user: &str = &env::var("PG_USER")
        .expect("PG_USER must be set.")
        .to_owned();
    let password: &str = &env::var("PG_PASSWORD")
        .expect("PG_PASSWORD must be set.")
        .to_owned();

    let adapter = PostgresAdapter::new(host, user, password);

    rocket::build()
        .manage(ServerState { adapter })
        .mount("/", routes![index])
}
