use rocket::{get, post, launch, routes};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use uuid::Uuid;

mod core;
use core::{Herald, Result, wrap};

use herald::data;
use herald::types::*;
use herald::types::request::*;
use herald::types::response::*;

expose_endpoint!(#[get("/health")] run_tests_health);
expose_endpoint!(#[get("/events/open")] get_open_events);
expose_endpoint!(#[get("/events/types")] get_event_types);

#[get("/events/types/<event_type_slug>/items")]
async fn get_bookable_items(mut db: Connection<Herald>, event_type_slug: &str) -> Result<Vec<Article>> {
    let event_type_slug = EventTypeSlug::try_query(&mut **db, event_type_slug).await?;
    let articles = herald::data::get_bookable_items(&mut db, &event_type_slug).await?;
    if articles.is_empty() {
        return Ok((Status::NoContent, articles).into());
    }
    Ok(articles.into())
}

#[get("/events/<event_id>/registrations/preview?<persons..>")]
async fn get_registration_preview(mut db: Connection<Herald>, event_id: Uuid, persons: Vec<PriceCheck>) -> Result<impl serde::Serialize> {
    let event_id = EventId::try_query(&mut **db, &event_id).await?;
    herald::data::get_registration_preview(&mut db, &event_id, &persons).await
        .map(Into::into)
        .map_err(Into::into)
}

#[post("/events/<event_id>/registrations", data = "<registration>")]
async fn create_registration(mut db: Connection<Herald>, event_id: Uuid, registration: Json<NewRegistration<'_>>) -> Result<()> {
    let event_id = EventId::try_query(&mut **db, &event_id).await?;
    data::create_registration(&mut db, &event_id, &registration).await
        .map(wrap(Status::Created))
        .map(Into::into)
        .map_err(Into::into)
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
