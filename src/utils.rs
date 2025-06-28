use std::fmt::{Display, Formatter, Result};

use inquire::MultiSelect;
use sqlite::Connection;

const RATIO: f64 = 0.6357;

struct BillOption {
    issuer: String,
    issue_date: String,
    due_date: String,
    amount: f64,
}

impl Display for BillOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}, {}, {}, {}",
            self.issuer, self.issue_date, self.due_date, self.amount
        )
    }
}

pub fn split_bill(connection: Connection) {
    let rows = connection
        .prepare("SELECT * FROM bill JOIN issuer on bill.issuer_id = issuer.id")
        .unwrap()
        .into_iter()
        .map(|row| row.unwrap());

    let options: Vec<BillOption> = rows
        .into_iter()
        .map(|row| BillOption {
            issuer: row.read::<&str, _>("name").to_owned(),
            issue_date: row.read::<&str, _>("issue_date").to_owned(),
            due_date: row.read::<&str, _>("due_date").to_owned(),
            amount: row.read::<i64, _>("amount") as f64 / 100.0,
        })
        .collect();

    let bills = MultiSelect::new("Select Bills:", options).prompt().unwrap();

    let mut total = 0.0;
    let mut gary_total = 0.0;
    let mut paige_total = 0.0;

    for bill in bills {
        let gary_bill = (bill.amount * RATIO * 100.0).round() / 100.0;
        let paige_bill = (bill.amount * (1.0 - RATIO) * 100.0).round() / 100.0;

        total += bill.amount;
        gary_total += gary_bill;
        paige_total += paige_bill;

        println!("Bill: ${}", bill.amount);
        println!("Gary's Bill: ${gary_bill:.2}");
        println!("Paige's Bill: ${paige_bill:.2}");
        println!()
    }

    println!("Total: ${total:.2}");
    println!("Gary's Total: ${gary_total:.2}");
    println!("Paige's Total: ${paige_total:.2}");
}
