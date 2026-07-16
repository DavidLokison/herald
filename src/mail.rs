use thiserror::Error;
use serde::Serialize;
use lettre::Message;
use lettre::message::Mailbox;
use lettre::message::header::ContentType;
use lettre::transport::AsyncTransport;
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
    #[error("smtp server error")]
    SmtpError(#[from] lettre::transport::smtp::Error),
    #[error("could not find mail templates for '{0}'")]
    TemplateMissingError(String),
    #[cfg(test)]
    #[error("stub server error")]
    StubError(#[from] lettre::transport::stub::Error),
}

pub async fn send_mail<T>(t: T, template: Template<'_>, contract: impl Serialize) -> Result<(), MailError>
where
    T: AsyncTransport + Sync,
    MailError: From<<T as AsyncTransport>::Error>,
{
    let handlebars = template.mail.ok_or_else(|| MailError::TemplateMissingError(template.slug.clone()))?;
    let mut email = Message::builder().header(ContentType::TEXT_PLAIN);
    email = email.from(handlebars.render("sender", &contract)?.parse::<Mailbox>()?);
    email = email.to(handlebars.render("recipient", &contract)?.parse::<Mailbox>()?);
    if handlebars.has_template("reply_to") {
        email = email.reply_to(handlebars.render("reply_to", &contract)?.parse::<Mailbox>()?);
    }
    email = email.subject(handlebars.render("subject", &contract)?);
    let email = email.body(handlebars.render("body", &contract)?)?;
    t.send(email).await.map(|_r| ()).map_err(MailError::from)
}

#[cfg(test)]
mod tests {
    use std::assert_matches;
    use std::env;
    use std::pin::pin;
    use std::task::{
        Poll,
        Context,
        Waker,
    };
    use crate::template::Template;
    use super::send_mail;
    use lettre::transport::stub::AsyncStubTransport;

    #[test]
    fn send_email_pipeline() {
        env::set_current_dir(format!("{}/tests", env!("CARGO_MANIFEST_DIR"))).unwrap();
        let template = Template::load("healthcheck".to_owned()).unwrap();
        let transport = AsyncStubTransport::new_ok();
        let future = send_mail(transport, template, {});
        let mut future = pin!(future);
        let mut context = Context::from_waker(Waker::noop());
        let mut poll = Poll::Pending;
        while let Poll::Pending = poll {
            poll = future.as_mut().poll(&mut context);
        }
        let Poll::Ready(result) = poll else { unreachable!() };
        assert_matches!(result, Ok(()));
    }
}
