use crate::api::object::Object;
use crate::services::{AppContext, Command, Message};

pub fn run(context: actix_web::web::Data<AppContext>) {
    info!("Started");
    loop {
        if let Some(Command::Quit) = context.commands.pop() {
            info!("received Command::Quit");
            context.commands.push(Command::Quit);
            break;
        }
        if let Some(msg) = context.unresolved.pop() {
            match msg {
                Message::Resolve((id, first)) => {
                    if let Some(object) = Object::new(&id) {
                        info!("received {} '{}' '{}'. Queue length {}",
                                    object.id,
                                    object.name,
                                    object.category,
                                    context.unresolved.len());
                        context.saver_queue.push(Message::Object(object));
                    } else {
                        warn!("failed to query object id {}. Queue length {}",
                                    id,
                                    context.unresolved.len());
                        if first {
                            // try again if it was first time
                            context.unresolved.push(Message::Resolve((id, false)));
                        }
                    }
                },
                _ => {
                    warn!("Unexpected message");
                }
            }
        }
    }
    info!("Ended");
}
