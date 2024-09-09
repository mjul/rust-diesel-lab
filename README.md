# Rust Diesel Lab
Taking the Diesel ORM for a spin.


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


