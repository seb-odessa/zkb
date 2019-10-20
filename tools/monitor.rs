#[macro_use]
extern crate log;
extern crate separator;
use separator::Separatable;
use std::fmt;

use lib::api::*;
use lib::api::system::*;
use lib::api::price::Prices;
use lib::api::object::Object;
use std::thread;
use std::convert::TryFrom;
use std::collections::HashMap;

struct NameProvider {
    names: HashMap<i32, String>,
}
impl NameProvider {
    pub fn new() -> Self {
        NameProvider {names: HashMap::new() }
    }

    pub fn get(&mut self, id: IntOptional) -> String {
        if let Some(id) = id {
            self.names.entry(id).or_insert(Object::try_from(id).ok().unwrap_or_default().name);
            self.names.get(&id).map(|name| name.clone()).unwrap_or_default()
        } else {
            String::new()
        }
    }
}

#[derive(Debug)]
struct Participant {
    pub name: String, 
    pub ship: String, 
    pub damage: i32,
}
impl Participant {
    pub fn new(name: String, ship: String, damage: i32) -> Self {
        Self { name, ship, damage }
    }
}

#[derive(Debug)]
struct Report {
    pub time: TimeRequired,
    pub zkb_url: String,
    pub system_id: i32,
    pub system_name: String,
    pub total_value: u64,
    pub victim: Participant,
    pub attackers: Vec<Participant>,

}
impl Report {
    pub fn new(package: zkb::Package, names: &mut NameProvider) -> Option<Self> {
        if let Some(content) = package.content {
            Some(
                Self {
                    time: content.killmail.killmail_time,
                    zkb_url: content.zkb_url(),
                    system_id: content.killmail.solar_system_id,
                    system_name: names.get(Some(content.killmail.solar_system_id)),
                    total_value: content.zkb.total_value as u64,
                    victim: Participant::new(
                        names.get(content.killmail.victim.character_id),
                        names.get(Some(content.killmail.victim.ship_type_id)),
                        content.killmail.victim.damage_taken),
                    attackers: content.killmail.attackers.iter()
                        .map(|a|
                            Participant::new(
                                names.get(a.character_id),
                                names.get(a.ship_type_id),
                                a.damage_done))
                        .collect(),
                }
            )
        } else {
            None
        }
    }
}
impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:>40} | {:>30} | {:>10} | J-{:>02} | A-{:>02} | D-{:>02} | R-{:>02} | H-{:>02} |", 
            self.zkb_url, 
            self.time.to_string(), 
            self.system_name, 
            route(JITA_ID, self.system_id).len(), 
            route(AMARR_ID, self.system_id).len(),
            route(DODIXIE_ID, self.system_id).len(),
            route(RENS_ID, self.system_id).len(), 
            route(HEK_ID, self.system_id).len()
        )?;
        writeln!(f, "{:>40} | {:>30} | {:>10} |", "", self.total_value.separated_string(),"")?;
        writeln!(f, "{:>40} | {:>30} | {:>10} |", self.victim.name, self.victim.ship, self.victim.damage)?;
        for attacker in &self.attackers {
            writeln!(f, "{:>40} | {:>30} | {:>10} |", attacker.name, attacker.ship, attacker.damage)?;
        }
        writeln!(f, "{}{}{}{}", 
                    format!("{:-^1$}|", "-", 41),
                    format!("{:-^1$}|", "-", 32),
                    format!("{:-^1$}|", "-", 12),
                    format!("{:-^1$}|", "-", 34))
    }
}


fn run_monitor(id: String, timeout: u32) {
    let mut names = NameProvider::new();
    let prices = Prices::new();
    while let Some(package) = gw::get_package(&id) {
        info!("Received response from API");
        if let Some(report) = Report::new(package, &mut names) {
            info!("Report ready to display");
            print!("{}", report);
        } else {
            thread::sleep(std::time::Duration::from_secs(timeout.into()));
        }
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
