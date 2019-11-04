#[macro_use]
extern crate log;

use lib::api;
use lib::api::killmail::Killmail;
use lib::models::{DB, KillmailsApi};
use std::thread;
use std::collections::HashMap;

fn flush(records: &Vec<Killmail>) -> Option<usize> {
    let conn = DB::connection();
    let mut map = HashMap::new();
    for km in records {
        if !KillmailsApi::exist(&conn, km.killmail_id) {
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

fn run_updater(id: String, timeout: u32) {
    let mut records: Vec<Killmail> = Vec::new();
    loop {
        while let Some(response) = api::gw::get_package(&id) {
            if let Some(content) = response.content {
                println!("{} {} {}", content.id, content.zkb.npc, content.zkb.href);
                records.push(content.killmail);
            } else if records.len() > 0 {
                if let Some(count) = flush(&records) {
                    println!("Saved {} killmails", count);
                    records.clear();
                }
            }
        }
        println!("Nothing to receive. Will wait {} sec", timeout);
        thread::sleep(std::time::Duration::from_secs(timeout.into()));
    }
}

fn main() {
    env_logger::init();
    let args: Vec<_> = std::env::args().collect();
    if 3 == args.len() {
        let id: String = args[1]
            .parse()
            .expect("Can't convert first argument to the request id");
        let timeout: u32 = args[2]
            .parse()
            .expect("Can't convert second argument to the timeout");
        run_updater(id , timeout);
    } else {
        println!("Usage:");
        println!("\n\t {} id timeout", args[0]);
    }
}
