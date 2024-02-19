use diesel::prelude::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;

use crate::models::{NewUser, User};
use crate::schema::users;

pub fn create_or_get_user<'a>(
    conn: &mut SqliteConnection,
    data: NewUser<'a>,
) -> Result<User, Error> {
    diesel::insert_or_ignore_into(users::table)
        .values(&data)
        .execute(conn)?;
    let users: Vec<User> = users::table
        .filter(users::uid.eq(data.uid))
        .select(User::as_select())
        .load(conn)?;
    Ok(users.into_iter().nth(0).unwrap()) // return the first user without copying
}

pub fn update_current_record(
    conn: &mut SqliteConnection,
    uid: i64,
    rid: Option<i64>,
) -> Result<(), Error> {
    diesel::update(users::table.filter(users::uid.eq(uid)))
        .set(users::current_rid.eq(rid))
        .execute(conn)?;
    Ok(())
}

pub fn get_all_users(conn: &mut SqliteConnection) -> Result<Vec<User>, Error> {
    users::table.select(User::as_select()).load(conn)
}
