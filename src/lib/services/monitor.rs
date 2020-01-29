use crate::api;
use crate::services::{AppContext, Command, Message, Model};

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
                info!("{} {} {} {}",
                    killmail.killmail_time.time().to_string(),
                    killmail.killmail_time.date().to_string(),
                    killmail.href(),
                    killmail.get_system_full_name()
                );
                if let Some(allowed) = &context.allowed {
                    if *allowed < killmail.killmail_time {
                        context.database.push(Message::Save(Model::Killmail(killmail)));
                    } else {
                        warn!("Killmail {} is too old {} will skipped.", killmail.killmail_id, killmail.killmail_time);
                    }
                }
            } else {
                let timeout = context.timeout.into();
                info!("monitor will suspended {} sec", timeout);
                Parker::new().park_timeout(std::time::Duration::from_secs(timeout))
            }
        } else {
            let timeout = context.timeout.into();
            info!("CCP API down? Will suspended {} sec", timeout);
            Parker::new().park_timeout(std::time::Duration::from_secs(timeout))
        }
    }
    info!("Ended");
}