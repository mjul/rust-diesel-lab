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
        println!("{}, {}", party.id, party.name);
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

fn main() {
    println!("Hello, world!");

    use self::schema::parties::dsl::*;

    let mut conn: SqliteConnection = establish_connection();

    list_parties(&mut conn);

    let buyer = create_party(&mut conn, "Bui Buyer");
    let seller = create_party(&mut conn, "Shel Seller");

    list_parties(&mut conn);
}
