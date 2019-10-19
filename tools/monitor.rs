#[macro_use]
extern crate log;
extern crate separator;
use separator::Separatable;

use lib::api;
use lib::api::system::*;
use lib::api::object::Object;
use std::thread;
use std::convert::TryFrom;
use std::collections::HashMap;

struct NameCollector {
    names: HashMap<i32, String>,
}
impl NameCollector {
    pub fn new() -> Self {
        Self {names: HashMap::new() }
    }

    pub fn get(&mut self, id: Option<i32>) -> String {
        if let Some(id) = id {
            self.names.entry(id).or_insert(Object::try_from(id).ok().unwrap_or_default().name);
            self.names.get(&id).map(|name| name.clone()).unwrap_or_default()
        } else {
            String::new()
        }
    }
}

fn run_monitor(id: String, timeout: u32) {
    let mut names = NameCollector::new();
    while let Some(response) = api::gw::get_package(&id) {
        info!("Received response from API");
        if let Some(content) = response.package {
            let total_value = content.zkb.total_value as u64;
            let time = content.killmail.killmail_time.to_string();
            let system_id = content.killmail.solar_system_id;
            let jita = route(JITA_ID, system_id).len();
            let amarr = route(AMARR_ID, system_id).len();
            let dodixie = route(DODIXIE_ID, system_id).len();
            let rens = route(RENS_ID, system_id).len();
            let hek = route(HEK_ID, system_id).len();
            if content.zkb.npc { //&& total_value > 50_000_000 {
                let victim = content.killmail.victim.clone();
                let victim_char = names.get(victim.character_id);
                let victim_ship = names.get(Some(victim.ship_type_id));
                let system = names.get(Some(system_id));
                println!("{:>40} | {:>30} | {:>10} | J-{:>02} | A-{:>02} | D-{:>02} | R-{:>02} | H-{:>02} |", 
                    content.zkb_url(), time, system, jita, amarr, dodixie, rens, hek);
                println!("{:>40} | {:>30} | {:>10} |", "", 
                    total_value.separated_string(),"");
                println!("{:>40} | {:>30} | {:>10} |", 
                    victim_char, victim_ship, victim.damage_taken);

                let mut attackers = content.killmail.attackers;
                while let Some(attacker) = attackers.pop() {
                    let attacker_char = names.get(attacker.character_id);
                    let attacker_ship = names.get(attacker.ship_type_id);
                    println!("{:>40} | {:>30} | {:>10} |", 
                        attacker_char, 
                        attacker_ship, 
                        attacker.damage_done);
                }
                println!("{}{}{}{}", 
                    format!("{:-^1$}|", "-", 41),
                    format!("{:-^1$}|", "-", 32),
                    format!("{:-^1$}|", "-", 12),
                    format!("{:-^1$}|", "-", 34)); 
            }
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
