use crate::api::price::Prices;

use std::sync::Mutex;

lazy_static! {
    static ref PRICES: Mutex<Prices> = Mutex::new(Prices::new());
}

pub fn get_avg_price(id: &Option<i32>) -> Option<f32> {
    if let Some(id) = id {
        if let Ok(ref prices) = PRICES.try_lock() {
            return prices.avg(*id)
        }
    }
    return None
}

pub fn get_adj_price(id: &Option<i32>) -> Option<f32> {
    if let Some(id) = id {
        if let Ok(ref prices) = PRICES.try_lock() {
            return prices.adj(*id)
        }
    }
    return None
}


#[cfg(test)]
mod tests {

    #[test]
    fn from_get_prices() {
        assert!(super::get_adj_price(&Some(3178)).is_some());
        assert!(super::get_adj_price(&Some(3178)).is_some());
    }
}