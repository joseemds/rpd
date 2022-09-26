use rusqlite::{Connection};

pub fn create_connection() -> Connection{
    Connection::open("db.sqlite").unwrap()
}


pub fn create_tables(conn: Connection) -> Connection{
    conn.execute_batch("
        BEGIN;
        CREATE TABLE IF NOT EXISTS feelings (
            id INTEGER PRIMARY KEY,
            name CHAR(255)
        );
        CREATE TABLE IF NOT EXISTS thoughts (
        id   INTEGER PRIMARY KEY,
        thought CHAR(255) NOT NULL,
        situation CHAR(255) NOT NULL,
        intensity INTEGER NOT NULL,
        feeling_id INTEGER NOT NULL,
        FOREIGN KEY (feeling_id)
            REFERENCES feelings (id)
        );
        COMMIT;").unwrap();

    conn
}
