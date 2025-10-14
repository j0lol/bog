use rusqlite::Connection;
use rusqlite_migration::{M, Migrations};

// Define migrations
const MIGRATIONS_SLICE: &[M<'_>] = &[
    M::up("CREATE TABLE post(title TEXT NOT NULL, contents TEXT NOT NULL);"),
    M::up(
        r#"
        INSERT INTO post(title, contents)
        VALUES ("Foo Post", "Lorum ipsum. Blablala.")
        "#,
    ),
];

const MIGRATIONS: Migrations<'_> = Migrations::from_slice(MIGRATIONS_SLICE);

pub fn connect() -> Connection {
    let mut conn = Connection::open("./db.db3").unwrap();

    // Apply some PRAGMA, often better to do it outside of migrations
    conn.pragma_update_and_check(None, "journal_mode", "WAL", |_| Ok(()))
        .unwrap();

    MIGRATIONS.to_latest(&mut conn).unwrap();

    conn
}
