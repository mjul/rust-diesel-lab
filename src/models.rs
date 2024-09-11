use chrono::NaiveDate;
use diesel::prelude::*;

/// A contract party.
#[derive(Queryable, Selectable, Identifiable)]
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

/// A framework agreement. A [Contract] may be a specific instance this.
#[derive(Queryable, Selectable, Identifiable)]
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

/// A contract instance.
#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = crate::schema::contracts)]
#[diesel(belongs_to(FrameworkAgreement))]
#[diesel(belongs_to(Party, foreign_key = buyer_id))]
//#[diesel(belongs_to(Party, foreign_key = seller_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Contract {
    pub id: i32,
    pub title: String,
    pub effective_date: NaiveDate,
    pub buyer_id: i32,
    pub seller_id: i32,
    pub framework_agreement_id: i32,
}

/// A new [Contract].
#[derive(Insertable)]
#[diesel(table_name = crate::schema::contracts)]
pub struct NewContract<'a> {
    pub title: &'a str,
    pub effective_date: &'a NaiveDate,
    pub buyer_id: i32,
    pub seller_id: i32,
    pub framework_agreement_id: i32,
}
