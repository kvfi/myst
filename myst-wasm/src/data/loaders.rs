use myst_core::db;
use myst_core::models::Link;
use myst_core::schema::links::dsl::*;
use diesel::{QueryDsl, RunQueryDsl};

pub fn load_links(limit: i64, offset: i64) {
    let connection = &mut db::establish_connection("postgres://myst:myst@localhost:9500/myst");
    let results = links
        .limit(limit)
        .offset(offset)
        .load::<Link>(connection)
        .expect("Error loading posts");

    println!("Displaying {} links", results.len());
    for link in results {
        println!("{}", link.resolved_title);
        println!("-----------\n");
        println!("{}", link.resolved_status);
    }
}