use lib::api::{Order, Orders, OrderType};
use separator::Separatable;


const DOMAIN_ID: i32 = 10000043;
const THE_FORGE: i32 = 10000002;
const ITEM_ID: i32 = 44992;

struct OrdersHolder {
    pub src_system_id: i32,
    pub dst_system_id: i32,
    pub src_system_orders: Orders,
    pub dst_system_orders: Orders,
}
impl OrdersHolder {
    pub fn new(src_system_id: i32, dst_system_id: i32) -> Self {
        Self {
            src_system_id: src_system_id,
            dst_system_id: dst_system_id,
            src_system_orders: Order::load_for_system(&src_system_id, OrderType::SELL, None),
            dst_system_orders: Order::load_for_system(&dst_system_id, OrderType::BUY, None)
        }
    }

    pub fn get_best_sells(self) -> Orders {
        use std::cmp::Ordering;
        let mut orders = self.src_system_orders.clone();
        orders.as_mut_slice().sort_by(|lv, rv| {
            match lv.type_id.cmp(&rv.type_id) {
                Ordering::Equal => {
                    match lv.location_id.cmp(&rv.location_id) {
                        Ordering::Equal => rv.price.partial_cmp(&lv.price).unwrap_or(Ordering::Equal),
                        other => other
                    }
                },
                other => other
            }
        });

        let mut type_id = 0;
        let mut location_id = 0;
        orders
            .iter()
            .filter(|o| {
                let ok = type_id != o.type_id || location_id != o.location_id;
                if ok {
                    type_id = o.type_id;
                    location_id = o.location_id;
                }
                return ok
            })
            .cloned()
            .collect()
    }

}

fn main() {
    let orders = OrdersHolder::new(30000142, 30002187);

    for order in &orders.get_best_sells() {
        println!("{} {} {} {} {:>6} {:>6} {:>2} {}",
            order.system_id,
            order.location_id,
            order.type_id,
             if order.is_buy_order {"B"} else {"S"},
            order.volume_remain,
            order.volume_total,
            order.min_volume,
            order.price.separated_string()
        );
    }

}
