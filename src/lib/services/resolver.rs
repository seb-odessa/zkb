use crate::api::object::Object;
use crate::services::{AppContext, Command, Message};

pub fn run(context: actix_web::web::Data<AppContext>) {
    info!("Started");
    loop {
        if let Some(Command::Quit) = context.commands.pop() {
            context.commands.push(Command::Quit);
            context.unresolved.push(Message::Ping);
            info!("received Command::Quit");
            break;
        }
        if let Some(msg) = context.unresolved.pop() {
            match msg {
                Message::Resolve((id, first)) => {
                    if let Some(object) = Object::new(&id) {
                        info!("received {:?}. Queue length {}", object, context.unresolved.len());
                    } else {
                        warn!("failed to query object id {}. Queue length {}", id, context.unresolved.len());
                        if first {
                            // try again if it was first time
                            context.unresolved.push(Message::Resolve((id, false)));
                        }
                    }
                },
                Message::Ping => {
                    info!("received Message::Ping");
                },
                message => {
                    warn!("received unexpected message: {:?} ", message);
                }
            }
        }
    }
    info!("Ended");
}
