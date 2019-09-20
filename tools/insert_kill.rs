extern crate diesel;
extern crate hex;
extern crate lib;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if 4 != args.len() {
        println!("Usage:\n\t {} kill_id kill_hash date_id", args[0]);
    } else {
        use lib::database::*;
        let id: i32 = args[1]
            .parse()
            .expect("Can't convert first argument to the Year");
        let hash_str: String = args[2]
            .parse()
            .expect("Can't convert second argument to the Hash");
        let date_id: i32 = args[1]
            .parse()
            .expect("Can't convert first argument to the Year");
        let conn = establish_connection();
        
        let hash = hex::decode(hash_str).expect("Decoding failed");
        let r = insert_kill(&conn, &lib::models::Kill::new(&id, &hash, &date_id));
        println!("Res = {:?}", r);
    }
}
