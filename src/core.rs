use rocket::{Rocket, Build, Request, Responder, catch, catchers};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::{Database, Connection as RocketConnection};
use serde::Serialize;

pub type Connection = RocketConnection<Herald>;
pub type Response<T> = Result<HeraldResponseOk<T>, HeraldResponseErr>;

#[derive(Database)]
#[database("herald")]
pub struct Herald(sqlx::MySqlPool);

#[derive(Serialize, Debug)]
struct HeraldResponseOkData<T> {
    status: Status,
    message: String,
    data: T,
}

#[derive(Serialize, Debug)]
struct HeraldResponseErrData {
    status: Status,
    message: String,
}

#[derive(Responder, Debug)]
pub struct HeraldResponseOk<T>((Status, Json<HeraldResponseOkData<T>>));
impl<T> HeraldResponseOk::<T> {
    fn new(status: Status, data: T) -> HeraldResponseOk<T> {
        HeraldResponseOk::<T>((status, Json(HeraldResponseOkData::<T> {
            status: status,
            message: String::new(),
            data: data,
        })))
    }
}

impl<T> From<T> for HeraldResponseOk<T> {
    fn from(data: T) -> HeraldResponseOk<T> {
        HeraldResponseOk::<T>::new(Status::Ok, data)
    }
}

impl<T> From<(Status, T)> for HeraldResponseOk<T> {
    fn from((status, data): (Status, T)) -> HeraldResponseOk<T> {
        HeraldResponseOk::<T>::new(status, data)
    }
}

#[derive(Responder, Debug)]
pub struct HeraldResponseErr((Status, Json<HeraldResponseErrData>));
impl HeraldResponseErr {
    fn new(status: Status, message: String) -> HeraldResponseErr {
        HeraldResponseErr((status, Json(HeraldResponseErrData {
            status: status,
            message: message,
        })))
    }
}

impl From<sqlx::Error> for HeraldResponseErr {
    fn from(error: sqlx::Error) -> HeraldResponseErr {
        HeraldResponseErr::new(Status::InternalServerError, format!("SQL Backend Error: {}", error.to_string()))
    }
}

#[catch(default)]
pub fn default(status: Status, _req: &Request) -> HeraldResponseErr {
    HeraldResponseErr::new(status, status.reason_lossy().to_string())
}

pub fn build() -> Rocket<Build> {
    rocket::build()
        .attach(Herald::init())
        .register("/", catchers![default])
}
