use crate::api;
use crate::services::Context;
use crate::reports::*;

#[derive(Debug, PartialEq)]
pub struct Stargate;
impl Stargate {

    pub fn report(arg: &String, ctx: &Context) -> String {
        if let Ok(ref id) = arg.parse::<i32>() {
            Self::report_by_id(id, ctx)
        }
        else {
            format!("<div>Stargate {} was not found</div>", arg)
        }
    }

    pub fn report_by_id(id: &i32, ctx: &Context) -> String {
        let mut output = String::new();
        if let Some(object) = api::stargate::Stargate::new(id) {
            lazy(&mut output, format!("api/system_brief/{}", object.destination.system_id), &ctx);
        } else {
            div(&mut output, format!("Can't query Stargate({}) from CCP API", id));
        }
        return output;
    }
}
