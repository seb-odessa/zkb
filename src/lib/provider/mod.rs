use crate::api::object::Object;
use crate::api::system::System;
use crate::api::stargate::Stargate;
use crate::api::character::Character;
use crate::api::region::Region;
use crate::api::constellation::Constellation;
use crate::api::alliance::Alliance;
use crate::api::corporation::Corporation;
use std::collections::HashMap;
use std::sync::Mutex;

mod prices;
pub use prices::get_avg_price;
pub use prices::get_adj_price;


#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
struct Route {
    departure: i32,
    destination: i32
}
impl Route {
    pub fn new(departure: i32, destination: i32) -> Self {
        Self{departure, destination}
    }
}


lazy_static! {
    static ref OBJECTS: Mutex<HashMap<i32, Object>> = Mutex::new(HashMap::new());
    static ref SYSTEMS: Mutex<HashMap<i32, System>> = Mutex::new(HashMap::new());
    static ref STARGATES: Mutex<HashMap<i32, Stargate>> = Mutex::new(HashMap::new());
    static ref CHARACTERS: Mutex<HashMap<i32, Character>> = Mutex::new(HashMap::new());
    static ref REGIONS: Mutex<HashMap<i32, Region>> = Mutex::new(HashMap::new());
    static ref CONSTELLATIONS: Mutex<HashMap<i32, Constellation>> = Mutex::new(HashMap::new());
    static ref ROUTES: Mutex<HashMap<Route, Vec<i32>>> = Mutex::new(HashMap::new());
    static ref ALLIANCES: Mutex<HashMap<i32, Alliance>> = Mutex::new(HashMap::new());
    static ref CORPORATIONS: Mutex<HashMap<i32, Corporation>> = Mutex::new(HashMap::new());
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
    let mut object = if let Ok(map) = CHARACTERS.try_lock() {
        map.get(key).cloned()
    } else {
        None
    };

    if object.is_none() {
        if let Some(received) = loader(key) {
            object = Some(received.clone());
            if let Ok(ref mut map) = CHARACTERS.try_lock() {
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
    let mut object = if let Ok(map) = CONSTELLATIONS.try_lock() {
        map.get(key).cloned()
    } else {
        None
    };

    if object.is_none() {
        if let Some(received) = loader(key) {
            object = Some(received.clone());
            if let Ok(ref mut map) = CONSTELLATIONS.try_lock() {
                map.entry(*key).or_insert(received);
            }
        }
    }
    return object;
}

pub fn get_region<L>(key: &i32, loader: &L) -> Option<Region>
    where
        L: Fn(&i32)->Option<Region>
{
    let mut object = if let Ok(map) = REGIONS.try_lock() {
        map.get(key).cloned()
    } else {
        None
    };

    if object.is_none() {
        if let Some(received) = loader(key) {
            object = Some(received.clone());
            if let Ok(ref mut map) = REGIONS.try_lock() {
                map.entry(*key).or_insert(received);
            }
        }
    }
    return object;
}

pub fn get_route<L>(src: &i32, dst: &i32, loader: &L) -> Option<Vec<i32>>
    where L: Fn(&i32, &i32)->Option<Vec<i32>>
{
    let key = Route::new(*src, *dst);
    let mut route = if let Ok(map) = ROUTES.try_lock() {
        map.get(&key).cloned()
    } else {
        None
    };

    if route.is_none() {
        if let Some(received) = loader(&src, &dst) {
            route = Some(received.clone());
            if let Ok(ref mut map) = ROUTES.try_lock() {
                map.entry(key).or_insert(received);
            }
        }
    }
    return route;
}

pub fn get_alliance<L>(key: &i32, loader: &L) -> Option<Alliance>
    where
        L: Fn(&i32)->Option<Alliance>
{
    let mut object = if let Ok(map) = ALLIANCES.try_lock() {
        map.get(key).cloned()
    } else {
        None
    };

    if object.is_none() {
        if let Some(received) = loader(key) {
            object = Some(received.clone());
            if let Ok(ref mut map) = ALLIANCES.try_lock() {
                map.entry(*key).or_insert(received);
            }
        }
    }
    return object;
}

pub fn get_corporation<L>(key: &i32, loader: &L) -> Option<Corporation>
    where
        L: Fn(&i32)->Option<Corporation>
{
    let mut object = if let Ok(map) = CORPORATIONS.try_lock() {
        map.get(key).cloned()
    } else {
        None
    };

    if object.is_none() {
        if let Some(received) = loader(key) {
            object = Some(received.clone());
            if let Ok(ref mut map) = CORPORATIONS.try_lock() {
                map.entry(*key).or_insert(received);
            }
        }
    }
    return object;
}