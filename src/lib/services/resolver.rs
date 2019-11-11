use crate::api;
use crate::services::{AppContext, Command, Message, Api};

pub fn run(context: actix_web::web::Data<AppContext>) {
    info!("Started");
    loop {
        if let Some(Command::Quit) = context.commands.pop() {
            context.commands.push(Command::Quit);
            context.resolver.push(Message::Ping); // ping other threads if any
            info!("received Command::Quit");
            break;
        }
        if let Some(msg) = context.resolver.pop() {
            match msg {
                Message::Receive(cmd) => {
                    match cmd {
                        Api::Object(id) =>{
                            if let Some(object) = api::object::Object::new(&id) {
                                info!("{:?}. Queue length {}", object, context.resolver.len());
                                context.database.push(Message::SaveObject(object));
                            }
                        },
                        Api::System(id) =>{
                            if let Some(object) = api::system::System::new(&id) {
                                info!("{:?}. Queue length {}", object, context.resolver.len());
                                //context.database.push(Message::SaveObject(object));
                                warn!("Save System not impl");
                            }
                        },
                        Api::Stargate(id) =>{
                            if let Some(object) = api::stargate::Stargate::new(&id) {
                                info!("{:?}. Queue length {}", object, context.resolver.len());
                                //context.database.push(Message::SaveObject(object));
                                warn!("Save Stargate not impl");
                            }
                        },
                        Api::Constellation(id) =>{
                            if let Some(object) = api::constellation::Constellation::new(&id) {
                                info!("{:?}. Queue length {}", object, context.resolver.len());
                                //context.database.push(Message::SaveObject(object));
                                warn!("Save Constellation not impl");
                            }
                        },
                    }
                },
                message => {
                    warn!("received: {:?} ", message);
                }
            }
        }
    }
    info!("Ended");
}
