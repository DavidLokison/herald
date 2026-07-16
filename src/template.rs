use thiserror::Error;
use std::fs;
use std::collections::HashMap;
use handlebars::{
    Handlebars,
};
use sqlx::{
    Executor,
    MySql,
};

#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("sqlx error")]
    SqlxError(#[from] sqlx::Error),
    #[error("handlebars template error")]
    HandlebarsTemplateError(#[from] handlebars::TemplateError),
}

pub struct Template<'a> {
    pub slug: String,
    pub mail: Option<Handlebars<'a>>,
}

impl<'a> Template<'a> {
    pub async fn prepare<'e, E>(e: E, status: u64) -> Result<Self, TemplateError>
    where
        E: Executor<'e, Database = MySql> + 'e,
    {
        let slug = sqlx::query_file_scalar!("sql/registration_statuses/template.sql", status).fetch_one(e).await?.expect("status should yield a valid template slug");
        Self::load(slug)
    }

    fn create_handlebars(slug: &str, prefix: &str, directories: Vec<&str>, options: impl Fn(&mut Handlebars<'_>)) -> Result<Option<Handlebars<'a>>, TemplateError> {
        let files = {
            let mut ret = HashMap::new();
            for directory in directories {
                ret.insert(directory.to_owned(), format!("./templates/{prefix}/{directory}/{slug}.hbs"));
            }
            ret
        };
        if files.values().into_iter().map(fs::exists).fold(Ok(true) as std::io::Result<bool>, |a, b| Ok(a? & b?))? {
            let mut handlebars = Handlebars::new();
            for (key, file) in files {
                handlebars.register_template_file(&key, file)?;
            }
            options(&mut handlebars);
            Ok(Some(handlebars))
        } else {
            Ok(None)
        }
    }

    pub fn load(slug: String) -> Result<Self, TemplateError> {
        Ok(Self {
            mail: Self::create_handlebars(&slug, "mail", vec!["subject", "body", "sender", "recipient"], |handlebars| {
                handlebars.set_strict_mode(true);
                handlebars.register_escape_fn(handlebars::no_escape);
            })?,
            slug: slug,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::assert_matches;
    use super::Template;

    #[test]
    fn template_load() {
        env::set_current_dir("./tests").unwrap();
        let template = Template::load("order_confirmation".to_owned()).unwrap();
        assert_matches!(template.mail, Some(_));
    }

    #[test]
    fn template_load_none_if_missing() {
        env::set_current_dir("./tests").unwrap();
        let template = Template::load("missing_slug".to_owned()).unwrap();
        assert_matches!(template.mail, None);
    }
}
