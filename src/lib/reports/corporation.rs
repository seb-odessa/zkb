use crate::services::Context;
use crate::reports;
use crate::api;

#[derive(Debug, PartialEq)]
pub struct Corporation;

impl reports::Reportable for Corporation {

    fn report_by_id(id: &i32, ctx: &Context) -> String {
        let mut output = String::new();
        reports::lazy(&mut output, format!("desc/corporation/{}", id), &ctx);
        reports::lazy(&mut output, format!("stat/corporation/{}", id), &ctx);
        reports::lazy(&mut output, format!("report/corporation/wins/{}/{}", id, 60), &ctx);
        reports::lazy(&mut output, format!("report/corporation/losses/{}/{}", id, 60), &ctx);
        reports::radar(&mut output, &ctx);
        reports::observer(&mut output, vec!["'Sun'", "'Mon'", "'Tue'", "'Wed'", "'Thu'", "'Fri'", "'Sat'"]);
        return output;
    }
}

impl Corporation {
    pub fn description(id: &i32, ctx: &Context) -> String {
        let mut output = String::new();
        if let Some(corporation) = api::corporation::Corporation::new(id) {
            reports::div(&mut output, format!("Corporation: [{}] {}", corporation.ticker, ctx.get_actor_desc("corporation", *id, corporation.name)));
            if let Some(ref alliance_id) = corporation.alliance_id {
                reports::div(&mut output, format!("Alliance:         {}",
                    ctx.get_actor_desc("alliance",
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
                ctx.get_actor_desc("character",
                    corporation.ceo_id,
                    api::character::Character::new(&corporation.ceo_id).map(|x| x.name).unwrap_or_default())
            ));
            reports::div(&mut output, format!("Creator:          {}",
                ctx.get_actor_desc("character",
                    corporation.creator_id,
                    api::character::Character::new(&corporation.creator_id).map(|x| x.name).unwrap_or_default())
            ));
        }
        return output;
    }

    pub fn stat(id: &i32, ctx: &Context) -> String {
        use api::stats::Stats;
        use api::stats::Entity;
        use api::stats::TopList;
        use api::stats::Activity;
        use std::collections::HashSet;

        let mut output = String::new();
        if let Some(stats) = Stats::new(Entity::Corporation(*id)) {
            Stats::report_win_loses(&mut output, "Ships", stats.ship_destroyed, stats.ship_lost);
            Stats::report_win_loses(&mut output, "Solo", stats.solo_kills, stats.solo_losses);
            reports::div(&mut output, format!("Danger: {} %", stats.danger_ratio()));
            reports::div(&mut output, format!("Gangs: {} %", stats.gang_ratio()));
            if let Some(ref activity) = stats.activity {
                Activity::write(&mut output, activity, ctx);
            }
            //character, corporation, alliance, shipType, solarSystem, location
            let allowed: HashSet<String> = vec!["character", "shipType", "solarSystem", "location"].into_iter().map(|s| String::from(s)).collect();
            TopList::write(&mut output, &stats.top_lists, allowed, ctx);
        }
        return output;
    }
}