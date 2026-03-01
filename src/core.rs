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
    fn new(status: Status, data: T) -> Self {
        Self((status, Json(HeraldResponseOkData::<T> {
            status: status,
            message: status.reason_lossy().to_string(),
            data: data,
        })))
    }
}

impl<T> From<T> for HeraldResponseOk<T> {
    #[inline]
    fn from(data: T) -> Self {
        Self::new(Status::Ok, data)
    }
}

impl<T> From<(Status, T)> for HeraldResponseOk<T> {
    #[inline]
    fn from((status, data): (Status, T)) -> Self {
        Self::new(status, data)
    }
}

#[derive(Responder, Debug)]
pub struct HeraldResponseErr((Status, Json<HeraldResponseErrData>));
impl HeraldResponseErr {
    fn new(status: Status, message: String) -> Self {
        Self((status, Json(HeraldResponseErrData {
            status: status,
            message: message,
        })))
    }
}

impl From<Status> for HeraldResponseErr {
    #[inline]
    fn from(status: Status) -> Self {
        Self::new(status, status.reason_lossy().to_string())
    }
}

impl From<(Status, String)> for HeraldResponseErr {
    #[inline]
    fn from((status, message): (Status, String)) -> Self {
        Self::new(status, match status.reason() {
            None => message,
            Some(reason) => format!("{}: {}", reason, message),
        })
    }
}

impl From<String> for HeraldResponseErr {
    #[inline]
    fn from(message: String) -> Self {
        Self::from((Status::InternalServerError, message))
    }
}

impl From<(Status, &str)> for HeraldResponseErr {
    #[inline]
    fn from((status, message): (Status, &str)) -> Self {
        Self::from((status, message.to_string()))
    }
}

impl From<sqlx::Error> for HeraldResponseErr {
    #[inline]
    fn from(error: sqlx::Error) -> Self {
        Self::new(Status::InternalServerError, format!("SQL Backend Error: {}", error.to_string()))
    }
}

#[catch(default)]
pub fn default(status: Status, _req: &Request) -> HeraldResponseErr {
    status.into()
}

pub fn build() -> Rocket<Build> {
    rocket::build()
        .attach(Herald::init())
        .register("/", catchers![default])
}
