use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::models::NewChap;
use crate::schema::chaps;

pub async fn create_or_get_chap<'a>(conn: &mut SqliteConnection, data: NewChap<'a>) -> i64 {
    diesel::insert_or_ignore_into(chaps::table)
        .values(&data)
        .execute(conn)
        .expect("Error inserting chap");

    chaps::table
        .filter(chaps::heading.eq(&data.heading))
        .filter(chaps::bid.eq(data.bid))
        .limit(1)
        .select(chaps::cid)
        .first::<Option<i64>>(conn)
        .expect("Error getting chap")
        .expect("Error unwrapping chap")
}
