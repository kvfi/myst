use std::time::{Duration, Instant, UNIX_EPOCH};

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use log::{debug, info};

use crate::api::LinkItemResponse;
use crate::config::Config;
use crate::models::{NewLink, Setting};
use crate::schema::settings::dsl::*;

struct DbStorageReport {
    total: usize,
    inserted: u32,
    time: u128,
}

impl Default for DbStorageReport {
    fn default() -> DbStorageReport {
        DbStorageReport {
            total: 0,
            inserted: 0,
            time: 0,
        }
    }
}

pub fn establish_connection(cfg: &Config) -> SqliteConnection {
    SqliteConnection::establish(&cfg.database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", cfg.database_url))
}

pub(crate) fn store_pocket_links(conn: &mut SqliteConnection, links: &Vec<LinkItemResponse>) {
    let mut db_storage_report = DbStorageReport {
        total: links.len(),
        ..Default::default()
    };

    let now = Instant::now();

    for link in links {
        let time_added_st = UNIX_EPOCH + Duration::from_secs(link.time_added.parse().unwrap());
        let time_added = DateTime::<Utc>::from(time_added_st)
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        let new_link = NewLink {
            resolved_title: match &link.resolved_title {
                None => "",
                Some(title) => title,
            },
            resolved_url: match &link.resolved_url {
                None => "",
                Some(url) => url,
            },
            resolved_status: &0,
            added_on: &time_added,
            item_id: &link.item_id,
        };

        use crate::schema::links;

        diesel::insert_into(links::table)
            .values(&new_link)
            .execute(conn)
            .expect("Error saving new post");

        db_storage_report.inserted = db_storage_report.inserted + 1;
        info!("Successfully added {}", new_link.resolved_title);
    }

    db_storage_report.time = now.elapsed().as_millis() / 100;

    debug!("Total links: {}", db_storage_report.total);
    debug!("Total inserted: {}", db_storage_report.inserted);
    debug!("Total time: {}", db_storage_report.time);

    info!("Links stored successfully.");
}

pub fn get_job_schedule(conn: &mut SqliteConnection) -> String {
    let result: Setting = settings
        .filter(key.eq("batch_schedule"))
        .first::<Setting>(conn)
        .expect("Error loading settings");
    match result.value {
        None => panic!("Cannot retrieve batch schedule setting"),
        Some(r) => return r
    }
}
