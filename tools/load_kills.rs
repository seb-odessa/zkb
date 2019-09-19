extern crate serde_json;

use lib::curl;
use lib::database::*;
use lib::models::{Date, Kill};
use std::collections::HashMap;

const HISTORY_URL: &str = "https://zkillboard.com/api/history/";

fn query_history(year: i32, month: i32, day: i32) -> String {
    let url = format!("{}{}{:02}{:02}.json", HISTORY_URL, year, month, day);
    String::from_utf8_lossy(&curl::query(&url)).to_string()
}

fn load_kills(year: i32, month: i32, day: i32) -> usize {
    let date = Date::new(&year, &month, &day);
    let conn = establish_connection();
    let date_id = get_date_id(&conn, &date)
                    .or(insert_date(&conn, &date))
                    .expect(&format!("Failed to fine or create date record {:?}", date));

    let json = query_history(year, month, day);
    let map: HashMap<i32, String> = serde_json::from_str(&json).expect("Cant parse json");
    let mut kills = Vec::new();
    for (kill_id, kill_hash) in map.iter() {
        kills.push(Kill::new(kill_id, kill_hash, &date_id));        
    }    
    insert_kills(&conn, &kills).expect("Can't insert kills")
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if 4 != args.len() {
        println!("Usage:\n\t {} YYYY MM DD", args[0]);
    } else {
        let year: i32 = args[1]
            .parse()
            .expect("Can't convert first argument to the Year");
        let month: i32 = args[2]
            .parse()
            .expect("Can't convert second argument to the Month");
        let day: i32 = args[3]
            .parse()
            .expect("Can't convert third argument to the Day number");
        let r = load_kills(year, month, day);
        println!("Loaded {} kill mails", r);
    }
}
