use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
pub mod models;
pub mod schema;


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


fn main() {
    println!("Hello, world!");

    use self::schema::parties::dsl::*;
    use self::models::{Party};
    
    let mut conn : SqliteConnection = establish_connection();
    let results = parties
        .limit(5)
        .select(Party::as_select())
        .load(&mut conn)
        .expect("Error loading parties");

    println!("Parties (count={})", results.len());
    for party in results {
        println!("{}, {}", party.id, party.name);
    }
    
}
