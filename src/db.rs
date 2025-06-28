use sqlite::Connection;

pub fn drop_tables(connection: &Connection) {
    const TABLES: [&str; 2] = ["issuer", "bill"];
    for table in TABLES {
        connection
            .execute(format!("DROP TABLE IF EXISTS {table}"))
            .unwrap();
    }
}

pub fn create_tables(connection: &Connection) {
    connection
        .execute(format!(
            "CREATE TABLE IF NOT EXISTS issuer (
                id INTEGER PRIMARY KEY,
                name TEXT UNIQUE
            )"
        ))
        .unwrap();

    connection
        .execute(format!(
            "CREATE TABLE IF NOT EXISTS bill (
                id INTEGER PRIMARY KEY,
                issuer_id INTEGER,
                issue_date DATE,
                due_date DATE,
                amount INTEGER
            )"
        ))
        .unwrap();
}

pub fn get_connection(path: &str) -> Connection {
    return match sqlite::open(path) {
        Ok(connection) => connection,
        Err(err) => panic!("{err}"),
    };
}
