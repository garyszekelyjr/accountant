use std::{
    fmt::{Display, Formatter, Result},
    fs,
};

use inquire::{MultiSelect, Select};
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

#[derive(serde::Deserialize)]
struct StatementRow {
    #[serde(rename = "Transaction Date")]
    transaction_date: String,
    #[serde(rename = "Posted Date")]
    posted_date: String,
    #[serde(rename = "Card No.")]
    card_no: i64,
    #[serde(rename = "Category")]
    category: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Debit")]
    debit: Option<f64>,
    #[serde(rename = "Credit")]
    credit: Option<f64>,
}

pub fn statement_to_bill(connection: Connection) {
    let mut paths: Vec<String> = fs::read_dir("./transactions")
        .unwrap()
        .map(|result| {
            result
                .unwrap()
                .path()
                .file_stem()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap()
        })
        .collect();

    paths.sort_by(|a, b| b.cmp(a));

    let path = Select::new("Select Capital One Bill:", paths)
        .prompt()
        .unwrap();

    let mut rdr = csv::Reader::from_path(format!("./transactions/{}.csv", path)).unwrap();

    let mut total = 0.0;
    for row in rdr.deserialize() {
        let row: StatementRow = row.unwrap();
        if row.category != "Payment/Credit" {
            match row.debit {
                Some(debit) => total += (debit * 100.0).round() / 100.0,
                None => {}
            }
            match row.credit {
                Some(credit) => total -= (credit * 100.0).round() / 100.0,
                None => {}
            }
        }
    }

    let rows: Vec<i64> = connection
        .prepare("SELECT id FROM issuer WHERE name = 'Capital One'")
        .unwrap()
        .into_iter()
        .map(|row| row.unwrap().read::<i64, _>("id"))
        .collect();
    let issuer_id = rows[0];

    let issue_date = path;
    let parts: Vec<&str> = issue_date.split("-").collect();
    let mut year = parts[0].parse::<i64>().unwrap();
    let mut month = parts[1].parse::<i64>().unwrap() + 1;
    let day = "12";

    if month > 12 {
        month = 1;
        year += 1;
    }

    let due_date = format!("{}-{:0>2}-{}", year, month, day);

    let mut statement = connection
        .prepare("INSERT INTO bill (issuer_id, issue_date, due_date, amount) VALUES (?, ?, ?, ?)")
        .unwrap();
    statement.bind((1, issuer_id)).unwrap();
    statement.bind((2, issue_date.as_str())).unwrap();
    statement.bind((3, due_date.as_str())).unwrap();
    statement.bind((4, (total * 100.0).round())).unwrap();
    statement.next().unwrap();
}

pub fn split_bill(connection: Connection) {
    let rows = connection
        .prepare(
            "SELECT * FROM bill JOIN issuer on bill.issuer_id = issuer.id ORDER BY issue_date DESC",
        )
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
