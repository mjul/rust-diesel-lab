use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
pub mod models;
pub mod schema;
use crate::models::{Contract, FrameworkAgreement, NewFrameworkAgreement, NewParty, Party};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn list_parties(conn: &mut SqliteConnection) {
    use self::schema::parties::dsl::*;
    let results = parties
        .limit(100)
        .select(Party::as_select())
        .load(conn)
        .expect("Error loading parties");

    println!("Parties (count={})", results.len());
    for party in results {
        println!("  {:>5}, {}", party.id, party.name);
    }
}

fn list_framework_agreements(conn: &mut SqliteConnection) {
    use self::schema::framework_agreements::dsl::*;
    let results = framework_agreements
        .limit(100)
        .select(FrameworkAgreement::as_select())
        .load(conn)
        .expect("Error loading framework agreements");

    println!("Framework agreements (count={})", results.len());
    for fa in results {
        println!(
            "  {:>5}, {:>10}, {:<40}",
            fa.id, fa.effective_date, fa.title
        );
    }
}

fn list_contracts(conn: &mut SqliteConnection) {
    use self::schema::contracts::dsl::*;
    let results = contracts
        .limit(100)
        .select(Contract::as_select())
        .load(conn)
        .expect("Error loading contracts");

    println!("Contracts (count={})", results.len());
    for contract in results {
        println!(
            "  {:>5}, {:>10}, {:<40}, ({})",
            contract.id, contract.effective_date, contract.title, "TODO"
        );
    }
}

fn create_party(conn: &mut SqliteConnection, name: &str) -> Party {
    use crate::schema::parties;

    let new_party = NewParty { name };

    diesel::insert_into(parties::table)
        .values(&new_party)
        .returning(Party::as_returning())
        .get_result(conn)
        .expect("Error saving new party")
}

fn create_framework_agreement(
    conn: &mut SqliteConnection,
    title: &str,
    effective_date: &NaiveDate,
) -> FrameworkAgreement {
    use crate::schema::framework_agreements;

    let new_fa = NewFrameworkAgreement {
        title,
        effective_date,
    };

    diesel::insert_into(framework_agreements::table)
        .values(&new_fa)
        .returning(FrameworkAgreement::as_returning())
        .get_result(conn)
        .expect("Error saving new framework agreement")
}

/// List all data in the database
fn list_data(conn: &mut SqliteConnection) {
    list_parties(conn);
    list_framework_agreements(conn);
    list_contracts(conn);
}

/// Populate the database with somme parties and contracts
fn populate_data(conn: &mut SqliteConnection) {
    let _buyer = create_party(conn, "Bui Buyer");
    let _seller = create_party(conn, "Shel Seller");
    let _fa = create_framework_agreement(
        conn,
        "MSA v1.1",
        &NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
    );
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    List,
    Populate,
}

fn main() {
    let mut conn: SqliteConnection = establish_connection();

    let cli = Cli::parse();
    match cli.command {
        Command::List => list_data(&mut conn),
        Command::Populate => populate_data(&mut conn),
    }
}
