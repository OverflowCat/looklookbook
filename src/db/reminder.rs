use diesel::prelude::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;

use crate::models::{NewReminder, Reminder};
use crate::schema::reminders;

pub fn create_reminder(conn: &mut SqliteConnection, data: NewReminder) -> Result<(), Error> {
    diesel::insert_into(reminders::table)
        .values(&data)
        .execute(conn)?;
    Ok(())
}

pub fn delete_reminder(conn: &mut SqliteConnection, uid: i64) -> Result<usize, Error> {
    diesel::delete(reminders::table.filter(reminders::uid.eq(uid))).execute(conn)
}

pub fn get_default_reminders(conn: &mut SqliteConnection, uid: i64) -> Result<Reminder, Error> {
    reminders::table
        .filter(reminders::uid.eq(uid))
        .filter(reminders::bid.is_null())
        .select(Reminder::as_select())
        .first::<Reminder>(conn)
}
