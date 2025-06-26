use inquire::Select;
use sqlite::Connection;

pub fn add(connection: &Connection) {
    let rows = connection
        .prepare("SELECT * FROM issuers")
        .unwrap()
        .into_iter()
        .map(|row| row.unwrap());
    let issuers: Vec<String> = rows
        .into_iter()
        .map(|row| row.read::<&str, _>("name").to_owned())
        .collect();
    let issuer = Select::new("Select Issuer:", issuers).prompt().unwrap();
}

pub fn ls(connection: &Connection) {}

pub fn patch(connection: &Connection) {}

pub fn rm(connection: &Connection) {}
