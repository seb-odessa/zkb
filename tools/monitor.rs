#[macro_use]
extern crate log;

use lib::api;
use std::thread;

fn run_monitor(id: String, timeout: u32) {
    while let Some(response) = api::gw::get_package(&id) {
        info!("Received response from API");
        if let Some(content) = response.package {
            if content.zkb.npc {
                println!("{} {}", content.id, content.zkb.href);
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
