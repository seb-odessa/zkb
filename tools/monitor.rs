#[macro_use]
extern crate log;
extern crate separator;

use separator::Separatable;
use std::fmt;

use lib::api::*;
use lib::api::system::*;
use lib::provider;
use std::thread;

#[derive(Debug)]
struct Participant {
    pub character_id: IntOptional,
    pub ship_id: IntOptional,
    pub damage: IntRequired,
}
impl Participant {
    pub fn new(character_id: IntOptional, ship_id: IntOptional, damage: IntRequired) -> Self {
        Self { character_id, ship_id, damage }
    }
    pub fn character(&self) -> String {
        provider::get_name(&self.character_id)
    }
    pub fn ship(&self) -> String {
        provider::get_name(&self.ship_id)
    }
    pub fn damage(&self) -> i32 {
        self.damage
    }
}

#[derive(Debug)]
struct Report {
    pub npc_only: bool,
    pub time: TimeRequired,
    pub zkb_url: String,
    pub system_id: i32,
    pub total_value: u32,
    pub dropped_value: u32,
    pub victim: Participant,
    pub attackers: Vec<Participant>,

}
impl Report {
    pub fn new(package: zkb::Package) -> Option<Self> {
        if let Some(content) = package.content {
            let killmail = content.killmail;
            Some(
                Self {
                    npc_only: content.zkb.npc,
                    time: killmail.killmail_time,
                    zkb_url: killmail.href(),
                    system_id: killmail.solar_system_id,
                    total_value: killmail.get_total_sum(),
                    dropped_value: killmail.get_dropped_sum(),
                    victim: Participant::new(
                        killmail.victim.character_id,
                        Some(killmail.victim.ship_type_id),
                        killmail.victim.damage_taken),
                    attackers: killmail.attackers.iter()
                        .map(|a| Participant::new(
                            a.character_id,
                            a.ship_type_id,
                            a.damage_done))
                        .collect(),
                }
            )
        } else {
            None
        }
    }
    pub fn system_name(&self) -> String {
        provider::get_name(&Some(self.system_id))
    }

}
impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:>30} | {:>40}   {:>20} | J-{:>02} | A-{:>02} | D-{:>02} | R-{:>02} | H-{:>02} |",
            self.time.to_string(),
            self.zkb_url,
            "",
            provider::get_route(JITA_ID, self.system_id).len(),
            provider::get_route(AMARR_ID, self.system_id).len(),
            provider::get_route(DODIXIE_ID, self.system_id).len(),
            provider::get_route(RENS_ID, self.system_id).len(),
            provider::get_route(HEK_ID, self.system_id).len()
        )?;
        writeln!(f, "{:^30} | {:>40} | {:>20} |",
            self.system_name(),
            self.total_value.separated_string(),
            self.dropped_value.separated_string())?;
        writeln!(f, "{:>30} | {:>40} | {:>20} |",
            self.victim.character(),
            self.victim.ship(),
            self.victim.damage())?;
        for attacker in &self.attackers {
            writeln!(f, "{:<30} | {:>40} | {:>20} |",
                attacker.character(),
                attacker.ship(),
                attacker.damage())?;
        }
        writeln!(f, "{}{}{}{}",
                    format!("{:-^1$}|", "-", 31),
                    format!("{:-^1$}|", "-", 42),
                    format!("{:-^1$}|", "-", 22),
                    format!("{:-^1$}|", "-", 34))
    }
}


fn run_monitor(id: String, timeout: u32) {
    loop {
        while let Some(package) = gw::get_package(&id) {
            info!("Received response from API");
            if let Some(report) = Report::new(package) {
                info!("Report ready to display");
                // let accepted = true;
                    // report.attackers.iter().any(|a| a.ship().starts_with("Mordu"));
                    // || report.npc_only;
                // if accepted {
                    print!("{}", report);
                // }
            }
        }
        warn!("Perform sleep {} sec ", timeout);
        thread::sleep(std::time::Duration::from_secs(timeout.into()));
    }
}

fn main() {
    env_logger::init();
    let args: Vec<_> = std::env::args().collect();
    if 3 == args.len() {
        let id: String = args[1]
            .parse()
            .expect("Can't convert first argument to the request id");
        let timeout: u32 = args[2]
            .parse()
            .expect("Can't convert second argument to the timeout");
        run_monitor(id , timeout);
    } else {
        println!("Usage:");
        println!("\n\t {} id timeout", args[0]);
    }
}
