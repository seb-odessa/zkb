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
            format_args!("<div>{}</div>", victim.get_name("character")
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
        match reports::load(Category::Victim(*id), &ctx) {
            Report::Victim(victim) => Self::write(&mut output, &victim, &root(&ctx)),
            Report::NotFoundId(id) => reports::div(&mut output, format!("Killmail {} was not found", id)),
            report => warn!("Unexpected report {:?}", report)
        }
        return output;
    }
}
