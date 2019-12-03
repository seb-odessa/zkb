use crate::services::Context;
use crate::reports;


#[derive(Debug, PartialEq)]
pub struct Alliance;

impl reports::Reportable for Alliance {

    fn report_by_id(id: &i32, ctx: &Context) -> String {
        let mut output = String::new();
        reports::div(&mut output, "Wins");
        reports::lazy(&mut output, format!("report/alliance/wins/{}/{}", id, 60), &ctx);
        reports::div(&mut output, "Losses");
        reports::lazy(&mut output, format!("report/alliance/losses/{}/{}", id, 60), &ctx);
        return output;
    }
}
