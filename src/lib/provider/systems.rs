use crate::api::system::System;

use std::sync::Mutex;
use std::collections::HashMap;

struct Cache<T> {
    cache: HashMap<i32, Option<T>>,
}
impl<T> Cache<T> {
    pub fn new() -> Self {
        Self {cache: HashMap::new() }
    }

    pub fn get<G>(&mut self, id: i32, getter: &G) -> Option<&T>
        where G: Fn(i32)->Option<T>
    {
        self.cache.entry(id).or_insert(getter(id)).as_ref()
    }

    pub fn get_cache_count(&self) -> usize {
        self.cache.len()
    }
}

lazy_static! {
    static ref SYSTEMS: Mutex<Cache<System>> = Mutex::new(Cache::new());
}

pub fn get_system(id: &i32) -> Option<System> {
    if let Ok(ref mut cache) = SYSTEMS.try_lock() {
        return cache.get(*id, &System::new).cloned();
    }
    return None;
}

pub fn get_cached_systems_count() -> Option<usize> {
    if let Ok(ref cache) = SYSTEMS.try_lock() {
            return Some(cache.get_cache_count())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn from_get_name() {
        assert_eq!(Some(0), super::get_cached_systems_count());
        assert_eq!("Jita", super::get_system(&30000142).map(|x| x.name).unwrap_or_default());
        assert_eq!(Some(1), super::get_cached_systems_count());
        assert_eq!("Hek", super::get_system(&30002053).map(|x| x.name).unwrap_or_default());
        assert_eq!(Some(2), super::get_cached_systems_count());
        assert_eq!("Jita", super::get_system(&30000142).map(|x| x.name).unwrap_or_default());
        assert_eq!(Some(2), super::get_cached_systems_count());
    }
}