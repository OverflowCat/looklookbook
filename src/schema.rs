// @generated automatically by Diesel CLI.

diesel::table! {
    books (bid) {
        bid -> Nullable<BigInt>,
        title -> Text,
    }
}

diesel::table! {
    chaps (cid) {
        cid -> Nullable<BigInt>,
        bid -> BigInt,
        creator_uid -> BigInt,
        heading -> Text,
    }
}

diesel::table! {
    records (rid) {
        rid -> Nullable<BigInt>,
        uid -> BigInt,
        cid -> BigInt,
        fromtime -> BigInt,
        totime -> Nullable<BigInt>,
    }
}

diesel::table! {
    reminders (id) {
        id -> Nullable<BigInt>,
        uid -> BigInt,
        bid -> Nullable<BigInt>,
        cron -> Nullable<Text>,
        interval -> Nullable<BigInt>,
    }
}

diesel::table! {
    users (uid) {
        uid -> BigInt,
        username -> Text,
        current_rid -> Nullable<BigInt>,
    }
}

diesel::joinable!(chaps -> books (bid));
diesel::joinable!(records -> chaps (cid));
diesel::joinable!(reminders -> users (uid));

diesel::allow_tables_to_appear_in_same_query!(
    books,
    chaps,
    records,
    reminders,
    users,
);
