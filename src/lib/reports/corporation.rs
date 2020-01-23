use crate::services::Context;
use crate::reports;
use crate::api;

#[derive(Debug, PartialEq)]
pub struct Corporation;

impl reports::Reportable for Corporation {

    fn report_by_id(id: &i32, ctx: &Context) -> String {
        let mut output = String::new();
        reports::lazy(&mut output, format!("desc/corporation/{}", id), &ctx);
        reports::div(&mut output, "Wins");
        reports::lazy(&mut output, format!("report/corporation/wins/{}/{}", id, 60), &ctx);
        reports::div(&mut output, "Losses");
        reports::lazy(&mut output, format!("report/corporation/losses/{}/{}", id, 60), &ctx);
        return output;
    }
}

impl Corporation {
    pub fn description(id: &i32, ctx: &Context) -> String {
        let mut output = String::new();
        if let Some(corporation) = api::corporation::Corporation::new(id) {
            reports::div(&mut output, format!("Corporation: [{}] {}", corporation.ticker, ctx.get_full_desc("corporation", *id, corporation.name)));
            if let Some(ref alliance_id) = corporation.alliance_id {
                reports::div(&mut output, format!("Alliance:         {}",
                    ctx.get_full_desc("alliance",
                        *alliance_id,
                        api::alliance::Alliance::new(&alliance_id).map(|x| x.name).unwrap_or_default()
                    )
                ));
            }
            reports::div(&mut output, format!("Members:          {}", corporation.member_count));
            reports::div(&mut output, format!("Taxes:            {}", corporation.tax_rate));
            reports::div(&mut output, format!("Eligible War:     {}", corporation.war_eligible.unwrap_or(false)));
            reports::div(&mut output, format!("URL    :          {}", corporation.url.clone().unwrap_or_default()));
            reports::div(&mut output, format!("Founded:          {}", corporation.date_founded.clone().map(|x| x.format("%Y-%m-%d %H:%M:%S").to_string()).unwrap_or_default()));
            reports::div(&mut output, format!("CEO:              {}",
                ctx.get_full_desc("character",
                    corporation.ceo_id,
                    api::character::Character::new(&corporation.ceo_id).map(|x| x.name).unwrap_or_default())
            ));
            reports::div(&mut output, format!("Creator:          {}",
                ctx.get_full_desc("character",
                    corporation.creator_id,
                    api::character::Character::new(&corporation.creator_id).map(|x| x.name).unwrap_or_default())
            ));
        }
        return output;
    }
}