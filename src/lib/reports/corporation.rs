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

/*
    pub alliance_id: IntOptional,
    pub faction_id: IntOptional,
    pub home_station_id: IntOptional,
    pub shares: LongOptional,
    pub url: StrOptional,
    pub war_eligible: BoolOptional,
    pub description: StrOptional,
*/

impl Corporation {
    pub fn description(id: &i32, ctx: &Context) -> String {
        let mut output = String::new();
        if let Some(corporation) = api::corporation::Corporation::new(id) {
            reports::div(&mut output, format!("Corporation: [{}] {}",corporation.ticker, ctx.get_full_desc("corporation", *id, corporation.name)));
            reports::div(&mut output, format!("Members:          {}", corporation.member_count));
            reports::div(&mut output, format!("Taxes:            {}", corporation.tax_rate));
            reports::div(&mut output, format!("Eligible War: {}", corporation.war_eligible.unwrap_or(false)));
            reports::div(&mut output, format!("CEO: {}",
                api::character::Character::new(&corporation.ceo_id).map(|ch| ch.name).unwrap_or_default()
            ));
            reports::div(&mut output, format!("Creator: {}",
                api::character::Character::new(&corporation.creator_id).map(|ch| ch.name).unwrap_or_default()
            ));
            if let Some(ref date_founded) = corporation.date_founded {
                reports::div(&mut output, format!("Founded:      {}", date_founded.format("%Y-%m-%d %H:%M:%S").to_string()));
            }

        }
        return output;
    }
}