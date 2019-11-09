use crate::api;
use crate::services::Context;
use crate::reports::div;

#[derive(Debug, PartialEq)]
pub struct Constellation;
impl Constellation {

    pub fn report(id: &i32, _: &Context) -> String {
        let mut output = String::new();
        if let Some(object) = api::constellation::Constellation::new(id) {
            div(&mut output, "Constellation", &object.name);
        } else {
            div(&mut output, "Constellation", &format!("{} not found", id));
        }
        return output;
    }
}
