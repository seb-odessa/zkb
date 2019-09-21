extern crate chrono;

use chrono::{Duration, TimeZone, Utc};

fn main() {
    
    let mut date = Utc.ymd(2018, 11, 1);
    let end = Utc.ymd(2019, 12, 1);
    while date < end {
        println!(" {:}", &date);
        date = date + Duration::days(1);
//        Duration::days(1);
    }
}
