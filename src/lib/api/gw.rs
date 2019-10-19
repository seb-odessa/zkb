use curl::easy::Easy;
use curl::Error;
use super::killmail::KillMail;
use super::zkb::Package;
use std::convert::TryFrom;

//https://esi.evetech.net/latest/swagger.json
//https://esi.evetech.net/latest/characters/2114350216/?datasource=tranquility
//https://esi.evetech.net/latest/universe/systems/30002659/?datasource=tranquility&language=en-us
//https://esi.evetech.net/latest/killmails/78146996/4ceed992204ea5cab36f954380b90f0417534f5/?datasource=tranquility

pub const EVE_API: &str = "https://esi.evetech.net/latest";
pub const EVE_SRV: &str = "?datasource=tranquility";

fn get(url: &str) -> Result<Vec<u8>, Error> {
    let mut easy = Easy::new();
    easy.accept_encoding("gzip")?;
    easy.useragent("Easy API, Maintainer: seb@ukr.net")?;
    easy.url(url)?;
    let mut content = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {content.extend_from_slice(data); Ok(data.len())})?;
        transfer.perform()?;
    }
    return Ok(content);
}

fn post(url: &str, request: &str) -> Result<Vec<u8>, Error> {
    let mut easy = Easy::new();
    easy.accept_encoding("gzip")?;
    easy.useragent("Easy API, Maintainer: seb@ukr.net")?;
    easy.url(url)?;
    easy.post_fields_copy(request.as_bytes())?;
    let mut content = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {content.extend_from_slice(data); Ok(data.len())})?;
        transfer.perform()?;
    }
    return Ok(content);
}

pub fn evetech(cmd: &str) -> Option<String> {
    let url = format!("{}/{}/{}", EVE_API, cmd, EVE_SRV);
    if let Some(response) = get(&url).ok() {
        String::from_utf8(response).ok()
    } else {
        None
    }
}

pub fn evetech_post(cmd: &str, request: &str) -> Option<String> {
    let url = format!("{}/{}/{}", EVE_API, cmd, EVE_SRV);
    if let Some(response) = post(&url, &request).ok() {
        String::from_utf8(response).ok()
    } else {
        None
    }
}

pub fn get_history(year: i32, month: u32, day: u32) -> String {
    let url = format!("https://zkillboard.com/api/history/{}{:02}{:02}.json", year, month, day);
    if let Some(response) = get(&url).ok() {
        String::from_utf8_lossy(&response).to_string()
    } else {
        String::new()
    }
}

pub fn get_killamil(killmail_id: i32, hash: &str) -> Option<KillMail> {
    // https://esi.evetech.net/latest/killmails/78146996/4ceed992204ea5cab36f9543e80b90f0417534f5/?datasource=tranquility
    let url = format!("https://esi.evetech.net/latest/killmails/{}/{}/?datasource=tranquility", killmail_id, hash);
    if let Some(response) = get(&url).ok() {
        let json = String::from_utf8_lossy(&response).to_string();
        KillMail::try_from(json).ok()
    } else {
        None
    }
}

pub fn get_package(queue_id: &str) -> Option<Package> {
    // https://redisq.zkillboard.com/listen.php?queueID=54689e7ff0b3cebfa1356bfbc9c7682c

    let url = format!("https://redisq.zkillboard.com/listen.php?queueID={}", queue_id);
    if let Some(response) = get(&url).ok() {
        let json = String::from_utf8_lossy(&response).to_string();
        Package::try_from(json).ok()
    } else {
        None
    }
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