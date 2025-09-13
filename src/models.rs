use crate::schema::url;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = url)]
pub struct Url {
    pub id: i32,
    pub shorten_url: String,
    pub original_url: String,
    pub click_count: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize)]
#[diesel(table_name = url)]
pub struct NewUrl<'a> {
  pub shorten_url: &'a str,
  pub original_url: &'a str
}
