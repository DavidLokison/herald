use std::hash::Hash;

use rocket::{Rocket, Build, Request, response::Responder, catch, catchers};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Database;
use serde::Serialize;

pub type Result<T> = std::result::Result<Envelope<<T as IntoEnvelope>::Responder>, Envelope<<herald::Error as IntoEnvelope>::Responder>>;

#[derive(Database)]
#[database("herald")]
pub struct Herald(sqlx::MySqlPool);

#[catch(404)]
pub fn endpoint_not_found(status: Status, req: &Request) -> (Status, Json<Error>) {
    (status, Json(Error { error: "ENDPOINT_NOT_FOUND", message: format!("request '{}' does not match an endpoint", req.uri()) }))
}

#[catch(default)]
pub fn default(status: Status, _req: &Request) -> (Status, Json<Error>) {
    (status, Json(Error { error: "UNKNOWN_ERROR", message: String::from("an unknown error occured") }))
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
        .register("/", catchers![endpoint_not_found, default])
}

#[macro_export]
macro_rules! expose_endpoint {
    ($(#[$meta:meta])* $name:ident $(,$arg:ident : $A:ty)*) => {
        $(#[$meta])*
        async fn $name(mut db: rocket_db_pools::Connection<crate::core::Herald> $(,$arg: $A)*) -> crate::core::Result<rocket::serde::json::Json<impl serde::Serialize>> {
            let ret = herald::data::$name(&mut db $(,&$arg)*).await?;
            crate::core::Envelope::ok(ret)
        }
    };
}

#[derive(Responder)]
pub struct Envelope<R>(R);

pub trait IntoEnvelope {
    type Responder;
    fn into_envelope(self) -> Envelope<Self::Responder>;
}

impl<T, R> From<T> for Envelope<R> where T: IntoEnvelope<Responder = R> {
    #[inline]
    fn from(t: T) -> Self {
        t.into_envelope()
    }
}

impl IntoEnvelope for herald::Error {
    type Responder = (Status, Json<Error>);

    fn into_envelope(self) -> Envelope<Self::Responder> {
        Envelope((
            match self {
                herald::Error::NotFound(_) => Status::NotFound,
                herald::Error::SqlxError(_) => Status::InternalServerError,
            },
            Json(Error {
                error: match self {
                    herald::Error::NotFound(_) => "RESOURCE_NOT_FOUND",
                    herald::Error::SqlxError(_) => "BACKEND_FAILURE",
                },
                message: self.to_string()
            }),
        ))
    }
}

impl<T> IntoEnvelope for Json<T> where T: Serialize {
    type Responder = Json<Success<T>>;

    fn into_envelope(self) -> Envelope<Self::Responder> {
        Envelope(Json(Success { data: self.0 }))
    }
}

impl<T> IntoEnvelope for Json<Created<T>> where T: Serialize + Hash {
    type Responder = Created<T>;

    fn into_envelope(self) -> Envelope<Self::Responder> {
        Envelope(self.0)
    }
}

pub struct Created<T: Serialize + Hash> {
    location: String,
    data: T,
}

#[derive(Serialize, Debug)]
pub struct Success<T: Serialize> {
    data: T,
}

#[derive(Serialize, Debug)]
pub struct Error {
    error: &'static str,
    message: String,
}

// Created implementations

impl<'r, 'o: 'r, T: Serialize + Hash> Responder<'r, 'o> for Created<T> {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'o> {
        let mut response = rocket::response::Response::build();
        let created = rocket::response::status::Created::new(self.location).tagged_body(Json(&self.data));
        response.merge(created.respond_to(req)?);
        response.merge((Status::Created, Json(Success { data: self.data })).respond_to(req)?);
        response.ok()
    }
}

impl<R> Envelope<R> {
    pub fn ok<T, E>(data: T) -> std::result::Result<Envelope<R>, E> where Json<T>: IntoEnvelope<Responder = R> {
        Ok(Json(data).into_envelope())
    }

    pub fn created<T: Serialize + Hash, S: Into<String>, E>(location: S, data: T) -> std::result::Result<Envelope<R>, E> where Json<Created<T>>: IntoEnvelope<Responder = R> {
       Ok(Json(Created {
           location: location.into(),
           data: data,
       }).into_envelope())
    }
}
