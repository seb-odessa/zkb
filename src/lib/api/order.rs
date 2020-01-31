use crate::api::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
pub enum OrderType{
    ALL,
    BUY,
    SELL
}

pub type Orders = Vec<Order>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Order {
    pub order_id: LongRequired,
    pub type_id: IntRequired,
    pub location_id: LongRequired,
    pub system_id: IntRequired,
    pub volume_total: IntRequired,
    pub volume_remain: IntRequired,
    pub min_volume: IntRequired,
    pub price: FloatRequired,
    pub is_buy_order: BoolRequired,
    pub duration: IntRequired,
    pub issued: TimeRequired,
    pub range: StrRequired
}
impl Order {
    fn load_page(region_id: &i32, order_type: &OrderType, page: &IntRequired, type_id: &IntOptional) -> Option<Orders> {
        let order_type_name = match order_type {
            OrderType::ALL => "all",
            OrderType::BUY => "buy",
            OrderType::SELL => "sell"
        };
        let filter = if let Some(type_id) = type_id {
            format!("order_type={}&page={}&type_id={}", order_type_name, page, type_id)
        } else {
            format!("order_type={}&page={}", order_type_name, page)
        };
        let response = gw::eve_api_ex(&format!("markets/{}/orders", region_id), &filter).unwrap_or_default();
        serde_json::from_str(&response).ok()
    }

    pub fn load(region_id: &i32, order_type: OrderType, type_id: IntOptional) -> Orders {
        let mut result = Vec::new();
        let mut page = 1;
        const API_MAX_ITEMS: usize = 1000;
        while let Some(mut records) = Self::load_page(region_id, &order_type, &page, &type_id) {
            let count = records.len();
            if count > 0 {
                result.append(&mut records);
                page = page + 1;
            }
            if count == 0 || count < API_MAX_ITEMS { // all records received
                break;
            }
        }
        return result;
    }

    pub fn load_for_system(system_id: &i32, order_type: OrderType, type_id: IntOptional) -> Orders {
        let mut orders = Vec::new();
        if let Some(system) = system::System::new(system_id) {
            if let Some(region_id) = system.get_region_id() {
                let mut filtered: Orders = Self::load(&region_id, order_type, type_id)
                            .iter()
                            .filter(|order| order.system_id == system.system_id)
                            .cloned()
                            .collect();
                orders.append(&mut filtered)
            }
        }
        return orders;
    }


    pub fn load_nth_best_sell_orders(region_id: &i32, type_id: IntRequired, count: usize) -> Orders {
        use std::cmp::Ordering;
        let mut orders = Self::load(region_id, OrderType::SELL, Some(type_id));
        orders.as_mut_slice().sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal));
        orders.iter().take(count).cloned().collect()
    }

    pub fn load_nth_best_buy_orders(region_id: &i32, type_id: IntRequired, count: usize) -> Orders {
        use std::cmp::Ordering;
        let mut orders = Self::load(region_id, OrderType::BUY, Some(type_id));
        orders.as_mut_slice().sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap_or(Ordering::Equal));
        orders.iter().take(count).cloned().collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    const REGION_ID: IntRequired = 10000002; // The Forge
    const PLEX_ID: IntRequired = 44992; // PLEX

    #[test]
    fn from_api_load_all() {
        let orders = Order::load(&REGION_ID, OrderType::ALL, Some(PLEX_ID));
        assert!(!orders.is_empty());
        assert!(orders.iter().all(|order| order.type_id == PLEX_ID));
    }

    #[test]
    fn from_api_load_buy() {
        let orders = Order::load(&REGION_ID, OrderType::BUY, Some(PLEX_ID));
        assert!(!orders.is_empty());
        assert!(orders.iter().all(|order| order.type_id == PLEX_ID));
        assert!(orders.iter().all(|order| order.is_buy_order));
    }

    #[test]
    fn from_api_load_sell() {
        let orders = Order::load(&REGION_ID, OrderType::SELL, Some(PLEX_ID));
        assert!(!orders.is_empty());
        assert!(orders.iter().all(|order| order.type_id == PLEX_ID));
        assert!(orders.iter().all(|order| !order.is_buy_order));
    }

}
