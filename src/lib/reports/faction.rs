use crate::services::Context;
use crate::reports;

#[derive(Debug, PartialEq)]
pub struct Faction;
impl reports::Reportable for Faction {

    fn report_by_id(id: &i32, ctx: &Context) -> String {
        let mut output = String::new();
        reports::div(&mut output, "Wins");
        reports::lazy(&mut output, format!("report/faction/wins/{}/{}", id, 60), &ctx);
        reports::div(&mut output, "Losses");
        reports::lazy(&mut output, format!("report/faction/losses/{}/{}", id, 60), &ctx);
        return output;
    }
}