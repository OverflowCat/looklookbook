use diesel::prelude::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;

use crate::models::NewChap;
use crate::schema::chaps;

pub fn create_or_get_chap<'a>(
    conn: &mut SqliteConnection,
    data: NewChap<'a>,
) -> Result<i64, Error> {
    diesel::insert_or_ignore_into(chaps::table)
        .values(&data)
        .execute(conn)?;
    chaps::table
        .filter(chaps::heading.eq(&data.heading))
        .filter(chaps::bid.eq(data.bid))
        .limit(1)
        .select(chaps::cid)
        .first::<Option<i64>>(conn)
        .map(|x| x.unwrap())
}
