use super::*;

#[derive(Clone, Serialize, Deserialize, FromRow, Debug)]
pub struct Behandler {
    identifier: i64,
    postnummer: i64,
    kliniktype: String,
    navn: String,
    adresse: String,
    beskrivelse: Option<String>,
    opdateret: chrono::NaiveDateTime,
}

#[derive(Clone, Serialize, Deserialize, FromRow, Debug)]
pub struct Aabningstider {
    identifier: i64,
    behandler_id: i64,
    day_of_week: i64,
    open_time: chrono::NaiveTime,
    close_time: chrono::NaiveTime,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(default)]
pub struct BehandlerQueryParamters {
    postnummer: u16,
    åben: bool,
}
impl Default for BehandlerQueryParamters {
    fn default() -> Self {
        Self {
            postnummer: 0,
            åben: false,
        }
    }
}

pub async fn list(db: sqlx::Pool<Sqlite>) -> Result<Vec<Behandler>, Error> {
    let query_res = sqlx::query_as::<_, Behandler>("SELECT * FROM behandler")
        .fetch_all(&db)
        .await;

    match query_res {
        Ok(result) => Ok(result),
        Err(err) =>  Err(tide::Error::new(tide::StatusCode::InternalServerError, err))
    }
}

pub async fn get(behandler_id: &str, db: sqlx::Pool<Sqlite>) -> Result<Vec<Behandler>, Error> {
    let query_res = sqlx::query_as::<_, Behandler>("SELECT * FROM behandler WHERE identifier = $1")
        .bind(behandler_id)
        .fetch_all(&db)
        .await;

    match query_res {
        Ok(result) => Ok(result),
        Err(err) =>  Err(tide::Error::new(tide::StatusCode::InternalServerError, err))
    }
}

pub async fn get_opening_hours(behandler_id: &str, db: sqlx::Pool<Sqlite>) -> Result<Vec<Aabningstider>, Error> {
    let query_res = sqlx::query_as::<_, Aabningstider>("SELECT * FROM behandler b join aabningstider oh on b.identifier = oh.behandler_id WHERE b.identifier = $1")
        .bind(behandler_id)
        .fetch_all(&db)
        .await;

    match query_res {
        Ok(result) => Ok(result),
        Err(err) =>  Err(tide::Error::new(tide::StatusCode::InternalServerError, err))
    }
}

pub async fn get_by_type(behandler_type: &str, behandler: BehandlerQueryParamters, db: sqlx::Pool<Sqlite>) -> Result<Vec<Behandler>, Error> {
    if behandler.åben == false {
        let query_res = sqlx::query_as::<_, Behandler>("SELECT * FROM behandler WHERE kliniktype = $1 AND ($2 = 0 OR postnummer = $2);")
            .bind(decode(behandler_type).expect("UTF-8"))
            .bind(behandler.postnummer)
            .fetch_all(&db)
            .await;

        match query_res {
            Ok(result) => Ok(result),
            Err(err) =>  Err(tide::Error::new(tide::StatusCode::InternalServerError, err))
        }
    } else {
        let query = "SELECT b.identifier, b.postnummer, b.kliniktype, b.navn, b.adresse, b.beskrivelse, b.opdateret
                     FROM behandler b JOIN aabningstider oh ON b.identifier = oh.behandler_id 
                     WHERE oh.day_of_week = strftime('%w', 'now', 'utc', '+4 hours') 
                     AND time('now', 'utc', '+4 hours') 
                     BETWEEN oh.open_time AND oh.close_time AND b.kliniktype = $1 AND ($2 = 0 OR b.postnummer = $2);";

        let query_res = sqlx::query_as::<_, Behandler>(query)
            .bind(decode(behandler_type).expect("UTF-8"))
            .bind(behandler.postnummer)
            .fetch_all(&db)
            .await;

        match query_res {
            Ok(result) => Ok(result),
            Err(err) =>  Err(tide::Error::new(tide::StatusCode::InternalServerError, err))
        }
    }
}

