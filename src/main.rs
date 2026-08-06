use rocket::{get, post, launch, routes};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use uuid::Uuid;
use time::Date;

mod core;
use core::{Herald, Result, Created, Envelope};

use herald::data;
use herald::types::*;
use herald::types::request::*;
use herald::types::response::*;

expose_endpoint!(#[get("/health")] run_tests_health);
expose_endpoint!(#[get("/events/open")] get_open_events);
expose_endpoint!(#[get("/events/types")] get_event_types);

#[get("/events/types/<event_type_slug>/items")]
async fn get_bookable_items(mut db: Connection<Herald>, event_type_slug: &str) -> Result<Json<Vec<Article>>> {
    let event_type_slug = EventTypeSlug::try_query(&mut **db, event_type_slug).await?;
    let articles = herald::data::get_bookable_items(&mut db, &event_type_slug).await?;
    Envelope::ok(articles)
}

#[get("/events/<event_id>/registrations/preview?<birthdays>")]
async fn get_registration_preview(mut db: Connection<Herald>, event_id: Uuid, birthdays: Vec<Date>) -> Result<Json<Vec<Article>>> {
    let event_id = EventId::try_query(&mut **db, &event_id).await?;
    let preview = herald::data::get_registration_preview(&mut db, &event_id, &birthdays).await?;
    Envelope::ok(preview)
}

#[post("/events/<event_id>/registrations", data = "<registration>")]
async fn create_registration(mut db: Connection<Herald>, event_id: Uuid, registration: Json<NewRegistration<'_>>) -> Result<Json<Created<()>>> {
    let event_id = EventId::try_query(&mut **db, &event_id).await?;
    let _ = data::create_registration(&mut db, &event_id, &registration).await?;
    Envelope::created("", ()) // TODO
}

#[launch]
fn rocket() -> _ {
    core::build()
        .mount("/", routes![
            run_tests_health,
            get_open_events,
            get_event_types,
            get_bookable_items,
            get_registration_preview,
            create_registration,
        ])
}
