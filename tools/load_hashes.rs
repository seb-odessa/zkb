extern crate serde_json;

use lib::api;
use lib::models::{DB, Connection};
use lib::models::kill::Kill;
use std::collections::HashMap;
use chrono::{Duration, TimeZone, Datelike, Utc, NaiveDate};
use std::io::Write;

use crossbeam::atomic::AtomicCell;
use crossbeam_queue::SegQueue;
use crossbeam_utils::thread::scope;



fn receiver(src: &SegQueue<NaiveDate>, dst: &SegQueue<Kill>) {
    loop {
        if let Ok(date) = src.pop() {
            let json = api::gw::get_history(date.year(), date.month(), date.day());
            let pairs: Option<HashMap<i32, String>> = serde_json::from_str(&json).ok();
            if let Some(map) = pairs {
                for (id, hash) in map.iter() {
                    dst.push(Kill::new(id, hash, &date));
                }
            } else {
//                thread::sleep(std::time::Duration::from_millis(600));
                src.push(date);
            }
        }
        if src.is_empty() {
            break;
        }
    }
}

fn flush(conn: &Connection, records: &mut Vec<Kill>) -> usize {
    DB::save_kills(&conn, &records).expect("Can't insert kills");
    let count = records.len();
    records.clear();
    return count;
}

fn saver(src: &SegQueue<Kill>, queue: &SegQueue<NaiveDate>, counter: &AtomicCell<usize>) {
    let conn = DB::connection();
    let mut records = Vec::new();
    let mut count = 0;
    loop {
        if let Ok(kill) = src.pop() {
            records.push(kill);
            if records.len() >= 1000 {
                count = count + flush(&conn, &mut records);
                print!("Loaded {:5} records\r", count);
                std::io::stdout().flush().unwrap_or_default();
                counter.store(count);
            }
            if queue.is_empty() && src.is_empty() {
                break;
            }
        }
    }
    count = count + flush(&conn, &mut records);
    print!("Loaded {:5} records\r", count);
    std::io::stdout().flush().unwrap_or_default();
    counter.store(count);
}

fn load_day_kills(year: i32, month: u32, day: u32) -> usize {
    let conn = DB::connection();

    let json = api::gw::get_history(year, month, day);
    let map: HashMap<i32, String> = serde_json::from_str(&json).expect("Cant parse json");
    let mut kills = Vec::new();
    for (kill_id, kill_hash) in map.iter() {
        let date = NaiveDate::from_ymd(year, month, day);
        kills.push(Kill::new(kill_id, kill_hash, &date));
    }
    DB::save_kills(&conn, &kills).expect("Can't insert kills")
}

fn load_month_kills(year: i32, month: u32) -> usize {
    let mut date = Utc.ymd(year, month, 1);
    let tasks = SegQueue::new();
    while date.month() == month as u32 {
        tasks.push(NaiveDate::from_ymd(year, month, date.day()));
        date = date + Duration::days(1);
    }

    let results = SegQueue::new();
    let total = AtomicCell::new(0);
    scope(|scope| {
        scope.spawn(|_| receiver(&tasks, &results));
        scope.spawn(|_| receiver(&tasks, &results));
        // scope.spawn(|_| receiver(&tasks, &results));
        // scope.spawn(|_| receiver(&tasks, &results));
        scope.spawn(|_| saver(&results, &tasks, &total));
    })
    .unwrap();
    return total.into_inner();
}

fn load_year_kills(year: i32) -> usize {
    let mut total = 0;
    for month in 1..13 {
        let kills = load_month_kills(year, month);
        println!("\tLoaded {} kill mails for {}-{:02}", kills, year, month);
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
        let month: u32 = args[2]
            .parse()
            .expect("Can't convert second argument to the Month number");
        let day: u32 = args[3]
            .parse()
            .expect("Can't convert third argument to the Day number");
        total_kills = load_day_kills(year, month, day);
    } else if 3 == args.len() {
        let year: i32 = args[1]
            .parse()
            .expect("Can't convert first argument to the Year");
        let month: u32 = args[2]
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
