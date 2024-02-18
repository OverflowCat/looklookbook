use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::models::{NewRecord, Record};
use crate::schema::records;

pub async fn create_and_get_record(conn: &mut SqliteConnection, data: NewRecord) -> Rec {
    diesel::insert_into(records::table)
        .values(&data)
        .returning(Record::as_returning())
        .get_result(conn)
        .expect("Error inserting record")
}
