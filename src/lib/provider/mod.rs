use crate::api::object::Object;
use crate::api::system::System;
use crate::api::character::Character;
use crate::api::constellation::Constellation;
use std::collections::HashMap;

use std::sync::Mutex;

mod prices;
mod cache;

pub use prices::get_avg_price;
pub use prices::get_adj_price;


lazy_static! {
    static ref OBJECTS: Mutex<HashMap<i32, Object>> = Mutex::new(HashMap::new());
    static ref SYSTEMS: Mutex<cache::Cache<i32, System>> = Mutex::new(cache::Cache::new());
    static ref CHARACTER: Mutex<cache::Cache<i32, Character>> = Mutex::new(cache::Cache::new());
    static ref CONSTELLATION: Mutex<cache::Cache<i32, Constellation>> = Mutex::new(cache::Cache::new());
}

pub fn get_object<L>(key: &i32, loader: &L) -> Option<Object>
    where L: Fn(&i32)->Option<Object>
{
    let mut object = if let Ok(map) = OBJECTS.try_lock() {
        map.get(key).cloned()
    } else {
        None
    };

    if object.is_none() {
        if let Some(received) = loader(key) {
            object = Some(received.clone());
            if let Ok(ref mut map) = OBJECTS.try_lock() {
                map.entry(*key).or_insert(received);
            }
        }
    }

    return object;
}

pub fn get_system<L>(key: &i32, loader: &L) -> Option<System>
    where
        L: Fn(&i32)->Option<System>
{
    if let Ok(ref mut cache) = SYSTEMS.try_lock() {
        return cache.get(key, loader).cloned()
    } else {
        None
    }
}

pub fn get_character<L>(key: &i32, loader: &L) -> Option<Character>
    where
        L: Fn(&i32)->Option<Character>
{
    if let Ok(ref mut cache) = CHARACTER.try_lock() {
        return cache.get(key, loader).cloned()
    } else {
        None
    }
}


pub fn get_constellation<L>(key: &i32, loader: &L) -> Option<Constellation>
    where
        L: Fn(&i32)->Option<Constellation>
{
    if let Ok(ref mut cache) = CONSTELLATION.try_lock() {
        return cache.get(key, loader).cloned()
    } else {
        None
    }
}

