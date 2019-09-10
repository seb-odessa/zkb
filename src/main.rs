

use serde::{Deserialize, Serialize};
use serde_json::json;

use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
//#[serde(rename_all(serialize = "camelCase"))]
//#[serde(rename_all(deserialize = "snake_case"))]
struct Zkb {
    #[serde(rename = "locationID")]
    #[serde(default)]
    location_id: u32,
    hash: String,
    #[serde(rename = "fittedValue")]
    fitted_value: f32,
    #[serde(rename = "totalValue")]
    total_value: f32,
    points: u16,
    npc: bool,
    solo: bool,
    awox: bool
}

#[derive(Serialize, Deserialize, Debug)]
struct Record {
    killmail_id: u32,
    zkb: Zkb
}



// curl https://esi.evetech.net/latest/killmails/78560358/7a942007d806fb9723c7b7234dab0e1045b5f59b/?datasource=tranquility  

fn main() {
    let record = json!({
        
            "78146996":"4ceed992204ea5cab36f9543e80b90f0417534f5",
            "78146999":"f22a5166bfc52151c029cc169d9e0c289c439233",
            "78147000":"34177ddc51664e50e2c6f7ef91f9e8a75f7addc1"

        // "killmail_id":78766279,
        // "zkb":{
        //     "locationID":40152304,
        //     "hash":"05b689f860cd720cf3c8f71ab4c5100aff396081",
        //     "fittedValue":43913214.04,
        //     "totalValue":102163132.51,
        //     "points":1,
        //     "npc":true,
        //     "solo":false,
        //     "awox":false}
        });
    


    let s = serde_json::to_string(&record).unwrap();
    println!("Record\n {}", &s);

    let restored: HashMap<u64, String> = serde_json::from_str(&s).unwrap();
    println!("Record\n {:?}", &restored);


    // let restored: Record = serde_json::from_str(&s).unwrap();
    // println!("Record\n {:?}", &restored);

    // let mut v: Vec<Record> = Vec::new();
    // v.push(serde_json::from_str(&s).unwrap());
    // v.push(serde_json::from_str(&s).unwrap());

    // let ss = serde_json::to_string(&v).unwrap();
    // println!("Record\n {}", &ss);



    // let stdin = std::io::stdin();
    // let array: Vec<Record> = serde_json::from_reader(stdin).unwrap();
    // println!("Done. Readed {}", array.len());
}
