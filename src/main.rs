use clap::{Parser, Subcommand};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
pub mod models;
pub mod schema;
use crate::models::{NewParty, Party};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn list_parties(conn: &mut SqliteConnection) {
    use self::schema::parties::dsl::*;
    let results = parties
        .limit(10)
        .select(Party::as_select())
        .load(conn)
        .expect("Error loading parties");

    println!("Parties (count={})", results.len());
    for party in results {
        println!("  {}, {}", party.id, party.name);
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

/// List all data in the database
fn list_data(conn: &mut SqliteConnection) {
    list_parties(conn);
}

/// Populate the database with somme parties and contracts
fn populate_data(conn: &mut SqliteConnection) {
    let _buyer = create_party(conn, "Bui Buyer");
    let _seller = create_party(conn, "Shel Seller");
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
