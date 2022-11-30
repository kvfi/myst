extern crate core;

use simple_logger::SimpleLogger;

mod api;
mod batch;
mod config;
mod db;
mod models;
mod schema;
mod util;

fn main() {
    SimpleLogger::new().init().unwrap();
    let mut cfg = config::get_config(None);
    let conn = &mut db::establish_connection(&cfg);
    let setting_schedule = db::get_job_schedule(conn);
    batch::store_new_links_job(conn, &setting_schedule, &mut cfg);
}
