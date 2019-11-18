use crate::models::*;
use crate::services::Context;
use crate::services::server::root;
use crate::reports::FAIL;
use crate::reports;

use std::fmt::Write;

#[derive(Debug, PartialEq)]
pub struct Victim;
impl Victim {

    pub fn write(output: &mut dyn Write, victim: &victim::VictimNamed, _root: &String) {
        std::fmt::write(
            output,
            format_args!(
                r#"<div>{name}</div>"#,
                name = victim.character_name.as_ref().map(|x|x.clone()).unwrap_or_default()
            )
        ).expect(FAIL);
    }

    pub fn brief(arg: &String, ctx: &Context) -> String {
        if let Ok(ref id) = arg.parse::<i32>() {
            Self::brief_impl(id, ctx)
        } else {
            format!("<div>Killmail {} was not found</div>", arg)
        }
    }

    pub fn brief_impl(id: &Integer, ctx: &Context) -> String {
        use crate::services::*;

        let mut output = String::new();
        let msg_id = crate::create_id().to_simple();
        ctx.database.push(Message::Find((msg_id, Category::Victim(*id))));
        match reports::wait_for(msg_id, &ctx) {
            Report::Victim(victim) => Self::write(&mut output, &victim, &root(&ctx)),
            Report::NotFoundId(id) => reports::div(&mut output, format!("Killmail {} was not found", id)),
            report => warn!("Unexpected report {:?}", report)
        }
        return output;
    }
}
