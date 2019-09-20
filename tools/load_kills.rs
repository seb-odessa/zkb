extern crate serde_json;
extern crate chrono;
extern crate hex;

use lib::curl;
use lib::database::*;
use lib::models::{Date, Kill};
use std::collections::HashMap;
use chrono::{Duration, TimeZone, Datelike, Utc};

const HISTORY_URL: &str = "https://zkillboard.com/api/history/";

fn query_history(year: i32, month: i32, day: i32) -> String {
    let url = format!("{}{}{:02}{:02}.json", HISTORY_URL, year, month, day);
    String::from_utf8_lossy(&curl::query(&url)).to_string()
}

fn load_day_kills(year: i32, month: i32, day: i32) -> usize {
    let date = Date::new(&year, &month, &day);
    let conn = establish_connection();
    let date_id = get_date_id(&conn, &date)
                    .or(insert_date(&conn, &date))
                    .expect(&format!("Failed to fine or create date record {:?}", date));

    let json = query_history(year, month, day);
    let map: HashMap<i32, String> = serde_json::from_str(&json).expect("Cant parse json");
    let mut kills = Vec::new();
    for (kill_id, kill_hash) in map.iter() {
        let hash = hex::decode(kill_hash).expect("Decoding failed");
        kills.push(Kill::new(kill_id, &hash, &date_id));        
    }    
    insert_kills(&conn, &kills).expect("Can't insert kills")
}

fn load_month_kills(year: i32, month: i32) -> usize {
    let mut total = 0;
    let mut date = Utc.ymd(year, month as u32, 1);
    let end = Utc.ymd(year, month as u32 + 1 , 1);
    while date < end {
        let kills = load_day_kills(year, month, date.day() as i32);
        println!("Loaded {} kill mails for {:}", kills, date);
        date = date + Duration::days(1);
        total = total + kills
    }
    return total;
}

fn load_year_kills(year: i32) -> usize {
    let mut total = 0;
    for month in 1..13 {
        let kills = load_month_kills(year, month);
        println!("\tLoaded {} kill mails for {}-{}", kills, year, month);
        total = total + kills
    }
    return total;
}


fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mut total_kills = 0;
    if 4 == args.len() {
        let year: i32 = args[1]
            .parse()
            .expect("Can't convert first argument to the Year");
        let month: i32 = args[2]
            .parse()
            .expect("Can't convert second argument to the Month");
        let day: i32 = args[3]
            .parse()
            .expect("Can't convert third argument to the Day number");
        total_kills = load_day_kills(year, month, day);
    } else if 3 == args.len() {
        let year: i32 = args[1]
            .parse()
            .expect("Can't convert first argument to the Year");
        let month: i32 = args[2]
            .parse()
            .expect("Can't convert second argument to the Month");
        total_kills = load_month_kills(year, month);
    } else if 2 == args.len() {
        let year: i32 = args[1]
            .parse()
            .expect("Can't convert first argument to the Year");
        total_kills = load_year_kills(year);
    } else {
        println!("Usage:");
        println!("\n\t {} YYYY MM DD", args[0]);
        println!("\n\t {} YYYY MM", args[0]);
        println!("\n\t {} YYYY", args[0]);
    }
    println!("Total loaded {} kill mails", total_kills);
}
