extern crate serde;
extern crate serde_json;

use lib::database::*;
use std::collections::HashMap;


const HISTORY_URL: &str = "https://zkillboard.com/api/history/";

fn query(url: &str) -> Vec<u8>
{
    let mut content = Vec::new();
    {
        use curl::easy::Easy;
        let mut easy = Easy::new();
        easy.url(url).expect(&format!("Can't open {}", url));
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            content.extend_from_slice(data);
            Ok(data.len())
        }).expect("Can't receive data from server");
        transfer.perform().expect("Can't complete request");
    }
    return content;
}

fn query_history(year: i32, month: i32, day: i32) -> String {
    let url = format!("{}{}{:02}{:02}.json", HISTORY_URL, year, month, day);
    println!("Res = {:?}", url);
    String::from_utf8_lossy(&query(&url)).to_string()
}

fn load_kills(year: i32, month: i32, day: i32) -> usize {
    use lib::models::NewKill;

    let conn = establish_connection();
    let date_id = get_date_id(&conn, year, month, day)
                    .or(insert_date(&conn, year, month, day))
                    .expect(&format!("Failed to fine or create date record < {} {} {} >", year, month, day));
    println!("date_id = {}", date_id);
    let json = query_history(year, month, day);
    let map: HashMap<i32, String> = serde_json::from_str(&json).expect("Cant parse json");
    let mut kills = Vec::new();
    for (kill_id, kill_hash) in map.iter() {
        kills.push(NewKill::new(kill_id, kill_hash, &date_id));        
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
        println!("Res = {:?}", r);
    }

}
