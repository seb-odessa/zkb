use crate::services::Context;
use crate::reports;
use crate::api;


#[derive(Debug, PartialEq)]
pub struct Alliance;

impl reports::Reportable for Alliance {
    fn report_by_id(id: &i32, ctx: &Context) -> String {
        let mut output = String::new();
        reports::lazy(&mut output, format!("desc/alliance/{}", id), &ctx);
        reports::lazy(&mut output, format!("stat/alliance/{}", id), &ctx);
        reports::lazy(&mut output, format!("report/alliance/wins/{}/{}", id, 60), &ctx);
        reports::lazy(&mut output, format!("report/alliance/losses/{}/{}", id, 60), &ctx);
        reports::radar(&mut output, &ctx);
        reports::observer(&mut output, vec!["'Sun'", "'Mon'", "'Tue'", "'Wed'", "'Thu'", "'Fri'", "'Sat'"]);
        return output;
    }
}

impl Alliance {
    pub fn description(id: &i32, ctx: &Context) -> String {
        let mut output = String::new();
        if let Some(alliance) = api::alliance::Alliance::new(id) {
            reports::div(&mut output, format!("Alliance: [{}] {}", alliance.ticker, ctx.get_actor_desc("alliance", *id, alliance.name)));
            reports::div(&mut output, format!("Founded:          {}", alliance.date_founded.format("%Y-%m-%d %H:%M:%S").to_string()));
            reports::div(&mut output, format!("Creator:          {}",
                ctx.get_actor_desc("character",
                    alliance.creator_id,
                    api::character::Character::new(&alliance.creator_id).map(|ch| ch.name).unwrap_or_default())
            ));
            reports::div(&mut output, format!("Creator Corp:     {}",
                ctx.get_actor_desc("corporation",
                    alliance.creator_corporation_id,
                    api::corporation::Corporation::new(&alliance.creator_corporation_id).map(|ch| ch.name).unwrap_or_default())
            ));
            if let Some(executor_id) = alliance.executor_corporation_id {
                reports::div(&mut output, format!("Executor Corp:    {}",
                    ctx.get_actor_desc("corporation",
                        executor_id,
                        api::corporation::Corporation::new(&executor_id).map(|ch| ch.name).unwrap_or_default())
                ));
            }
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
        if let Some(stats) = Stats::new(Entity::Alliance(*id)) {
            Stats::report_win_loses(&mut output, "Ships", stats.ship_destroyed, stats.ship_lost);
            Stats::report_win_loses(&mut output, "Solo", stats.solo_kills, stats.solo_losses);
            reports::div(&mut output, format!("Danger: {} %", stats.danger_ratio()));
            reports::div(&mut output, format!("Gangs: {} %", stats.gang_ratio()));
            if let Some(ref activity) = stats.activity {
                Activity::write(&mut output, activity, ctx);
            }
            //character, corporation, alliance, shipType, solarSystem, location
            let allowed: HashSet<String> = vec!["character", "corporation", "shipType", "solarSystem", "location"].into_iter().map(|s| String::from(s)).collect();
            TopList::write(&mut output, &stats.top_lists, allowed, ctx);
        }
        return output;
    }
}
