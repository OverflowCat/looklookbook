use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use std::env;
use tokio::sync::{Mutex, OnceCell};

pub mod book;
pub mod chap;
pub mod record;
pub mod reminder;
pub mod user;

static DB: OnceCell<Mutex<SqliteConnection>> = OnceCell::const_new();

pub async fn get_conn() -> &'static Mutex<SqliteConnection> {
    DB.get_or_init(|| async {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = SqliteConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));
        Mutex::new(conn)
    })
    .await
}
