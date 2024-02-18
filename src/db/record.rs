use diesel::prelude::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;

use crate::models::{NewRecord, Record};
use crate::schema::records;

pub fn create_and_get_record(
    conn: &mut SqliteConnection,
    data: NewRecord,
) -> Result<Record, Error> {
    diesel::insert_into(records::table)
        .values(&data)
        .returning(Record::as_returning())
        .get_result(conn)
}

pub fn delete_record(conn: &mut SqliteConnection, rid: i64) -> Result<(), Error> {
    diesel::delete(records::table.filter(records::rid.eq(rid))).execute(conn)?;
    Ok(())
}

pub fn finish_record(conn: &mut SqliteConnection, rid: i64, totime: i64) -> Result<i64, Error> {
    let res: Record = diesel::update(records::table.filter(records::rid.eq(rid)))
        .set(records::totime.eq(Some(totime)))
        .returning(Record::as_returning())
        .get_result(conn)?;
    Ok(res.fromtime)
}

pub fn get_record(conn: &mut SqliteConnection, rid: i64) -> Result<Record, Error> {
    records::table
        .filter(records::rid.eq(rid))
        .select(Record::as_select())
        .first(conn)
}
