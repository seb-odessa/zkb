use crate::api;
use crate::services::{AppContext, Command, Message};

use crossbeam_utils::sync::Parker;

pub fn run(context: actix_web::web::Data<AppContext>) {
    info!("Started");
    let mut enabled = true;
    while enabled {
        while let Some(package) = api::gw::get_package(&context.client) {
            if let Some(Command::Quit) = context.commands.pop() {
                context.commands.push(Command::Quit);
                context.saver.push(Message::Ping);
                info!("received Command::Quit");            
                enabled = false;
                break;
            }
            if let Some(content) = package.content {
                let killmail = content.killmail;
                info!("monitor {} {} {:>12}/{:>12} {}",
                    killmail.killmail_time.time().to_string(),
                    killmail.href(),
                    killmail.get_dropped_sum(),
                    killmail.get_total_sum(),
                    killmail.get_system_full_name()
                );
                context.saver.push(Message::Killmail(killmail));
            }
        }
        if !enabled {
            break;
        }
        let timeout = context.timeout.into();
        info!("monitor will suspended {} sec", timeout);
        Parker::new().park_timeout(std::time::Duration::from_secs(timeout))
    }
    info!("Ended");
}