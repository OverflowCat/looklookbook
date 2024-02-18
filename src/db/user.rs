use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::models::{NewUser, User};
use crate::schema::users;

pub async fn create_or_get_user<'a>(conn: &mut SqliteConnection, data: NewUser<'a>) -> User {
    diesel::insert_or_ignore_into(users::table)
        .values(&data)
        .execute(conn)
        .expect(&format!("Error inserting user"));

    let users: Vec<User> = users::table
        .filter(users::uid.eq(data.uid))
        .select(User::as_select())
        .load(conn)
        .expect("Error getting user");

    users.into_iter().nth(0).unwrap() // return the first user without copying
}

pub async fn update_current_record(conn: &mut SqliteConnection, uid: i64, rid: i64) {
    diesel::update(users::table.filter(users::uid.eq(uid)))
        .set(users::current_rid.eq(rid))
        .execute(conn)
        .expect("Error updating user's current record");
}
