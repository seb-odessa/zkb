extern crate lib;
extern crate diesel;

fn main() {

    let args: Vec<_> = std::env::args().collect();

    if 4 != args.len() {
        println!("Usage:\n\t {} YYYY MM DD", args[0]);
    } else {
        use lib::database::*;
        let year:i32 = args[1].parse().expect("Can't convert first argument to the Year");
        let month:i32 = args[2].parse().expect("Can't convert second argument to the Month");
        let day:i32 = args[3].parse().expect("Can't convert third argument to the Day number");
        let conn = establish_connection();
        let r = insert_date(&conn, year, month, day);
        println!("Res = {:?}", r);

    }
}
