use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};

// Define migrations
const MIGRATIONS_SLICE: &[M<'_>] = &[
    M::up("CREATE TABLE post(title TEXT NOT NULL, contents TEXT NOT NULL);"),

    // In the future, add more migrations here:
    //M::up("ALTER TABLE friend ADD COLUMN email TEXT;"),
];

const MIGRATIONS: Migrations<'_> = Migrations::from_slice(MIGRATIONS_SLICE);

pub fn connect() -> Connection {
    let mut conn = Connection::open_in_memory().unwrap();

    // Apply some PRAGMA, often better to do it outside of migrations
    conn.pragma_update_and_check(None, "journal_mode", "WAL", |_| Ok(()))
        .unwrap();

    MIGRATIONS.to_latest(&mut conn).unwrap();

    conn
}
