use crate::api;
use crate::services::{AppContext, Command, Message};

use crossbeam_utils::sync::Parker;

pub fn run(context: actix_web::web::Data<AppContext>) {
    info!("Started");
    loop {
        if let Some(Command::Quit) = context.commands.pop() {
            context.commands.push(Command::Quit);
            info!("received Command::Quit");            
            break;
        }
        if let Some(package) = api::gw::get_package(&context.client) {
            if let Some(content) = package.content {
                let killmail = content.killmail;
                info!("{} {} {}/{} {}",
                    killmail.killmail_time.time().to_string(),
                    killmail.href(),
                    killmail.get_dropped_sum(),
                    killmail.get_total_sum(),
                    killmail.get_system_full_name()
                );
                context.database.push(Message::SaveKill(killmail));
            } else {
                let timeout = context.timeout.into();
                info!("monitor will suspended {} sec", timeout);
                Parker::new().park_timeout(std::time::Duration::from_secs(timeout))
            }
        }
    }
    info!("Ended");
}