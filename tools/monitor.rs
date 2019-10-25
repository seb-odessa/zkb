
use lib::api::*;
use lib::api::system::*;
use std::thread;

fn run_monitor(id: String, timeout: u32) {
    loop {
        while let Some(package) = gw::get_package(&id) {
            if let Some(content) = package.content {
                let killmail = content.killmail;
                println!("{} {} {:>12}/{:<12} {}",
                    killmail.killmail_time.to_string(),
                    killmail.href(),
                    killmail.get_dropped_sum(),
                    killmail.get_total_sum(),
                    System::new(&killmail.solar_system_id).map(|s| s.get_full_name()).unwrap_or_default()
                );
            }
        }
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
