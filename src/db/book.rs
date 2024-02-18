use diesel::prelude::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;

use crate::models::NewBook;
use crate::schema::books;

pub fn create_or_get_book(conn: &mut SqliteConnection, major: &str) -> Result<i64, Error> {
    let book = NewBook {
        title: &major.to_string(),
        bid: None,
    };
    diesel::insert_or_ignore_into(books::table)
        .values(book)
        .execute(conn)?;
    let res = books::table
        .filter(books::title.eq(major))
        .limit(1)
        .select(books::bid)
        .first::<Option<i64>>(conn);
    res.map(|x| x.unwrap())
}
