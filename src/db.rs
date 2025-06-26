use sqlite::Connection;

const TABLES: [&str; 2] = ["issuers", "bills"];

pub fn drop_tables(connection: &Connection) {
    for table in TABLES {
        connection
            .execute(format!("DROP TABLE IF EXISTS {table}"))
            .unwrap();
    }
}

pub fn create_tables(connection: &Connection) {
    connection
        .execute(format!(
            "CREATE TABLE issuers (
                id INTEGER PRIMARY KEY,
                name TEXT UNIQUE
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
