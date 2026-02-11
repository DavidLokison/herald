use rocket::{Rocket, Build, Request, Responder, catch, catchers};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::{Database, Connection as RocketConnection};
use serde::Serialize;

pub type Connection = RocketConnection<Herald>;

#[derive(Database)]
#[database("herald")]
pub struct Herald(sqlx::MySqlPool);

#[derive(Serialize, Debug)]
struct HeraldResponseData<T> {
    status: Status,
    message: String,
    data: Option<T>,
}

#[derive(Responder, Debug)]
pub struct HeraldResponse<T>((Status, Json<HeraldResponseData<T>>));
impl<T> HeraldResponse::<T> {
    pub fn ok(status: Status, data: T) -> HeraldResponse<T> {
        Self::new(status, HeraldResponseData::<T> {
            status: status,
            message: String::new(),
            data: Some(data),
        })
    }

    pub fn err(status: Status, message: String) -> HeraldResponse<T> {
        Self::new(status, HeraldResponseData::<T> {
            status: status,
            message: message,
            data: None,
        })
    }

    fn new(status: Status, data: HeraldResponseData<T>) -> HeraldResponse<T> {
        HeraldResponse::<T>((status, Json(data)))
    }
}

#[catch(default)]
pub fn default(status: Status, _req: &Request) -> HeraldResponse<u8> {
    HeraldResponse::err(status, status.reason_lossy().to_string())
}

pub fn build() -> Rocket<Build> {
    rocket::build()
        .attach(Herald::init())
        .register("/", catchers![default])
}
