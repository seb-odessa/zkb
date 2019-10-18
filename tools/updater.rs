#[macro_use]
extern crate log;

use lib::api;
use lib::api::killmail::KillMail;
use lib::models::DB;
use std::thread;
use std::collections::HashMap;

fn flush(records: &Vec<KillMail>) -> Option<usize> {
    let conn = DB::connection();
    let mut map = HashMap::new();
    for km in records {
        if !DB::exists(&conn, km.killmail_id) {
            map.insert(km.killmail_id, km.clone());
        }
    }

    match DB::save_all(&conn, &map.values().map(|kill| kill.clone() ).collect()) {
        Ok(()) => {
            Some(map.len())
        },
        Err(err) => {
            error!("Failed to save to DB {:?}", err);
            None
        }
    }
}

fn run_updater(timeout: u32) {
    let mut records: Vec<KillMail> = Vec::new();
    while let Some(response) = api::gw::get_package("54689e7ff0b3cebfa1356bfbc9c7682c") {
        info!("Received response from API");
        if let Some(content) = response.package {
            println!("{} {} {}", content.id, content.zkb.npc, content.zkb.href);
            records.push(content.killmail);
        } else if records.len() > 0 {
            if let Some(count) = flush(&records) {
                println!("Saved {} killmails", count);
                records.clear();
            }
            thread::sleep(std::time::Duration::from_secs(timeout.into()));
        }
    }
}

fn main() {
    env_logger::init();
    let args: Vec<_> = std::env::args().collect();
    if 2 == args.len() {
        let timeout: u32 = args[1]
            .parse()
            .expect("Can't convert first argument to the timeout");
        run_updater(timeout);
    } else {
        println!("Usage:");
        println!("\n\t {} timeout", args[0]);
    }
}
