use std::time::Duration;

use diesel::prelude::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;

use crate::models::{NewRecord, Record, User};
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

pub fn get_duration_sum(conn: &mut SqliteConnection, uid: i64) -> Result<Duration, Error> {
    let res = records::table
        .filter(records::uid.eq(uid))
        .filter(records::totime.is_not_null())
        // .select(dsl::sum(records::totime.assume_not_null() - records::fromtime).nullable())
        .select((records::totime.assume_not_null(), records::fromtime))
        .load(conn);
    res.map(|x: Vec<(i64, i64)>| {
        x.iter()
            .map(|(totime, fromtime)| Duration::from_secs((totime - fromtime) as u64))
            .sum()
    })
}

pub fn get_duration_sum_rank(
    conn: &mut SqliteConnection,
    _uid: i64,
) -> Result<Vec<(Duration, User)>, Error> {
    crate::db::user::get_all_users(conn)?
        .into_iter()
        .map(|user| Ok((get_duration_sum(conn, user.uid)?, user)))
        .collect()
}
