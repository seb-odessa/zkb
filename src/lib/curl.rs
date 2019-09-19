use curl::easy::Easy;



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curl() {
        let mut content = Vec::new();
        let mut easy = Easy::new();
        assert!(easy.url("https://example.com/").is_ok());
        let mut transfer = easy.transfer();
        let done = transfer.write_function(|data| {
            content.extend_from_slice(data);
            Ok(data.len())
        });
        assert!(done.is_ok());
        assert!(transfer.perform().is_ok());
    }
}
