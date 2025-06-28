use std::fmt::{Display, Formatter, Result};

use inquire::{CustomType, DateSelect, Select};
use sqlite::Connection;

struct IssuerOption {
    id: i64,
    name: String,
}

impl Display for IssuerOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.name)
    }
}

pub fn add(connection: Connection) {
    let rows = connection
        .prepare("SELECT * FROM issuer")
        .unwrap()
        .into_iter()
        .map(|row| row.unwrap());

    let issuers: Vec<IssuerOption> = rows
        .into_iter()
        .map(|row| IssuerOption {
            id: row.read::<i64, _>("id"),
            name: row.read::<&str, _>("name").to_owned(),
        })
        .collect();

    let issuer = Select::new("Select Issuer:", issuers).prompt().unwrap();
    let issue_date = DateSelect::new("Select Issue Date:").prompt().unwrap();
    let due_date = DateSelect::new("Select Due Date:").prompt().unwrap();
    let amount = CustomType::<f64>::new("Enter Amount:")
        .with_formatter(&|i| format!("${:.2}", i))
        .prompt()
        .unwrap();

    let mut statement = connection
        .prepare("INSERT INTO bill (issuer_id, issue_date, due_date, amount) VALUES (?, ?, ?, ?)")
        .unwrap();
    statement.bind((1, issuer.id)).unwrap();
    statement
        .bind((2, issue_date.format("%Y-%m-%d").to_string().as_str()))
        .unwrap();
    statement
        .bind((3, due_date.format("%Y-%m-%d").to_string().as_str()))
        .unwrap();
    statement.bind((4, amount * 100.0)).unwrap();
    statement.next().unwrap();
}

pub fn ls(connection: Connection) {
    let bills = connection
        .prepare(
            "SELECT * FROM bill JOIN issuer ON bill.issuer_id = issuer.id ORDER BY issue_date DESC",
        )
        .unwrap()
        .into_iter()
        .map(|row| row.unwrap());

    for bill in bills {
        let name = bill.read::<&str, _>("name");
        let issue_date = bill.read::<&str, _>("issue_date");
        let due_date = bill.read::<&str, _>("due_date");
        let amount = bill.read::<i64, _>("amount") as f64 / 100.0;
        println!("{name:<25} {issue_date} {due_date} ${amount:.2}");
    }
}

pub fn patch(connection: Connection) {}

pub fn rm(connection: Connection) {}
