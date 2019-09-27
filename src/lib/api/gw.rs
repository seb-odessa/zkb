use curl::easy::Easy;
use super::killmail::KillMail;
use std::convert::TryFrom;

fn get(url: &str) -> Vec<u8> {
    let mut content = Vec::new();
    {
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


pub fn get_history(year: i32, month: u32, day: u32) -> String {
    let url = format!("https://zkillboard.com/api/history/{}{:02}{:02}.json", year, month, day);
    String::from_utf8_lossy(&get(&url)).to_string()
}

pub fn get_killamil(killmail_id: i32, hash: &str) -> Option<KillMail> {
    // https://esi.evetech.net/latest/killmails/78146996/4ceed992204ea5cab36f9543e80b90f0417534f5/?datasource=tranquility
    let url = format!("https://esi.evetech.net/latest/killmails/{}/{}/?datasource=tranquility", killmail_id, hash);
    let json = String::from_utf8_lossy(&get(&url)).to_string();
    let result = KillMail::try_from(json.clone());
    if result.is_err() {
        println!("{}", killmail_id);
        println!("{}", json);
    }
    result.ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_get_history() {
        let json = get_history(2019, 01, 01);
        let parsed = serde_json::from_str(&json);
        assert!(parsed.is_ok());
        let map: HashMap<i32, String> = parsed.unwrap();
        assert_eq!(15306, map.len());
        assert!(map.get(&74351681).is_some());
        assert_eq!("2627f994d452c5d87d1eb35b9978e8f81e7e9d31", map.get(&74351681).unwrap());
    }

    #[test]
    fn test_get_killamil() {
        let km = get_killamil(78146996, "4ceed992204ea5cab36f9543e80b90f0417534f5");
        assert!(km.is_some());
        let killamil = km.unwrap();
        assert_eq!(78146996, killamil.killmail_id);
        assert_eq!(30045352, killamil.solar_system_id);
    }
}