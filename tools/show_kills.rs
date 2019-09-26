extern crate diesel;
extern crate lib;
use chrono::NaiveDate;
use lib::models::DB;

fn perform_action(year: i32, month: u32, day: u32) {
    let conn = DB::connection();
    let date = NaiveDate::from_ymd(year, month, day);
    let kills = DB::load_kills(&conn, &date).expect("Failed to query Kills");    
    for kill in kills {
        print!("{:?}\n", kill);
    }     
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if 4 != args.len() {
        println!("Usage:\n\t {} year month day", args[0]);
    } else {
        let year: i32 = args[1]
            .parse()
            .expect("Can't convert first argument to the Year");
        let month: u32 = args[2]
            .parse()
            .expect("Can't convert second argument to the Month");
        let day: u32 = args[3]
            .parse()
            .expect("Can't convert third argument to the Day");
        perform_action(year, month, day);
    }

}
