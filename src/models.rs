use diesel::prelude::*;

/// A contract party.
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::parties)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Party {
    pub id: i32,
    pub name: String,
}
