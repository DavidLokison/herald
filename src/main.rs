#[macro_use] extern crate rocket;
use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("herald")]
struct Herald(sqlx::MySqlPool);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Herald::init())
        .mount("/", routes![index])
}
