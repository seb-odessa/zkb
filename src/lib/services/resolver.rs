use crate::api;
use crate::services::{AppContext, Command, Message, Api, Model, Category};

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
                                info!("Received Object({})", id);
                                context.database.push(Message::Save(Model::Object(object)));
                            } else {
                                warn!("Failed to resolve Object({})", id);
                            }
                        },
                        Api::System(id) =>{
                            if let Some(object) = api::system::System::new(&id) {
                                info!("Received System({})", id);
                                context.database.push(Message::Check(Category::Constellation(object.constellation_id)));
                                context.database.push(Message::Check(Category::Object(object.constellation_id)));
                                if let Some(gates) = &object.stargates {
                                    for id in gates {
                                        context.database.push(Message::Check(Category::Stargate(*id)));
                                    }
                                }
                                context.database.push(Message::Check(Category::Object(object.system_id)));
                                context.database.push(Message::Save(Model::System(object)));
                            } else {
                                warn!("Failed to resolve System({})", id);
                            }

                        },
                        Api::Stargate(id) =>{
                            if let Some(object) = api::stargate::Stargate::new(&id) {
                                info!("Received Stargate({})", id);
                                context.database.push(Message::Save(Model::Stargate(object)));
                            } else {
                                warn!("Failed to resolve Stargate({})", id);
                            }
                        },
                        Api::Constellation(id) =>{
                            if let Some(object) = api::constellation::Constellation::new(&id) {
                                info!("Received Constellation({})", id);
                                context.database.push(Message::Save(Model::Constellation(object)));
                            } else {
                                warn!("Failed to resolve Constellation({})", id);
                            }
                        },
                    };
                    info!("resolver queue length: {}", context.resolver.len());
                },
                message => {
                    warn!("received: {:?} ", message);
                }
            }
        }
    }
    info!("Ended");
}
