extern crate core;

use myst_core::db;
use simple_logger::SimpleLogger;

mod api;
mod batch;
mod config;
mod util;

fn main() {
    SimpleLogger::new().init().unwrap();
    let mut cfg = config::get_config(None);
    let conn = &mut db::establish_connection(&cfg.database_url);
    let setting_schedule = db::get_job_schedule(conn);
    batch::store_new_links_job(conn, &setting_schedule, &mut cfg);
}
