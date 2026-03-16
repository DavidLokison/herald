use rocket::{get, post, launch, routes};
use rocket::serde::json::Json;
use uuid::Uuid;

mod core;

use herald::types::request::*;

expose_endpoint!(#[get("/health")] run_tests_health);
expose_endpoint!(#[get("/events/open")] get_open_events);
expose_endpoint!(#[get("/events/types")] get_event_types);
expose_endpoint!(#[get("/events/types/<event_type_slug>/items")] get_bookable_items, event_type_slug: &str);
expose_endpoint!(#[post("/events/<event_id>/persons_price_check", format = "json", data = "<persons>")] check_persons_price, event_id: Uuid, persons: Json<Vec<PriceCheck>>);
expose_endpoint!(#[post("/events/<event_id>/registrations", format = "json", data = "<registration>")] create_registration, event_id: Uuid, registration: Json<NewRegistration<'_>>);

#[launch]
fn rocket() -> _ {
    core::build()
        .mount("/", routes![
            run_tests_health,
            get_open_events,
            get_event_types,
            get_bookable_items,
            check_persons_price,
            create_registration,
        ])
}
