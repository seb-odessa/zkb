use serde::{Deserialize, Serialize};
use crate::api::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
struct Item {
    type_id: IntRequired,
    adjusted_price: FloatOptional,
    average_price: FloatOptional
}

pub struct Price {
    pub adjusted: FloatOptional,
    pub average: FloatOptional
}
impl Price {
    fn new(item: &Item) -> Self {
        Self { adjusted: item.adjusted_price, average: item.average_price }
    }
}

pub struct Prices {
    pub items: HashMap<IntRequired, Price>
}
impl Prices {
    fn receive() -> Vec<Item> {
        let response = gw::eve_api("markets/prices").unwrap_or_default();
        serde_json::from_str(&response).ok().unwrap_or_default()
    }

    pub fn new() -> Self {
        let items = Self::receive()
                        .iter()
                        .map(|item| (item.type_id, Price::new(item)))
                        .collect();
        Prices{ items: items }
    }

    pub fn avg(&self, id: IntRequired) -> FloatOptional {
        self.items.get(&id).and_then(|price| price.average)
    }

    pub fn adj(&self, id: IntRequired) -> FloatOptional {
        self.items.get(&id).and_then(|price| price.average)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_api() {
        let prices = Prices::new();
        assert!(!prices.items.len() > 0);
    }
}
