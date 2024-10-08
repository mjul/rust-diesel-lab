# Rust Diesel Lab
Taking the Diesel ORM (object-relational mapper) for a spin.
Specifically, database setup and migrations and using a SQLite database with Rust. 

## What is Diesel
[Diesel](https://diesel.rs/) provides synchronous database access and mapping between Rust types and database relations.
It has relatively small overhead in terms of code as just a few derive macros are needed.
Diesel provides compile-time checking for the program to database mappings.
It also provides database migrations via SQL "up" and "down" scripts.

## SeaORM, The Other Popular ORM
Another frequently used ORM for Rust is [SeaORM](https://www.sea-ql.org/SeaORM/). Unlike Diesel, it is async and takes
a more dynamic approach, checking the database and code correspondence at runtime instead
of compile time. It appears to require more code for the mappings than Diesel. Its migration model is an 
embedded Rust DSL rather than SQL, making the database migrations "more" portable 
across different database back-ends.


# Quick Start
This repo build a command line app that manipulates a SQLite database.
Try it out:

```
    diesel migration run
    cargo run list
    cargo run populate
    cargo run list
```

# Setup 

Refer to the Diesel documentation: https://diesel.rs/guides/getting-started.html

## Install Diesel CLI

```
    cargo install diesel_cli --no-default-features --features sqlite-bundled
```

This installs the Diesel command-line interface with a statically linked SQLite.

If you have a dynamic link library for sqlite3 on your system you can also do this:

```
    cargo install diesel_cli --no-default-features --features sqlite
```

## Set up the database
The name of the database file is defined in the `.env` file.
To set up the database and create a `migrations` folder run this command:

```
    diesel setup
```

## Define the initial DB migration

```
    diesel migration generate create_contracts
```

Now, fill out the data definitions for the `up.sql` and `down.sql` migrations under the `migrations` folder.

## Install the tables

```
    diesel migration run
```

To rebuild the last migration you can run this
```
    diesel migration redo
```


# Code

- `.env` contains the environment settings
- `diesel.toml` is the configuration file for Diesel, it tells Diesel
  to generate the [`schema.rs`](src/schema.rs) file and where to find
  the migration scripts.
- `src`
  - `main.rs` is the entry point.
  - `schema.rs` is generated by Diesel and defines the database schema.
  - `models.rs` is the Rust type definitions for to the database model.


# Notes

## Date and Time
Diesel has a `chrono` feature that enables it to use the Chrono crate
with *e.g.* `NaiveDate` for SQL time types like `DATE`.  This is used
for effective dates on the contracts and framework agreements. See
[`models.rs`](src/models.rs).

## Foreign Key Relations

See the guide to Diesel relations here https://diesel.rs/guides/relations.html

Child-to-parent relationships can be marked with the 
`belongs_to` derive macro, _e.g._ `#[diesel(belongs_to(FrameworkAgreement))]`.
These relationships are always child to parent.

You can query these with an inner join, which gives you a tuple of 
database model instances for each join:

```rust
    // Note how we inner join to get the corresponding Framework Agreement
    let results = self::schema::contracts::table
        .inner_join(self::schema::framework_agreements::table)
        .select((Contract::as_select(), FrameworkAgreement::as_select()))
        .load::<(Contract, FrameworkAgreement)>(conn)
        .expect("Error loading contracts");
```

You can use explicit inner joins to select the keys if the fields does not have the
default names:

```rust
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
```

Apparently, it is not possible to have a relation like a Contract associated to two 
Party instances from the same Party relation (table) this way, so it is quite limited.

Joining with multiple foreign keys into the same table is possible via
the `alias!` macro, see https://docs.diesel.rs/2.2.x/diesel/macro.alias.html





