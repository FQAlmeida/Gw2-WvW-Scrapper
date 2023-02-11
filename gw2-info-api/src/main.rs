use chrono::NaiveDate;
use chrono::ParseError;

use rocket::get;
use rocket::launch;
use rocket::request::FromParam;
use rocket::routes;

pub struct NaiveDateForm(pub NaiveDate);

impl<'a> FromParam<'a> for NaiveDateForm {
    type Error = ParseError;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match NaiveDate::parse_from_str(&param, "%Y-%m-%d") {
            Ok(date) => Ok(NaiveDateForm(date)),
            Err(e) => Err(e),
        }
    }
}

#[get("/<_start_date>/<_end_date>")]
fn index(_start_date: NaiveDateForm, _end_date: NaiveDateForm) -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}