use rocket::{get, post, launch, routes};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use uuid::Uuid;

mod core;
use core::{Herald, Result};

use herald::types::request::*;

expose_endpoint!(#[get("/health")] run_tests_health);
expose_endpoint!(#[get("/events/open")] get_open_events);
expose_endpoint!(#[get("/events/types")] get_event_types);
expose_endpoint!(#[get("/events/types/<event_type_slug>/items")] get_bookable_items, event_type_slug: &str);


#[get("/events/<event_id>/registrations/preview?<persons..>")]
async fn get_registration_preview(mut db: Connection<Herald>, event_id: Uuid, persons: Vec<PriceCheck>) -> Result<impl serde::Serialize> {
    herald::data::get_registration_preview(&mut db, &event_id, &persons).await
        .map(Into::into)
        .map_err(Into::into)
}

expose_endpoint!(#[post("/events/<event_id>/registrations", format = "json", data = "<registration>")] create_registration, event_id: Uuid, registration: Json<NewRegistration<'_>>);

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
