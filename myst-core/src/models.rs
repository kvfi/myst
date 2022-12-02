use diesel::prelude::*;

use crate::schema::links;

#[derive(Queryable)]
pub struct Link {
    pub id: i32,
    pub resolved_title: String,
    pub resolved_url: String,
    pub resolved_status: i32,
    pub added_on: String,
    pub item_id: String,
}

#[derive(Insertable)]
#[diesel(table_name = links)]
pub struct NewLink<'a> {
    pub resolved_title: &'a str,
    pub resolved_url: &'a str,
    pub resolved_status: &'a i32,
    pub added_on: &'a str,
    pub item_id: &'a str,
}

#[derive(Queryable, Debug)]
pub struct Setting {
    pub id: i32,
    pub key: String,
    pub value: Option<String>,
}
