use crate::models::*;

use killmail::KillmailNamed;
use attacker::AttackerNamed;
use victim::VictimNamed;

use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Killmail {
    pub killmail_id: Integer,
    pub killmail_time: DateTime,
    pub system_id: Integer,
    pub system_name: OptString,
    pub moon_id: OptInteger,
    pub moon_name: OptString,
    pub war_id: OptInteger,
    pub victim: VictimNamed,
    pub attackers: Vec<AttackerNamed>,
}
impl Killmail {
    pub fn load(conn: &Connection, id: &Integer) -> QueryResult<Self> {
        let killmail = KillmailNamed::load(conn, id)?;
        Ok(Self {
            killmail_id: killmail.killmail_id,
            killmail_time: killmail.killmail_time,
            system_id: killmail.system_id,
            system_name: killmail.system_name,
            moon_id: killmail.moon_id,
            moon_name: killmail.moon_name,
            war_id: killmail.war_id,
            victim: VictimNamed::load(conn, id)?,
            attackers: AttackerNamed::load(conn, id)?,
        })
    }
}

fn href(category: &'static str, id: &Option<i32>, name: &Option<String>) -> String {
    format!("<a href=\"https://zkillboard.com/{}/{}/\">{}</a>", 
        category, 
        id.as_ref().cloned().unwrap_or(0), 
        name.as_ref().cloned().unwrap_or_default())
}

impl fmt::Display for Killmail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<div>\n")?;
        write!(f, "{}\n", href("kill", &Some(self.killmail_id), &Some(self.killmail_time.to_string())))?;
        write!(f, "&nbsp;&nbsp;&nbsp;\n")?;
        write!(f, "{}\n", href("system", &Some(self.system_id), &self.system_name))?;
        write!(f, "<div>\n")?;

        write!(f, "<table>\n")?;
        write!(f, "<tr><th>Damage</th><th>Ship</th><th>Character</th><th>Corporation</th><th>Alliance</th><th>Weapon</th></tr>\n")?;
        write!(f, "<tr><td align=\"right\">{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td></td></tr>\n",
            self.victim.damage_taken,
            href("item", &Some(self.victim.ship_id), &self.victim.ship_name),
            href("character", &self.victim.character_id, &self.victim.character_name),
            href("corporation", &self.victim.corporation_id, &self.victim.corporation_name),
            href("alliance", &self.victim.alliance_id, &self.victim.alliance_name),
        )?;
        for attacker in &self.attackers {
            write!(f, "<tr><td  align=\"right\">{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
                attacker.damage_done,
                href("item", &attacker.ship_id, &attacker.ship_name),
                href("character", &attacker.character_id, &attacker.character_name),
                href("corporation", &attacker.corporation_id, &attacker.corporation_name),
                href("alliance", &attacker.alliance_id, &attacker.alliance_name),
                href("item", &attacker.weapon_id, &attacker.weapon_name),
            )?;
        }
        write!(f, "</table>\n")
    }
}