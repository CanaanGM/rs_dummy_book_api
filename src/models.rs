use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::books;

#[derive(Deserialize, Serialize, Queryable, AsChangeset)]
pub struct Book {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub category: String,
    #[serde(skip_deserializing)]
    pub created_at: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = books)]

pub struct NewBook {
    pub name: String,
    pub category: String,
}
