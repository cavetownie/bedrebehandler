use tide::{convert::{Deserialize, Serialize}, prelude, Request, Error, log};
use sqlx::{migrate::MigrateDatabase, FromRow, Row, Sqlite, SqlitePool};
use urlencoding::decode;

mod controllers;
mod handlers;
use controllers::behandler;

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

#[derive(Clone)]
struct State {
    db_pool: sqlx::Pool<Sqlite>
}

// Holy shit... this is god awful!
// TODO: Add dotenv
const DB_URL: &str = "sqlite://../../db/bedrebehandler.db";

#[tokio::main]
async fn main() -> tide::Result<()> {
    log::start();

    let state = State {
        // We want it to panic, as the application should not start, if it cannot find the db.
        db_pool: SqlitePool::connect(DB_URL).await.unwrap()
    };

    let mut app = tide::with_state(state);

    // All physicians
    app.at("/behandlere").get(behandler::list);

    // Sort by physician type
    app.at("/behandlere/:kliniktype").get(behandler::get_by_type);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
