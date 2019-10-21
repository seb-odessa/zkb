#[macro_use]
extern crate log;
extern crate separator;
extern crate ansi_term;

use separator::Separatable;
use std::fmt;

use lib::api::*;
use lib::api::system::*;
use lib::provider;
use std::thread;



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

fn calc_dropped_items(items: &Option<Vec<killmail::Item>>) -> f32 {
    items.as_ref().map_or(0.0, |items|{
        items.iter().map(|item| {
            let mut sum = 0.0;
            if let Some(quantity) = item.quantity_dropped {
                sum = sum + quantity as f32 * provider::get_avg_price(Some(item.item_type_id)).unwrap_or(0.0);
            }
            sum = sum + calc_dropped_items(&item.items);

            return sum;
        }).fold(0.0, |acc, x| acc + x)
    })
}

#[derive(Debug)]
struct Report {
    pub time: TimeRequired,
    pub zkb_url: String,
    pub system_id: i32,
    pub system_name: String,
    pub total_value: u64,
    pub dropped_value: u64,
    pub victim: Participant,
    pub attackers: Vec<Participant>,

}
impl Report {
    pub fn new(package: zkb::Package) -> Option<Self> {
        if let Some(content) = package.content {
            Some(
                Self {
                    time: content.killmail.killmail_time,
                    zkb_url: content.zkb_url(),
                    system_id: content.killmail.solar_system_id,
                    system_name: provider::get_name(Some(content.killmail.solar_system_id)),
                    total_value: content.zkb.total_value as u64,
                    dropped_value: calc_dropped_items(&content.killmail.victim.items) as u64,
                    victim: Participant::new(
                        provider::get_name(content.killmail.victim.character_id),
                        provider::get_name(Some(content.killmail.victim.ship_type_id)),
                        content.killmail.victim.damage_taken),
                    attackers: content.killmail.attackers.iter()
                        .map(|a|
                            Participant::new(
                                provider::get_name(a.character_id),
                                provider::get_name(a.ship_type_id),
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
            self.system_name,
            self.total_value.separated_string(),
            self.dropped_value.separated_string())?;
        writeln!(f, "{:>30} | {:>40} | {:>20} |",
            self.victim.name,
            self.victim.ship,
            self.victim.damage)?;
        for attacker in &self.attackers {
            writeln!(f, "{:<30} | {:>40} | {:>20} |", attacker.name, attacker.ship, attacker.damage)?;
        }
        write!(f, "{}{}{}{}",
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
                println!("{}", report);
            }
        }
        warn!("Perform sleep {} sec ", timeout);
        thread::sleep(std::time::Duration::from_secs(timeout.into()));
    }
}

fn main() {
    env_logger::init();
    //ansi_term::enable_ansi_support();
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
