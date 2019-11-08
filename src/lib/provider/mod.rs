use crate::api::object::Object;
use crate::api::system::System;
use crate::api::stargate::Stargate;
use crate::api::character::Character;
use crate::api::constellation::Constellation;
use std::collections::HashMap;

use std::sync::Mutex;

mod prices;

pub use prices::get_avg_price;
pub use prices::get_adj_price;


lazy_static! {
    static ref OBJECTS: Mutex<HashMap<i32, Object>> = Mutex::new(HashMap::new());
    static ref SYSTEMS: Mutex<HashMap<i32, System>> = Mutex::new(HashMap::new());
    static ref STARGATES: Mutex<HashMap<i32, Stargate>> = Mutex::new(HashMap::new());
    static ref CHARACTER: Mutex<HashMap<i32, Character>> = Mutex::new(HashMap::new());
    static ref CONSTELLATION: Mutex<HashMap<i32, Constellation>> = Mutex::new(HashMap::new());
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

pub fn get_stargate<L>(key: &i32, loader: &L) -> Option<Stargate>
    where L: Fn(&i32)->Option<Stargate>
{
    let mut object = if let Ok(map) = STARGATES.try_lock() {
        map.get(key).cloned()
    } else {
        None
    };

    if object.is_none() {
        if let Some(received) = loader(key) {
            object = Some(received.clone());
            if let Ok(ref mut map) = STARGATES.try_lock() {
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
    let mut object = if let Ok(map) = SYSTEMS.try_lock() {
        map.get(key).cloned()
    } else {
        None
    };

    if object.is_none() {
        if let Some(received) = loader(key) {
            object = Some(received.clone());
            if let Ok(ref mut map) = SYSTEMS.try_lock() {
                map.entry(*key).or_insert(received);
            }
        }
    }
    return object;
}

pub fn get_character<L>(key: &i32, loader: &L) -> Option<Character>
    where
        L: Fn(&i32)->Option<Character>
{
    let mut object = if let Ok(map) = CHARACTER.try_lock() {
        map.get(key).cloned()
    } else {
        None
    };

    if object.is_none() {
        if let Some(received) = loader(key) {
            object = Some(received.clone());
            if let Ok(ref mut map) = CHARACTER.try_lock() {
                map.entry(*key).or_insert(received);
            }
        }
    }
    return object;
}


pub fn get_constellation<L>(key: &i32, loader: &L) -> Option<Constellation>
    where
        L: Fn(&i32)->Option<Constellation>
{
    let mut object = if let Ok(map) = CONSTELLATION.try_lock() {
        map.get(key).cloned()
    } else {
        None
    };

    if object.is_none() {
        if let Some(received) = loader(key) {
            object = Some(received.clone());
            if let Ok(ref mut map) = CONSTELLATION.try_lock() {
                map.entry(*key).or_insert(received);
            }
        }
    }
    return object;
}

