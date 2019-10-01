
use lib::api;
use lib::api::killmail::KillMail;
use lib::models::{DB, Hash};
use std::collections::HashMap;
use chrono::{Duration, TimeZone, Datelike, Utc, NaiveDate};
use std::io::Write;

use crossbeam_queue::SegQueue;
use crossbeam_utils::thread::scope;

#[derive(Debug, PartialEq, Clone)]
struct Id {
    id: i32,
    hash: Hash,
}

fn receiver(src: &SegQueue<Id>, dst: &SegQueue<KillMail>) {
    loop {
        if let Ok(id) = src.pop() {
            let response = api::gw::get_killamil(id.id, &id.hash);
            if let Some(killmail) = response {
                dst.push(killmail);
            } else {
                src.push(id);
            }
        }
        if src.is_empty() {
            break;
        }
    }
}

fn saver(src: &SegQueue<KillMail>, queue: &SegQueue<Id>, year: i32, month: u32, day: u32, start: usize, total: usize) {
    let conn = DB::connection();
    let mut counter = start;
    loop {
        if let Ok(killmail) = src.pop() {
            DB::save(&conn, &killmail).expect("Failed to save killmail into DB");
            counter = counter + 1;
            print!("{:4}-{:02}-{:02} Loading {:5}/{:5}\r", year, month, day, counter, total);
            std::io::stdout().flush().unwrap();
        }
        if queue.is_empty() {
            break;
        }
    }
}

fn load_day_kills(year: i32, month: u32, day: u32) -> usize {
    let json = api::gw::get_history(year, month, day);
    let map: HashMap<i32, String> = serde_json::from_str(&json).expect("Cant parse json");
    let done = DB::get_saved_killmails(&DB::connection(), &NaiveDate::from_ymd(year, month, day));
    let counter = done.len();
    let total = map.len();
    let rest = map.into_iter()
                  .filter(|row|{ !done.contains(&row.0)})
                  .map(|row| { Id{ id: row.0, hash: row.1.clone()} })
                  .collect::<Vec<Id>>();
    let todo = rest.len();

    let tasks = SegQueue::new();
    for id in rest.iter() {
        tasks.push(id.clone());
    }
    let results = SegQueue::new();
        scope(|scope| {
        scope.spawn(|_| receiver(&tasks, &results));
        scope.spawn(|_| receiver(&tasks, &results));
        scope.spawn(|_| receiver(&tasks, &results));
        scope.spawn(|_| receiver(&tasks, &results));
        scope.spawn(|_| saver(&results, &tasks, year, month, day, counter, total));
    })
    .unwrap();
    println!("");
    return todo;
}

fn load_month_kills(year: i32, month: u32) -> usize {
    let mut total = 0;
    let mut date = Utc.ymd(year, month, 1);
    while date.month() == month as u32 {
        let kills = load_day_kills(year, month, date.day());
        date = date + Duration::days(1);
        total = total + kills
    }
    println!("\tLoaded {} kill mails for {}-{:02}", total, year, month);
    return total;
}

fn load_year_kills(year: i32) -> usize {
    let mut total = 0;
    for month in 1..13 {
        let kills = load_month_kills(year, month);
        total = total + kills
    }
    println!("\tLoaded {} kill mails for {}", total, year);
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
    println!("Total loaded {} killmails", total_kills);
}
