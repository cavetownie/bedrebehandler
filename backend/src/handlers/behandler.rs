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

pub async fn get_by_type_and_open(behandler_type: &str, db: sqlx::Pool<Sqlite>) -> Result<Vec<Behandler>, Error> {
    let query = "SELECT b.identifier, b.postnummer, b.kliniktype, b.navn, b.adresse, b.beskrivelse, b.opdateret
                 FROM behandler b JOIN aabningstider oh ON b.identifier = oh.behandler_id 
                 WHERE oh.day_of_week = strftime('%w', 'now', 'utc', '+4 hours') 
                 AND time('now', 'utc', '+4 hours') 
                 BETWEEN oh.open_time AND oh.close_time AND kliniktype = $1;";

    let query_res = sqlx::query_as::<_, Behandler>(query)
        .bind(decode(behandler_type).expect("UTF-8"))
        .fetch_all(&db)
        .await;

    match query_res {
        Ok(result) => Ok(result),
        Err(err) =>  Err(tide::Error::new(tide::StatusCode::InternalServerError, err))
    }
}