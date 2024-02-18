-- Your SQL goes here
CREATE TABLE
    IF NOT EXISTS users (
        uid INTEGER PRIMARY KEY NOT NULL,
        username TEXT NOT NULL,
        current_rid INTEGER,
        FOREIGN KEY (current_rid) REFERENCES records (rid)
    );

CREATE TABLE
    IF NOT EXISTS books (
        bid INTEGER PRIMARY KEY AUTOINCREMENT,
        title TEXT UNIQUE NOT NULL
    );

CREATE TABLE
    IF NOT EXISTS chaps (
        cid INTEGER PRIMARY KEY AUTOINCREMENT,
        bid INTEGER NOT NULL,
        creator_uid INTEGER NOT NULL,
        heading TEXT NOT NULL,
        FOREIGN KEY (bid) REFERENCES books (bid)
    );

CREATE TABLE
    IF NOT EXISTS records (
        rid INTEGER PRIMARY KEY AUTOINCREMENT,
        uid INTEGER NOT NULL, -- Don't panic, uid is a keyword only in Oracle
        cid INTEGER NOT NULL,
        fromtime INTEGER NOT NULL,
        totime INTEGER,
        FOREIGN KEY (cid) REFERENCES chaps (cid),
        FOREIGN KEY (uid) REFERENCES users (uid)
    );

CREATE TABLE
    IF NOT EXISTS reminders (
        id INTEGER PRIMARY KEY,
        uid INTEGER NOT NULL,
        bid INTEGER,
        cron TEXT,
        interval INTEGER,
        FOREIGN KEY (uid) REFERENCES users (uid)
    );
