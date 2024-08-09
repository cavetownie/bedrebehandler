use std::{str::FromStr, time::Duration};
use http_types::headers::HeaderValue;
use tide::security::{CorsMiddleware, Origin};

use tide::{convert::{Deserialize, Serialize}, prelude, Request, Error, log};
use sqlx::{migrate::MigrateDatabase, sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous}, FromRow, Row, Sqlite, SqlitePool};
use urlencoding::decode;

mod controllers;
mod handlers;

use controllers::behandler;


#[derive(Clone)]
struct State {
    db_pool: sqlx::Pool<Sqlite>
}

// Holy shit... this is god awful!
// TODO: Add dotenv
const DB_URL: &str = "sqlite://../db/bedrebehandler.db";
    

#[tokio::main]
async fn main() -> tide::Result<()> {
    log::start();

    // url: https://kerkour.com/sqlite-for-servers
    let conn = SqliteConnectOptions::from_str(DB_URL)?
        // Readers do not block writers, allows for more concurrency
        .journal_mode(SqliteJournalMode::Wal)
        // Will sync at only most critical moment, WAL is corruption
        // safe with syncrhonous=NORMAL
        .synchronous(SqliteSynchronous::Normal)
        .foreign_keys(true)
        .busy_timeout(Duration::new(5, 0));
        
    // We want it to panic, as the application should not start, if it cannot find the db.
    let sqlite_pool = SqlitePool::connect_with(conn).await.unwrap(); 

    sqlx::query("PRAGMA temp_store = memory;")
        .execute(&sqlite_pool)
        .await?;

    sqlx::query("PRAGMA cache_size = 1000000000;")
        .execute(&sqlite_pool)
        .await?;

    let state = State {
        db_pool: sqlite_pool
    };

    let mut app = tide::with_state(state);

    let cors = CorsMiddleware::new()
        .allow_methods("GET".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*")); // Temporary for local test

    app.with(cors);

    // Single physician
    app.at("/behandler/:id").get(behandler::get);

    // Single physician opening hours for scheduling an appointment
    app.at("/behandler/aabningstider/:id").get(behandler::get_opening_hours);

    // All physicians
    app.at("/behandlere").get(behandler::list);

    // Sort by physician type
    app.at("/behandlere/:kliniktype").get(behandler::get_by_type);

    // Sort by physician type
    app.at("/behandler/telefonnumre/:id").get(behandler::get_phone_numbers);

    // Sort by physician type and open TODO: Turn into query parameter
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
