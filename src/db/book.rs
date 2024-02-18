use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::models::NewBook;
use crate::schema::books;

pub async fn create_or_get_book(conn: &mut SqliteConnection, major: &str) -> i64 {
    let book = NewBook {
        title: &major.to_string(),
        bid: None,
    };
    diesel::insert_or_ignore_into(books::table)
        .values(book)
        .execute(conn)
        .expect("Error inserting book");

    books::table
        .filter(books::title.eq(major))
        .limit(1)
        .select(books::bid)
        .first::<Option<i64>>(conn)
        .expect("Error getting book")
        .expect("Error unwrapping book")
}
