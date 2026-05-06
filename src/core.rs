use std::hash::Hash;

use rocket::{Rocket, Build, Request, response::Responder, catch, catchers};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Database;
use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Database)]
#[database("herald")]
pub struct Herald(sqlx::MySqlPool);

#[catch(default)]
pub fn default(status: Status, _req: &Request) -> Error {
    status.into()
}

pub fn build() -> Rocket<Build> {
    let _ = dotenv::from_path(std::env::current_dir().unwrap().join(".env"));
    rocket::custom(rocket::Config::figment()
        .merge(("databases.herald", rocket_db_pools::Config {
            url: std::env::var("DATABASE_URL").expect("DATABASE_URL should be specified by the environment or .env file"),
            min_connections: None,
            max_connections: 1024,
            connect_timeout: 3,
            idle_timeout: None,
            ..Default::default()
        })))
        .attach(Herald::init())
        .register("/", catchers![default])
}

#[macro_export]
macro_rules! expose_endpoint {
    ($(#[$meta:meta])* $name:ident $(,$arg:ident : $A:ty)*) => {
        $(#[$meta])*
        async fn $name(mut db: rocket_db_pools::Connection<crate::core::Herald> $(,$arg: $A)*) -> std::result::Result<crate::core::Custom<impl serde::Serialize>, crate::core::Error> {
            let ret = herald::data::$name(&mut db $(,&$arg)*).await?;
            crate::core::Custom::ok(ret)
        }
    };
}

pub struct Created<T: Serialize + Hash> {
    location: String,
    data: T,
}

#[derive(Serialize, Debug)]
pub struct Custom<T: Serialize> {
    status: Status,
    reason: &'static str,
    data: T,
}

#[derive(Serialize, Debug)]
pub struct Error {
    status: Status,
    reason: &'static str,
    error: String,
}

// Created implementations

impl<T: Serialize + Hash> Created<T> {
    fn new<S: Into<String>>(location: S, data: T) -> Self {
        Self {
            location: location.into(),
            data: data,
        }
    }

    pub fn ok<S: Into<String>>(location: S, data: T) -> Result<Self> {
        Ok(Self::new(location, data))
    }
}

impl<'r, 'o: 'r, T: Serialize + Hash> Responder<'r, 'o> for Created<T> {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'o> {
        let mut response = rocket::response::Response::build();
        let created = rocket::response::status::Created::new(self.location).tagged_body(Json(&self.data));
        response.merge(created.respond_to(req)?);
        response.merge(Custom::new(Status::Created, self.data).respond_to(req)?);
        response.ok()
    }
}

// Custom implementations

impl<T: Serialize> Custom<T> {
    fn new(status: Status, data: T) -> Self {
        Self {
            status: status,
            reason: status.reason_lossy(),
            data: data,
        }
    }

    pub fn ok(data: T) -> Result<Self> {
        Ok(Self::new(Status::Ok, data))
    }
}

impl<'r, 'o: 'r, T: Serialize> Responder<'r, 'o> for Custom<T> {
    #[inline]
    fn respond_to(self, r: &'r Request<'_>) -> rocket::response::Result<'o> {
        (self.status, Json(self)).respond_to(r)
    }
}

// Error implementations

impl Error {
    fn new<S: Into<String>>(status: Status, error: S) -> Self {
        Self {
            status: status,
            reason: status.reason_lossy(),
            error: error.into(),
        }
    }
}

impl From<Status> for Error {
    #[inline]
    fn from(status: Status) -> Self {
        Self::new(status, "")
    }
}

impl From<String> for Error {
    #[inline]
    fn from(error: String) -> Self {
        Self::new(Status::InternalServerError, error)
    }
}

impl From<herald::Error> for Error {
    fn from(error: herald::Error) -> Self {
        Self::new(
            match error {
                herald::Error::NotFound(_) => Status::NotFound,
                herald::Error::SqlxError(_) => Status::InternalServerError,
            },
            error.to_string(),
        )
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    #[inline]
    fn respond_to(self, r: &'r Request<'_>) -> rocket::response::Result<'o> {
        (self.status, Json(self)).respond_to(r)
    }
}
