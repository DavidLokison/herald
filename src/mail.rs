use thiserror::Error;
use crate::template::Template;
use crate::types::contract::Contract;
use sqlx::{
    Executor,
    MySql,
};
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum MailError {
    #[error("render error")]
    RenderError(#[from] handlebars::RenderError),
    #[error("sqlx error")]
    SqlxError(#[from] sqlx::Error),
    #[error("could not find mail templates for '{0}'")]
    TemplateMissingError(String),
}

pub async fn send_mail<'e, E>(e: E, template: Template<'_>, id: Uuid) -> Result<(), MailError>
where
    E: Executor<'e, Database = MySql> + 'e,
{
    let handlebars = template.mail.ok_or_else(|| MailError::TemplateMissingError(template.slug.clone()))?;
    let contract = Contract::fetch(&template.slug, e, id).await?.into_inner();
    let sender = handlebars.render("sender", &contract)?;
    let recipient = handlebars.render("recipient", &contract)?;
    let subject = handlebars.render("subject", &contract)?;
    let body = handlebars.render("body", &contract)?;
    Ok(())
}
