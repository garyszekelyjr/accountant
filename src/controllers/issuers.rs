use inquire::Text;
use sqlite::Connection;

pub fn add(connection: &Connection) {
    let name = Text::new("Issuer's Name:").prompt().unwrap();
    let mut statement = connection
        .prepare("INSERT INTO issuers (name) VALUES (?)")
        .unwrap();
    statement.bind((1, name.as_str())).unwrap();
    statement.next().unwrap();
}

pub fn ls(connection: &Connection) {
    let issuers = connection
        .prepare("SELECT * FROM issuers")
        .unwrap()
        .into_iter()
        .map(|row| row.unwrap());

    for issuer in issuers {
        let name = issuer.read::<&str, _>("name");
        println!("{name}");
    }
}

pub fn patch(connection: &Connection) {}

pub fn rm(connection: &Connection) {}
