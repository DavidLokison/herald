mod common;

use uuid::Uuid;

use herald::types::contract::*;
use crate::common::SqlServerContainer;

#[tokio::test]
async fn contracts_do_serialize() -> Result<(), Box<dyn std::error::Error>> {
    let server = SqlServerContainer::spawn().await;
    server.create_club_information_stub().await;
    server.create_pseudo_registration().await;
    server.approve_single_registration().await;
    let pool = sqlx::MySqlPool::connect(&server.url().await).await?;
    let id: Uuid = sqlx::query_scalar!("SELECT registrations.registration_id AS `registration_id: _` FROM registrations").fetch_one(&pool).await?;
    let contract_order_confirmation = Contract::fetch("order_confirmation", &pool, id).await?;
    let contract_payment_information = Contract::fetch("payment_information", &pool, id).await?;
    let contract_payment_confirmation = Contract::fetch("payment_confirmation", &pool, id).await?;
    pool.close().await;
    assert!(rocket::serde::json::to_value(contract_order_confirmation).is_ok());
    assert!(rocket::serde::json::to_value(contract_payment_information).is_ok());
    assert!(rocket::serde::json::to_value(contract_payment_confirmation).is_ok());
    Ok(())
}

#[tokio::test]
async fn order_confirmation_contract() -> Result<(), Box<dyn std::error::Error>> {
    let server = SqlServerContainer::spawn().await;
    server.create_club_information_stub().await;
    server.create_pseudo_registration().await;
    let pool = sqlx::MySqlPool::connect(&server.url().await).await?;
    let id: Uuid = sqlx::query_scalar!("SELECT registrations.registration_id AS `registration_id: _` FROM registrations").fetch_one(&pool).await?;
    let contract = Contract::fetch("order_confirmation", &pool, id).await?;
    pool.close().await;
    let json = rocket::serde::json::to_value(contract)?;
    let expected = rocket::serde::json::to_value(OrderConfirmation {
        event_title: String::from("Test Event"),
        recipient: Contact {
            name: String::from("Mister\tTesto\tTestoson"),
            email: String::from("testo@example.com"),
        },
        club: ClubInformation {
            short: String::from("Test Team"),
            long: String::from("Team Integration Testing"),
            email: String::from("team@example.com"),
        },
        autoapproved: false,
    })?;
    assert_eq!(json, expected);
    Ok(())
}

#[tokio::test]
async fn payment_information_valid() -> Result<(), Box<dyn std::error::Error>> {
    let server = SqlServerContainer::spawn().await;
    server.create_club_information_stub().await;
    server.create_pseudo_registration().await;
    server.approve_single_registration().await;
    let pool = sqlx::MySqlPool::connect(&server.url().await).await?;
    let id: Uuid = sqlx::query_scalar!("SELECT registrations.registration_id AS `registration_id: _` FROM registrations").fetch_one(&pool).await?;
    let contract = Contract::fetch("payment_information", &pool, id).await?;
    pool.close().await;
    let Contract::PaymentInformation(info) = contract else { panic!() };
    let total: i32 = info.articles.iter().map(|p| p.price.value * (p.count as i32)).sum();
    assert_eq!(total, info.total.value);
    Ok(())
}

#[tokio::test]
async fn payment_confirmation_valid() -> Result<(), Box<dyn std::error::Error>> {
    let server = SqlServerContainer::spawn().await;
    server.create_club_information_stub().await;
    server.create_pseudo_registration().await;
    server.approve_single_registration().await;
    let pool = sqlx::MySqlPool::connect(&server.url().await).await?;
    let id: Uuid = sqlx::query_scalar!("SELECT registrations.registration_id AS `registration_id: _` FROM registrations").fetch_one(&pool).await?;
    let contract = Contract::fetch("payment_confirmation", &pool, id).await?;
    pool.close().await;
    let Contract::PaymentConfirmation(_) = contract else { panic!() };
    Ok(())
}
