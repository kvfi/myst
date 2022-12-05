use diesel::{QueryDsl, RunQueryDsl};
use myst_core::db;
use myst_core::models::Link;
use myst_core::schemas::links::dsl::*;

pub fn load_links(limit: i64, offset: i64) -> Link {
    let conn = &mut db::establish_connection("postgres://myst:myst@localhost:9500/myst");
    let results = links
        .first::<Link>(conn)
        .expect("Error loading posts");

    results
}
