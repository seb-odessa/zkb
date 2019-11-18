use crate::models::*;
use crate::services::Context;
use crate::services::server::root;
use crate::reports::{FAIL, System};

use killmail::KillmailNamed;

use std::fmt::Write;
use std::fmt::write;

#[derive(Debug, PartialEq)]
pub struct Killmail {
    pub killmail_id: Integer,
    pub killmail_time: DateTime,
    pub system_id: Integer,
    pub system_name: OptString,
    pub constellation_id: OptInteger,
    pub constellation_name: OptString,
    pub region_id: OptInteger,
    pub region_name: OptString,
}
impl Killmail {
    pub fn load(conn: &Connection, id: &Integer) -> QueryResult<Self> {
        let killmail = KillmailNamed::load(conn, id)?;
        Ok(Self {
            killmail_id: killmail.killmail_id,
            killmail_time: killmail.killmail_time,
            system_id: killmail.system_id,
            system_name: killmail.system_name,
            constellation_id: killmail.constellation_id,
            constellation_name: killmail.constellation_name,
            region_id: killmail.region_id,
            region_name: killmail.region_name,
        })
    }

    pub fn write(output: &mut dyn Write, killmail: &killmail::KillmailNamed, root: &String) {
        let empty = String::new();
        let status_id: String = crate::create_id().to_string();
        write(
            output,
            format_args!(
                r##"
                    <div>
                    <a href="{root}/api/killmail/{id}">{id}</a>
                    <a href="https://zkillboard.com/kill/{id}/">zkb</a>
                    {timestamp}
                    <a href="{root}/api/region/{region_id}">{region_name}</a> :
                    <a href="{root}/api/constellation/{constellation_id}">{constellation_name}</a> :
                    <a href="{root}/api/system/{system_id}">{system}</a>
                    <span>{status}</span>
                    </div>
                "##,
                id = killmail.killmail_id,
                timestamp = killmail.killmail_time.to_string(),
                region_id = killmail.region_id.unwrap_or_default(),
                region_name = killmail.region_name.as_ref().unwrap_or(&empty),
                constellation_id = killmail.constellation_id.unwrap_or_default(),
                constellation_name = killmail.constellation_name.as_ref().unwrap_or(&empty),
                root = root,
                system_id = killmail.system_id,
                system = killmail.system_name.as_ref().unwrap_or(&empty),
                status = System::security_status(killmail.system_id),
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
        let root = root(&ctx);
        let msg_id = crate::create_id().to_simple();
        ctx.database.push(Message::Find((msg_id, Category::Killmail(*id))));
        while let Some(msg) = ctx.responses.pop() {
            if let Message::Report((report_id, ref report)) = msg {
                if report_id == msg_id {
                    match report {
                        Report::Killmail(killmail) => {
                            Self::write(&mut output, &killmail, &root);
                        },
                        Report::NotFoundId(killmail_id) => {
                            write(&mut output, format_args!("<div>Killmail {} was not found</div>", killmail_id)).expect(FAIL);
                        }
                        report => {
                            warn!("Unexpected report {:?}", report);
                        }
                    }
                    break;
                } else {
                   ctx.responses.push(msg);
                }
            }
        }
        return output;
    }
}
