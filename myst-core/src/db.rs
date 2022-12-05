use crate::models::Setting;
use crate::schemas::settings::dsl::*;
use diesel::prelude::*;
use diesel::Connection;
use diesel::PgConnection;

pub struct DbStorageReport {
    pub total: usize,
    pub inserted: u32,
    pub time: u128,
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

pub fn establish_connection(url: &str) -> PgConnection {
    PgConnection::establish(url).unwrap_or_else(|_| panic!("Error connecting to {}", url))
}

pub fn get_job_schedule(conn: &mut PgConnection) -> String {
    let result: Setting = settings
        .filter(key.eq("batch_schedule"))
        .first::<Setting>(conn)
        .expect("Error loading settings");
    match result.value {
        None => panic!("Cannot retrieve batch schedule setting"),
        Some(r) => return r,
    }
}
