use crate::models::*;
use super::zkb_href;
use crate::services::Context;
use crate::services::server::root;
use crate::reports::FAIL;

use killmail::KillmailNamed;
use attacker::AttackerNamed;
use victim::VictimNamed;

use std::fmt;
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
    // pub victim: VictimNamed,
    // pub attackers: Vec<AttackerNamed>,
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
            // victim: VictimNamed::load(conn, id)?,
            // attackers: AttackerNamed::load(conn, id)?,
        })
    }

    pub fn write(&self, output: &mut dyn Write, root: &String) {
        let empty = String::new();
        write(
            output,
            format_args!(
                r##"
                    <div>
                    <a href="{root}/api/killmail/{id}">{id}</a>
                    <a href="https://zkillboard.com/kill/{id}/">zkb</a>
                    {timestamp}
                    <a href="https://zkillboard.com/region/{region_id}/">{region_name}</a>/
                    {constellation}/
                    <a href="{root}/api/system/{system_id}">{system}</a>
                    </div>
                "##,
                id = self.killmail_id,
                timestamp = self.killmail_time.to_string(),
                region_id = self.region_id.unwrap_or_default(),
                region_name = self.region_name.as_ref().unwrap_or(&empty),
                constellation = self.constellation_name.as_ref().unwrap_or(&empty),
                root = root,
                system_id = self.system_id,
                system = self.system_name.as_ref().unwrap_or(&empty),
            )
        ).expect(FAIL);

    }

    pub fn brief(id: &Integer, ctx: &Context) -> String {
        use crate::services::*;

        let mut output = String::new();
        let root = root(&ctx);
        ctx.database.push(Message::Load(Category::Killmail(*id)));
        while let Some(msg) = ctx.responses.pop() {
            if let Message::Report(Report::Killmail(killmail)) = msg {
                if killmail.killmail_id == *id {
                    killmail.write(&mut output, &root);
                    break;
                } else {
                    ctx.responses.push(Message::Report(Report::Killmail(killmail)));
                }
            } else if let Message::NotFound(id) = msg {
                write(&mut output, format_args!("<div>Killmail {} was not found</div>", id)).expect(FAIL);
                break;
            } else {
                warn!("Unexpected {:?}", &msg);
                ctx.responses.push(msg);
            }
        }
        return output;
    }

}
impl fmt::Display for Killmail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<div>\n")?;
        write!(f, "{}\n", zkb_href("kill", &Some(self.killmail_id), &Some(self.killmail_time.to_string())))?;
        write!(f, "&nbsp;&nbsp;&nbsp;\n")?;
        write!(f, "&nbsp;{}&nbsp;", zkb_href("system", &Some(self.system_id), &self.system_name))?;
        write!(f, "&nbsp;{}&nbsp;", zkb_href("constellation", &self.constellation_id, &self.constellation_name))?;
        write!(f, "&nbsp;{}&nbsp;", zkb_href("region", &self.region_id, &self.region_name))?;
        write!(f, "</div>\n")?;

        write!(f, "<table>\n")?;
        write!(f, "<tr><th>Damage</th><th>Ship</th><th>Weapon</th><th>Character</th><th>Corporation</th><th>Alliance</th></tr>\n")?;
        // write!(f, "<tr><td align=\"right\">{}</td><td>{}</td><td></td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
        //     self.victim.damage_taken,
        //     zkb_href("item", &Some(self.victim.ship_id), &self.victim.ship_name),
        //     zkb_href("character", &self.victim.character_id, &self.victim.character_name),
        //     zkb_href("corporation", &self.victim.corporation_id, &self.victim.corporation_name),
        //     zkb_href("alliance", &self.victim.alliance_id, &self.victim.alliance_name),
        // )?;
        // for attacker in &self.attackers {
        //     if attacker.faction_id.is_some() {
        //         write!(f, "<tr><td  align=\"right\">{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
        //             attacker.damage_done,
        //             zkb_href("item", &attacker.ship_id, &attacker.ship_name),
        //             zkb_href("item", &attacker.weapon_id, &attacker.weapon_name),
        //             zkb_href("faction", &attacker.faction_id, &attacker.faction_name),
        //             "",
        //             "")?;
        //     } else {
        //             write!(f, "<tr><td  align=\"right\">{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
        //                 attacker.damage_done,
        //                 zkb_href("item", &attacker.ship_id, &attacker.ship_name),
        //                 zkb_href("item", &attacker.weapon_id, &attacker.weapon_name),
        //                 zkb_href("character", &attacker.character_id, &attacker.character_name),
        //                 zkb_href("corporation", &attacker.corporation_id, &attacker.corporation_name),
        //                 zkb_href("alliance", &attacker.alliance_id, &attacker.alliance_name))?;
        //     }
        // }
        write!(f, "</table>\n")
    }
}