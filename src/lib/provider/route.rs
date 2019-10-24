use crate::api;
use std::sync::Mutex;
use std::collections::HashMap;

pub type Ids = Vec<i32>;

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

struct Routes {
    routes: HashMap<Route, Ids>,
}
impl Routes {
    pub fn new() -> Self {
        Self {routes: HashMap::new() }
    }

    fn new_route(route: &Route) -> Ids {
        let uri = format!("route/{}/{}", route.departure, route.destination);
        let response = api::gw::eve_api(&uri).unwrap_or_default();
        serde_json::from_str(&response).unwrap_or_default()
    }

    pub fn get(&mut self, src: i32, dst: i32) -> Ids {

        let id = Route::new(src, dst);
        self.routes.entry(id.clone()).or_insert(Self::new_route(&id)).to_vec()
    }

    pub fn get_cache_count(&self) -> usize {
        self.routes.len()
    }
}

lazy_static! {
    static ref ROUTES: Mutex<Routes> = Mutex::new(Routes::new());
}

pub fn get_route(departure: i32, destination: i32) -> Ids {
    if let Ok(ref mut routes) = ROUTES.try_lock() {
        routes.get(departure, destination)
    } else {
        warn!("Can't lock ROUTE for update");
        Ids::new()
    }
}

pub fn get_cached_route_count() -> Option<usize> {
    if let Ok(ref names) = ROUTES.try_lock() {
        return Some(names.get_cache_count())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn from_get_route() {
        const HEK_ID: i32 = 30002053;
        const JITA_ID: i32 = 30000142;
        let jita_hek = vec![30000142, 30000144, 30002642, 30002643, 30002644, 30002691, 30002718, 30002719, 30002723, 30002053];

        assert_eq!(Some(0), super::get_cached_route_count());
        assert_eq!(super::get_route(JITA_ID, HEK_ID), jita_hek.clone());
        assert_eq!(Some(1), super::get_cached_route_count());
        assert_eq!(super::get_route(HEK_ID, JITA_ID), jita_hek.clone().iter().rev().cloned().collect::<Vec<i32>>());
        assert_eq!(Some(2), super::get_cached_route_count());
        assert_eq!(super::get_route(JITA_ID, HEK_ID), jita_hek.clone());
        assert_eq!(Some(2), super::get_cached_route_count());
    }
}