use rocket::{Rocket, Build, Request, Responder, catch, catchers};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::{Database, Connection as RocketConnection};
use serde::Serialize;

use herald::Error;

pub type Connection = RocketConnection<Herald>;
pub type Result<T> = std::result::Result<T, HeraldResponseErr>;
pub type Response<T> = Result<HeraldResponseOk<T>>;

#[derive(Database)]
#[database("herald")]
pub struct Herald(sqlx::MySqlPool);

#[catch(default)]
pub fn default(status: Status, _req: &Request) -> HeraldResponseErr {
    status.into()
}

pub fn build() -> Rocket<Build> {
    rocket::build()
        .attach(Herald::init())
        .register("/", catchers![default])
}

#[macro_export]
macro_rules! expose_endpoint {
    ($(#[$meta:meta])* $name:ident -> $T:ty) => {
        $(#[$meta])*
        async fn $name(mut db: rocket_db_pools::Connection<crate::core::Herald>) -> Result<crate::core::HeraldResponseOk<$T>, crate::core::HeraldResponseErr> {
            herald::data::$name(&mut db).await.map(Into::into).map_err(Into::into)
        }
    };

    ($(#[$meta:meta])* $name:ident -> $T:ty, $($arg:ident : $A:ty),*) => {
        $(#[$meta])*
        async fn $name(mut db: rocket_db_pools::Connection<crate::core::Herald>, $($arg: $A),*) -> Result<crate::core::HeraldResponseOk<$T>, crate::core::HeraldResponseErr> {
            herald::data::$name(&mut db, $(&$arg),*).await.map(Into::into).map_err(Into::into)
        }
    };
}



#[derive(Responder, Debug)]
pub struct HeraldResponseOk<T>((Status, Json<HeraldResponseOkData<T>>));

#[derive(Responder, Debug)]
pub struct HeraldResponseErr((Status, Json<HeraldResponseErrData>));

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

// HeraldResponseOk implementations

impl<T> From<(Status, T)> for HeraldResponseOk<T> {
    #[inline]
    fn from((status, data): (Status, T)) -> Self {
        Self((status, Json((status, data).into())))
    }
}

impl<T> From<T> for HeraldResponseOk<T> {
    #[inline]
    fn from(data: T) -> Self {
        (Status::Ok, data).into()
    }
}

// HeraldResponseErr implementations

impl From<Status> for HeraldResponseErr {
    #[inline]
    fn from(status: Status) -> Self {
        Self((status, Json(status.into())))
    }
}

impl From<(Status, String)> for HeraldResponseErr {
    #[inline]
    fn from((status, message): (Status, String)) -> Self {
        Self((status, Json((status, message).into())))
    }
}

impl From<String> for HeraldResponseErr {
    #[inline]
    fn from(message: String) -> Self {
        (Status::InternalServerError, message).into()
    }
}

impl From<(Status, &str)> for HeraldResponseErr {
    #[inline]
    fn from((status, message): (Status, &str)) -> Self {
        (status, message.to_string()).into()
    }
}

impl From<Error> for HeraldResponseErr {
    #[inline]
    fn from(error: Error) -> Self {
        (
            match error {
                Error::NotFound(_) => Status::NotFound,
                Error::SqlxError(_) => Status::InternalServerError,
            },
            error.to_string(),
        ).into()
    }
}

// HeraldResponseOkData implementations

impl<T> From<(Status, T)> for HeraldResponseOkData<T> {
    #[inline]
    fn from((status, data): (Status, T)) -> Self {
        Self {
            status: status,
            message: status.reason_lossy().to_string(),
            data: data,
        }
    }
}

// HeraldResponseErrData implementations

impl From<Status> for HeraldResponseErrData {
    #[inline]
    fn from(status: Status) -> Self {
        Self {
            status: status,
            message: status.reason_lossy().to_string(),
        }
    }
}

impl From<(Status, String)> for HeraldResponseErrData {
    #[inline]
    fn from((status, message): (Status, String)) -> Self {
        Self {
            status: status,
            message: match status.reason() {
                None => message,
                Some(reason) => format!("{}: {}", reason, message),
            }
        }
    }
}
