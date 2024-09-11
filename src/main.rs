use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::{collections::HashMap, env};
pub mod models;
pub mod schema;
use crate::models::{
    Contract, FrameworkAgreement, NewContract, NewFrameworkAgreement, NewParty, Party,
};

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
    // We and use Contract::belonging_to(&all_fas) as an alternate query
    let _all_fas = self::schema::framework_agreements::table
        .select(FrameworkAgreement::as_select())
        .load(conn)
        .expect("Error loading framework agreements");

    let all_parties = self::schema::parties::table
        .select(Party::as_select())
        .load(conn)
        .expect("Error loading parties");

    let (_buyers, _sellers) = diesel::alias!(
        self::schema::parties as buyer,
        self::schema::parties as seller
    );

    // Here is a simple join via the belongs_to derive macro association
    let _result_c_fa = self::schema::contracts::table
        .inner_join(self::schema::framework_agreements::table)
        .select((Contract::as_select(), FrameworkAgreement::as_select()))
        .load::<(Contract, FrameworkAgreement)>(conn)
        .expect("Error loading contracts");

    // This is the best we can do with inner joins:
    // if we have two associations from contracts to parties (buyer and seller)
    // the Associations trait does not work
    let result_c_fa_b = Contract::belonging_to(&all_parties)
        .inner_join(self::schema::framework_agreements::table)
        .inner_join(
            self::schema::parties::table
                .on(self::schema::parties::id.eq(self::schema::contracts::buyer_id)),
        )
        .select((
            Contract::as_select(),
            FrameworkAgreement::as_select(),
            Party::as_select(),
        ))
        .load(conn)
        .expect("Error loading contracts");

    // If we cannot inner join the second party (seller),
    // we can join it manually
    let party_by_id: HashMap<i32, Party> =
        all_parties.into_iter().map(|p| (p.id.clone(), p)).collect();

    let result = result_c_fa_b;

    println!("Contracts (count={})", result.len());
    for (contract, fa, buyer) in result {
        // manually join the seller (challenge: find a better way to do this)
        let seller = party_by_id
            .get(&contract.seller_id)
            .expect("Seller must exist in Parties");
        println!(
            "  {:>5}, Eff: {:>10}, FA: {:<10}, {:<20}, B: {:<20}, S: {:<20}",
            contract.id, contract.effective_date, fa.title, contract.title, buyer.name, seller.name
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

/// Create a contract instance for a [FrameworkAgreement].
fn create_contract_for_fa(
    conn: &mut SqliteConnection,
    framework_agreement: &FrameworkAgreement,
    title: &str,
    effective_date: &NaiveDate,
    buyer: &Party,
    seller: &Party,
) -> Contract {
    use crate::schema::contracts;

    let new_contract = NewContract {
        title,
        effective_date,
        buyer_id: buyer.id,
        seller_id: seller.id,
        framework_agreement_id: framework_agreement.id,
    };

    diesel::insert_into(contracts::table)
        .values(&new_contract)
        .returning(Contract::as_returning())
        .get_result(conn)
        .expect("Error saving new contract")
}

/// List all data in the database
fn list_data(conn: &mut SqliteConnection) {
    list_parties(conn);
    list_framework_agreements(conn);
    list_contracts(conn);
}

/// Populate the database with somme parties and contracts
fn populate_data(conn: &mut SqliteConnection) {
    let buyer = create_party(conn, "Bui Buyer");
    let seller = create_party(conn, "Shel Seller");
    let msa10 = create_framework_agreement(
        conn,
        "MSA v1.0",
        &NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
    );
    let msa11 = create_framework_agreement(
        conn,
        "MSA v1.1",
        &NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
    );
    let _c_a = create_contract_for_fa(
        conn,
        &msa10,
        "Foobar 2023 License",
        &NaiveDate::from_ymd_opt(2023, 7, 1).unwrap(),
        &buyer,
        &seller,
    );
    let _c_b = create_contract_for_fa(
        conn,
        &msa11,
        "Foobar 2024 License",
        &NaiveDate::from_ymd_opt(2024, 7, 1).unwrap(),
        &buyer,
        &seller,
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
