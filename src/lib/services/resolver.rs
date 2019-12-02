use crate::api;
use crate::services::{AppContext, Command, Message, Api, Model, Category};
use std::collections::HashSet;

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
                                info!("Received Object({}) queue length: {}", id, context.resolver.len());
                                context.database.push(Message::Save(Model::Object(object)));
                            } else {
                                warn!("Failed to resolve Object({})", id);
                            }
                        },
                        Api::System(id) =>{
                            if let Some(object) = api::system::System::new(&id) {
                                info!("Received System({}) queue length: {}", id, context.resolver.len());
                                context.database.push(Message::Check(Category::Constellation(object.constellation_id)));
                                context.database.push(Message::Check(Category::Object(object.constellation_id)));
                                if let Some(gates) = &object.stargates {
                                    for id in gates {
                                        context.database.push(Message::Check(Category::Stargate(*id)));
                                    }
                                }
                                // Enqueue system name query
                                context.database.push(Message::Check(Category::Object(object.system_id)));
                                context.database.push(Message::Save(Model::System(object)));
                            } else {
                                warn!("Failed to resolve System({})", id);
                            }
                        },
                        Api::Stargate(id) =>{
                            if let Some(object) = api::stargate::Stargate::new(&id) {
                                info!("Received Stargate({}) queue length: {}", id, context.resolver.len());
                                context.database.push(Message::Check(Category::Stargate(object.destination.stargate_id)));
                                context.database.push(Message::Check(Category::System(object.destination.system_id)));
                                context.database.push(Message::Save(Model::Stargate(object)));
                            } else {
                                warn!("Failed to resolve Stargate({})", id);
                            }
                        },
                        Api::Constellation(id) =>{
                            if let Some(object) = api::constellation::Constellation::new(&id) {
                                info!("Received Constellation({}) queue length: {}", id, context.resolver.len());
                                context.database.push(Message::Check(Category::Object(object.region_id)));
                                context.database.push(Message::Save(Model::Constellation(object)));
                            } else {
                                warn!("Failed to resolve Constellation({})", id);
                            }
                        },
                    };
                },
                message => {
                    warn!("received: {:?} ", message);
                }
            }
        }
    }
    info!("Ended");
}
