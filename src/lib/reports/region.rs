use crate::api;
use crate::services::Context;
use crate::reports::{div, href};

#[derive(Debug, PartialEq)]
pub struct Region;
impl Region {
    pub fn report(id: &i32, _ctx: &Context) -> String {
        let mut output = String::new();
        if let Some(object) = api::region::Region::new(id) {
            div(&mut output, "Region", &href(object.zkb(), object.name.clone()));
        } else {
            div(&mut output, "Region", &format!("{} not found", id));
        }
        return output;
    }
}
