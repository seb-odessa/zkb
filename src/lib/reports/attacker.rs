use crate::models::*;
use crate::services::Context;
use crate::services::server::root;
use crate::reports;

use std::fmt::Write;

#[derive(Debug, PartialEq)]
pub struct Victim;
impl Victim {

    pub fn write(output: &mut dyn Write, attacker: &attacker::AttackerNamed, _root: &String) {
        std::fmt::write(
            output,
            format_args!(
                r#"<div>{name}</div>"#,
                name = attacker.character_name.as_ref().unwrap_or(&String::new())
            )
        ).expect(reports::FAIL);
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
        let root = root(&ctx);

        match reports::load(Category::Attakers(*id), &ctx) {
            Report::Attakers(attackers) => {
                for attacker in attackers {
                    Self::write(&mut output, &attacker, &root);
                    }
                },
            Report::NotFoundId(id) => reports::div(&mut output, format!("Killmail {} was not found", id)),
            report => warn!("Unexpected report {:?}", report)
        }

        return output;
    }
}
