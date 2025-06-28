use inquire::Text;
use sqlite::Connection;

pub fn add(connection: &Connection) {
    let name = Text::new("Issuer's Name:").prompt().unwrap();
    let mut statement = connection
        .prepare("INSERT INTO issuer (name) VALUES (?)")
        .unwrap();
    statement.bind((1, name.as_str())).unwrap();
    statement.next().unwrap();
}

pub fn ls(connection: &Connection) {
    let rows = connection
        .prepare("SELECT * FROM issuer")
        .unwrap()
        .into_iter()
        .map(|row| row.unwrap());

    for row in rows {
        let name = row.read::<&str, _>("name");
        println!("{name}");
    }
}

pub fn patch(connection: &Connection) {}

pub fn rm(connection: &Connection) {}
