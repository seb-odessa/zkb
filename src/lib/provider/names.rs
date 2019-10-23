use crate::api::object::Object;

use std::sync::Mutex;
use std::convert::TryFrom;
use std::collections::HashMap;

struct Names {
    names: HashMap<i32, String>,
}
impl Names {
    pub fn new() -> Self {
        Self {names: HashMap::new() }
    }

    pub fn get(&mut self, id: i32) -> String {
        let count = self.get_cache_count();
        self.names.entry(id).or_insert(Object::try_from(id).ok().unwrap_or_default().name);
        if self.get_cache_count() != count {
            info!("Name provider contains {} cached names", self.get_cache_count());
        }
        self.names.get(&id).map(|name| name.clone()).unwrap_or_default()
    }

    pub fn get_cache_count(&self) -> usize {
        self.names.len()
    }
}

lazy_static! {
    static ref NAMES: Mutex<Names> = Mutex::new(Names::new());
}

pub fn get_name(id: &Option<i32>) -> String {
    if let Some(id) = id {
        if let Ok(ref mut names) = NAMES.try_lock() {
            return names.get(*id)
        }
    }
    return String::new()
}

pub fn get_cached_names_count() -> Option<usize> {
    if let Ok(ref names) = NAMES.try_lock() {
            return Some(names.get_cache_count())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn from_get_name() {
        assert_eq!(Some(0), super::get_cached_names_count());
        assert_eq!(super::get_name(&Some(2114350216)), "Seb Odessa");
        assert_eq!(Some(1), super::get_cached_names_count());
        assert_eq!(super::get_name(&Some(3178)), "Light Neutron Blaster II");
        assert_eq!(Some(2), super::get_cached_names_count());
        assert_eq!(super::get_name(&Some(2114350216)), "Seb Odessa");
        assert_eq!(Some(2), super::get_cached_names_count());
    }
}