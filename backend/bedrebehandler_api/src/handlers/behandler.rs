use super::*;

pub async fn list(db_pool: sqlx::Pool<Sqlite>) -> Result<Vec<Behandler>, Error> {
    let query_res = sqlx::query_as::<_, Behandler>("SELECT * from behandler")
        .fetch_all(&db_pool)
        .await;

    match query_res {
        Ok(result) => Ok(result),
        Err(err) =>  Err(tide::Error::new(tide::StatusCode::InternalServerError, err))
    }
}