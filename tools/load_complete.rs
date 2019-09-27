
use lib::api;
use lib::models::{Connection, DB, Hash};
use std::collections::HashMap;
use chrono::{Duration, TimeZone, Datelike, Utc, NaiveDate};
use std::io::Write;

fn load_killmail(conn: &Connection, killmail_id: i32, killmail_hash: &Hash)->usize{
    let response = api::gw::get_killamil(killmail_id, killmail_hash);
    if let Some(killmail) = response {
        DB::save(&conn, &killmail).expect("Failed to save killmail into DB");
        1
    } else {
        0
    }
}

fn load_day_kills(year: i32, month: u32, day: u32) -> usize {
    let conn = DB::connection();
    let json = api::gw::get_history(year, month, day);
    let map: HashMap<i32, String> = serde_json::from_str(&json).expect("Cant parse json");
    let done = DB::get_saved_killmails(&conn, &NaiveDate::from_ymd(year, month, day));
    let mut counter = done.len();
    let total = map.len();
    std::io::stdout().flush().unwrap();
    print!("{:4}-{:02}-{:02}", year, month, day);
    for (killmail_id, killmail_hash) in map.iter() {
        if !done.contains(killmail_id) {
            counter = counter + load_killmail(&conn, *killmail_id, killmail_hash);
        } else {
            counter = counter + 1;
        }
        print!("\r{:4}-{:02}-{:02} Loading {:5}/{:5}", year, month, day, counter, total);
        std::io::stdout().flush().unwrap();

    }
    println!(". Done.");
    return counter;
}

fn load_month_kills(year: i32, month: u32) -> usize {
    let mut total = 0;
    let mut date = Utc.ymd(year, month, 1);
    while date.month() == month as u32 {
        let kills = load_day_kills(year, month, date.day());
        date = date + Duration::days(1);
        total = total + kills
    }
    return total;
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
    println!("Total loaded {} killmails", total_kills);
}
