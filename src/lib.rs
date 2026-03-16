pub mod types;
pub mod data;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NotFound(String),
    SqlxError(sqlx::Error),
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::NotFound(s) => write!(f, "Resource Not Found: {}", s),
            Self::SqlxError(_) => write!(f, "SQL Backend Error"),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::SqlxError(value)
    }
}
