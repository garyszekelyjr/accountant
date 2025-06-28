mod bills;
mod db;
mod issuers;
mod utils;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Add {
        #[command(subcommand)]
        entity: Entity,
    },
    Ls {
        #[command(subcommand)]
        entity: Entity,
    },
    Patch {
        #[command(subcommand)]
        entity: Entity,
    },
    Rm {
        #[command(subcommand)]
        entity: Entity,
    },
    SplitBill,
    StatementToBill,
}

#[derive(Subcommand)]
enum Entity {
    Bill,
    Issuer,
}

fn main() {
    let args = Args::parse();
    let connection = db::get_connection("db.sqlite");

    // db::drop_tables(&connection);
    db::create_tables(&connection);

    match args.command {
        Command::Add { entity } => match entity {
            Entity::Bill => bills::add(connection),
            Entity::Issuer => issuers::add(connection),
        },
        Command::Ls { entity } => match entity {
            Entity::Bill => bills::ls(connection),
            Entity::Issuer => issuers::ls(connection),
        },
        Command::Patch { entity } => match entity {
            Entity::Bill => bills::patch(connection),
            Entity::Issuer => issuers::patch(connection),
        },
        Command::Rm { entity } => match entity {
            Entity::Bill => bills::rm(connection),
            Entity::Issuer => issuers::rm(connection),
        },
        Command::SplitBill => utils::split_bill(connection),
        Command::StatementToBill => utils::statement_to_bill(connection),
    }
}
