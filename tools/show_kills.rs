extern crate diesel;
extern crate lib;

use lib::database::*;
/*
fn perform_action(year: i32, month: i32, day: i32) {
    let conn = establish_connection();
    let date = Date::new(&year, &month, &day);
    let id = date.load_id(&conn).expect("Failed to find records by date");
    let kills = get_kills(&conn, id).expect("Failed to query Kills");    
    for kill in kills {
        print!("{:?}\n", kill);
    }     
}
*/
fn main() {
    let args: Vec<_> = std::env::args().collect();

    if 4 != args.len() {
        println!("Usage:\n\t {} year month day", args[0]);
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
        //perform_action(year, month, day);
    }

}
