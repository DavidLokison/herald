use thiserror::Error;
use serde::Serialize;
use lettre::Message;
use lettre::message::header::ContentType;
use crate::template::Template;

#[derive(Error, Debug)]
pub enum MailError {
    #[error("render error")]
    RenderError(#[from] handlebars::RenderError),
    #[error("sqlx error")]
    SqlxError(#[from] sqlx::Error),
    #[error("address error")]
    AddressError(#[from] lettre::address::AddressError),
    #[error("message builder error")]
    BuilderError(#[from] lettre::error::Error),
    #[error("could not find mail templates for '{0}'")]
    TemplateMissingError(String),
}

pub fn render(template: &Template<'_>, contract: impl Serialize) -> Result<Message, MailError> {
    let handlebars = template.mail.as_ref().ok_or_else(|| MailError::TemplateMissingError(template.slug.clone()))?;
    let mut email = Message::builder().header(ContentType::TEXT_PLAIN);
    email = email.from(handlebars.render("sender", &contract)?.parse()?);
    email = email.to(handlebars.render("recipient", &contract)?.parse()?);
    if handlebars.has_template("reply_to") {
        email = email.reply_to(handlebars.render("reply_to", &contract)?.parse()?);
    }
    email = email.subject(handlebars.render("subject", &contract)?);
    Ok(email.body(handlebars.render("body", &contract)?)?)
}

#[cfg(test)]
mod tests {
    use std::assert_matches;
    use std::env;
    use std::pin::pin;
    use std::future::Future;
    use std::task::{
        Poll,
        Context,
        Waker,
    };
    use crate::template::Template;
    use super::render;
    use lettre::transport::AsyncTransport;
    use lettre::transport::stub::AsyncStubTransport;

    #[test]
    fn send_email_pipeline() {
        env::set_current_dir(format!("{}/tests", env!("CARGO_MANIFEST_DIR"))).unwrap();
        let template = Template::load("healthcheck").unwrap();
        let transport = AsyncStubTransport::new_ok();
        let email = render(&template, {}).unwrap();
        let future = transport.send(email);
        let mut future = pin!(future);
        let mut context = Context::from_waker(Waker::noop());
        let mut poll = Poll::Pending;
        while let Poll::Pending = poll {
            poll = future.as_mut().poll(&mut context);
        }
        let Poll::Ready(result) = poll else { unreachable!() };
        assert_matches!(result, Ok(_));
    }
}
