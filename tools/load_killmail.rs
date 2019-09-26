extern crate serde_json;

use lib::api;
use lib::models::Connection;
use lib::models::DB;
use lib::models::Hash;
use lib::models::kill::Kill;
use std::collections::HashMap;
use chrono::{Duration, TimeZone, Datelike, Utc, NaiveDate};

fn load_killmail(conn: &Connection, killmail_id: i32, killmail_hash: &Hash){
    let killmail = api::gw::get_killamil(killmail_id, killmail_hash).expect("Failed to query Killmail from API");
    print!("{:?}\n", &killmail);
    DB::save(&conn, &killmail).expect("Failed to save killmail into DB");
}

fn perform_action(killmail_id: i32) {
    let conn = DB::connection();
    let kill = Kill::load(&conn, killmail_id).expect("Filed to query hash for killmail id");    
    print!("{:?}\n", &kill);    
    load_killmail(&conn, killmail_id, &kill.killmail_hash);
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
     if 2 == args.len() {
        let killmail_id: i32 = args[1]
            .parse()
            .expect("Can't convert first argument to the killmail_id");
        perform_action(killmail_id);
    } else {
        println!("Usage:");
        println!("\n\t {} killmail_id", args[0]);
    }
}
