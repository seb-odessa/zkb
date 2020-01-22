use crate::services::Context;
use crate::reports;
use crate::api;


#[derive(Debug, PartialEq)]
pub struct Alliance;

impl reports::Reportable for Alliance {
    fn report_by_id(id: &i32, ctx: &Context) -> String {
        let mut output = String::new();
        reports::lazy(&mut output, format!("desc/alliance/{}", id), &ctx);
        reports::div(&mut output, "Wins");
        reports::lazy(&mut output, format!("report/alliance/wins/{}/{}", id, 60), &ctx);
        reports::div(&mut output, "Losses");
        reports::lazy(&mut output, format!("report/alliance/losses/{}/{}", id, 60), &ctx);
        return output;
    }
}

impl Alliance {
    pub fn description(id: &i32, ctx: &Context) -> String {
        let mut output = String::new();
        if let Some(alliance) = api::alliance::Alliance::new(id) {
            reports::div(&mut output, format!("Alliance: [{}] {}",alliance.ticker, ctx.get_full_desc("alliance", *id, alliance.name)));
            reports::div(&mut output, format!("Founded:          {}", alliance.date_founded.format("%Y-%m-%d %H:%M:%S").to_string()));
            reports::div(&mut output, format!("Creator:          {}",
                ctx.get_full_desc("character",
                    alliance.creator_id,
                    api::character::Character::new(&alliance.creator_id).map(|ch| ch.name).unwrap_or_default())
            ));
            reports::div(&mut output, format!("Creator Corp:     {}",
                ctx.get_full_desc("corporation",
                    alliance.creator_corporation_id,
                    api::corporation::Corporation::new(&alliance.creator_corporation_id).map(|ch| ch.name).unwrap_or_default())
            ));
            if let Some(executor_id) = alliance.executor_corporation_id {
                reports::div(&mut output, format!("CEO: {}",
                    ctx.get_full_desc("corporation",
                        executor_id,
                        api::corporation::Corporation::new(&executor_id).map(|ch| ch.name).unwrap_or_default())
                ));
            }
        }
        return output;
    }
}
