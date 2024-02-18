use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::books)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Book {
    pub bid: Option<i64>,
    pub title: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::books)]
pub struct NewBook<'a> {
    pub bid: Option<i64>,
    pub title: &'a String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::chaps)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Chap {
    pub cid: Option<i64>,
    pub bid: i64,
    pub creator_uid: i64,
    pub heading: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::chaps)]
pub struct NewChap<'a> {
    pub cid: Option<i64>,
    pub bid: i64,
    pub creator_uid: i64,
    pub heading: &'a String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::records)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Record {
    pub rid: Option<i64>,
    pub uid: i64,
    pub cid: i64,
    pub fromtime: i64,
    pub totime: Option<i64>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::records)]
pub struct NewRecord {
    pub rid: Option<i64>,
    pub uid: i64,
    pub cid: i64,
    pub fromtime: i64,
    pub totime: Option<i64>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::reminders)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Reminder {
    pub id: Option<i64>,
    pub uid: i64,
    pub bid: Option<i64>,
    pub cron: Option<String>,
    pub interval: Option<i64>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub uid: i64,
    pub username: String,
    pub current_rid: Option<i64>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub uid: i64,
    pub username: &'a String,
    pub current_rid: Option<i64>,
}
