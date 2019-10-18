use lib::api;
use lib::api::killmail::KillMail;
use lib::api::zkb::Package;
use lib::models::{DB, Connection, Hash};
use std::thread;


fn flush(records: &Vec<KillMail>) -> Option<usize> {
    let conn = DB::connection();
    if let Some(_) = DB::save_all(&conn, &records).ok() {
        Some(records.len())
    } else {
        None
    }
}

fn main() {
    let mut records: Vec<KillMail> = Vec::new();

    while let Some(response) = api::gw::get_package("54689e7ff0b") {

        if let Some(content) = response.package {
            println!("{}", content.id);
            records.push(content.killmail);
        } else {
            if let Some(count) = flush(&records) {
                println!("Saved {} killmails", count);
                records.clear();
            }
            thread::sleep(std::time::Duration::from_secs(10));
        }
    }
}
