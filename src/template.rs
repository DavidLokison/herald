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

    fn files(slug: &str, prefix: &str, directories: Vec<&str>) -> HashMap<String, String> {
        let mut ret = HashMap::new();
        for directory in directories {
            ret.insert(directory.to_owned(), format!("./templates/{prefix}/{directory}/{slug}.hbs"));
        }
        ret
    }

    fn check_exists(files: impl IntoIterator<Item: AsRef<std::path::Path>>) -> std::io::Result<bool> {
        files.into_iter().map(fs::exists).fold(Ok(true), |a, b| Ok(a? & b?))
    }

    fn create_handlebars(files: HashMap<String, String>) -> Result<Option<Handlebars<'a>>, TemplateError> {
        if Self::check_exists(files.values())? {
            let mut handlebars = Handlebars::new();
            for (key, file) in files {
                handlebars.register_template_file(&key, file)?;
            }
            Ok(Some(handlebars))
        } else {
            Ok(None)
        }
    }

    pub fn load(slug: String) -> Result<Self, TemplateError> {
        Ok(Self {
            mail: Self::create_handlebars(Self::files(&slug, "mail", vec!["subject", "body", "sender", "recipient"]))?,
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
