use sqlx::{
    Executor,
    MySql,
};

pub fn query<'e, E>(e: E, status: u64) -> impl Future<Output = Result<Option<String>, sqlx::Error>>
where
    E: Executor<'e, Database = MySql> + 'e,
{
    sqlx::query_file_scalar!("sql/registration_statuses/template.sql", status).fetch_one(e)
}
