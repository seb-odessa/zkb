use crate::api;
use crate::services::Context;
use crate::reports::{div, lazy};

#[derive(Debug, PartialEq)]
pub struct Stargate;
impl Stargate {
    pub fn report(id: &i32, ctx: &Context) -> String {
        let mut output = String::new();
        if let Some(object) = api::stargate::Stargate::new(id) {
            lazy(&mut output, format!("api/system_brief/{}", object.destination.system_id), &ctx);
        } else {
            div(&mut output, "Stargate", &format!("{} not found", id));
        }
        return output;
    }
}
