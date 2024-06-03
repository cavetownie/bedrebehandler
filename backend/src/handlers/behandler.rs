use super::*;

pub async fn list(db: sqlx::Pool<Sqlite>) -> Result<Vec<Behandler>, Error> {
    let query_res = sqlx::query_as::<_, Behandler>("SELECT * FROM behandler")
        .fetch_all(&db)
        .await;

    match query_res {
        Ok(result) => Ok(result),
        Err(err) =>  Err(tide::Error::new(tide::StatusCode::InternalServerError, err))
    }
}

pub async fn get_by_type(behandler_type: &str, db: sqlx::Pool<Sqlite>) -> Result<Vec<Behandler>, Error> {
    let query_res = sqlx::query_as::<_, Behandler>("SELECT * FROM behandler WHERE kliniktype = $1")
        .bind(decode(behandler_type).expect("UTF-8"))
        .fetch_all(&db)
        .await;

    match query_res {
        Ok(result) => Ok(result),
        Err(err) =>  Err(tide::Error::new(tide::StatusCode::InternalServerError, err))
    }
}