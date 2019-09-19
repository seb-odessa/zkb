use curl::easy::Easy;

pub fn query(url: &str) -> Vec<u8> {
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

