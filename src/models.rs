use chrono::NaiveDate;
use diesel::prelude::*;

/// A contract party.
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::parties)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Party {
    pub id: i32,
    pub name: String,
}

/// A new [Party]
#[derive(Insertable)]
#[diesel(table_name = crate::schema::parties)]
pub struct NewParty<'a> {
    pub name: &'a str,
}

/// A contract party.
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::contracts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Contract {
    pub id: i32,
    pub title: String,
    pub effective_date: NaiveDate,
}

/// A framework agreement. A [Contract] may be a specific instance this.
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::framework_agreements)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct FrameworkAgreement {
    pub id: i32,
    pub title: String,
    pub effective_date: NaiveDate,
}

/// A new [FrameworkAgreement].
#[derive(Insertable)]
#[diesel(table_name = crate::schema::framework_agreements)]
pub struct NewFrameworkAgreement<'a> {
    pub title: &'a str,
    pub effective_date: &'a NaiveDate,
}
