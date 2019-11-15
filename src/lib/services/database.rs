use crate::api;
use crate::services::*;
use crate::models;
use crate::reports;
use crate::services::{AppContext, Command, Message, Category, Report};
use models::Connection;

fn enqueue_check(queue: &Queue, id: &i32) {
    queue.push(Message::Check(Category::Object(*id)));
}

fn try_enqueue_check(queue: &Queue, id: &Option<i32>) {
    if let Some(id) = id {
        enqueue_check(queue, id);
    }
}

fn handle_killmail(queue: &Queue, killmail: &api::Killmail) {
    enqueue_check(queue, &killmail.solar_system_id);
    try_enqueue_check(queue, &killmail.moon_id);
    enqueue_check(queue, &killmail.victim.ship_type_id);
    try_enqueue_check(queue, &killmail.victim.character_id);
    try_enqueue_check(queue, &killmail.victim.corporation_id);
    try_enqueue_check(queue, &killmail.victim.alliance_id);
    try_enqueue_check(queue, &killmail.victim.faction_id);
    for attacker in &killmail.attackers {
        try_enqueue_check(queue, &attacker.ship_type_id);
        try_enqueue_check(queue, &attacker.character_id);
        try_enqueue_check(queue, &attacker.corporation_id);
        try_enqueue_check(queue, &attacker.alliance_id);
        try_enqueue_check(queue, &attacker.faction_id);
        try_enqueue_check(queue, &attacker.weapon_type_id);
    }
    if let Some(items) = &killmail.victim.items {
        for item in items {
            enqueue_check(queue, &item.item_type_id);
            if let Some(items) = &item.items {
                for item in items {
                    enqueue_check(queue, &item.item_type_id);
                }
            }
        }
    }

    //@todo handle items as well
}

pub fn run(conn: Connection, context: actix_web::web::Data<AppContext>) {
    info!("Started");

    loop {
        if let Some(Command::Quit) = context.commands.pop() {
            context.commands.push(Command::Quit);
            info!("received Command::Quit");
            break;
        }
        if let Some(msg) = context.database.pop() {
            match msg {
                Message::Save(model) => {
                    match model {
                        Model::Killmail(killmail) =>{
                            if let Err(err) = models::KillmailsApi::save(&conn, &killmail) {
                                warn!("was not able to save killmail: {}", err);
                            } else {
                                info!("Killmail({}) saved, queue length: {}", killmail.killmail_id, context.database.len());
                            }

                            handle_killmail(&context.database, &killmail);
                            context.database.push(Message::Check(Category::System(killmail.solar_system_id)));
                        },
                        Model::Object(object) => {
                            if let Err(err) = models::ObjectsApi::save(&conn, &object) {
                                warn!("was not able to save object: {}", err);
                            } else {
                                info!("Object({}) saved, queue length: {}", object.id, context.database.len());
                            }
                        },
                        Model::System(object) => {
                            if let Err(err) = models::system::System::save(&conn, &object) {
                                warn!("was not able to save system: {}", err);
                            } else {
                                info!("System({}) saved, queue length: {}", object.system_id, context.database.len());
                            }
                        },
                        Model::Constellation(object) => {
                            if let Err(err) = models::constellation::Constellation::save(&conn, &object) {
                                warn!("was not able to save constellation: {}", err);
                            } else {
                                info!("Constellation({}) saved, queue length: {}", object.constellation_id, context.database.len());
                            }
                        },
                        Model::Stargate(object) => {
                            if let Err(err) = models::stargate::Stargate::save(&conn, &object) {
                                warn!("was not able to save stargate: {}", err);
                            } else {
                                info!("Stargate({}) saved, queue length: {}", object.stargate_id, context.database.len());
                            }
                        },
                    };
                },
                Message::Find((msg_id, ref category)) => {
                    match category {
                        Category::Killmail(id) => {
                            match models::killmail::KillmailNamed::load(&conn, &id) {
                                Ok(object) => {
                                    info!("loaded killmail {} queue length: {}", id, context.database.len());
                                    context.responses.push(Message::Report((msg_id, Report::Killmail(object))));
                                },
                                Err(e) => {
                                    warn!("was not able to load killmail: {}", e);
                                    context.responses.push(Message::Report((msg_id, Report::NotFoundId(*id))));
                                }
                            }
                        },
                        Category::Victim(id) => {
                            match models::victim::VictimNamed::load(&conn, &id) {
                                Ok(object) => {
                                    info!("loaded victim {} queue length: {}", id, context.database.len());
                                    context.responses.push(Message::Report((msg_id, Report::Victim(object))));
                                },
                                Err(e) => {
                                    warn!("was not able to load killmail: {}", e);
                                    context.responses.push(Message::Report((msg_id, Report::NotFoundId(*id))));
                                }
                            }
                        },
                        Category::History((system_id, minutes)) => {
                            match models::killmail::KillmailNamed::load_history(&conn, &system_id, &minutes) {
                                Ok(object) => {
                                    info!("loaded {} minutes history for system {} queue length: {}", minutes, system_id, context.database.len());
                                    context.responses.push(Message::Report((msg_id, Report::History(object))));
                                },
                                Err(e) => {
                                    warn!("was not able to load history: {}", e);
                                    context.responses.push(Message::Report((msg_id, Report::QueryFailed(e.to_string()))));
                                }
                            }
                        },
                        Category::ObjectDesc((category, name)) => {
                            match models::category::Category::find(&conn, &category) {
                                Ok(categories) => {
                                    if categories.is_empty() {
                                        warn!("Failed to find category {}", category);
                                        context.responses.push(Message::Report((msg_id, Report::NotFoundName(category.clone()))));
                                    } else if categories.len() > 1 {
                                        warn!("Category name pattern is not unique {}", category);
                                        context.responses.push(Message::Report((msg_id, Report::NotUniqName(category.clone()))));
                                    } else {
                                        if let Ok(objects) = models::object::Object::find(&conn, &categories[0], &name) {
                                            if objects.is_empty() {
                                                warn!("Failed to find object {}", name);
                                                context.responses.push(Message::Report((msg_id, Report::NotFoundName(name.clone()))));
                                            } else if objects.len() > 1 {
                                                warn!("Object name pattern is not unique {}", name);
                                                context.responses.push(Message::Report((msg_id, Report::NotUniqName(name.clone()))));
                                            } else {
                                                context.responses.push(Message::Report((msg_id, Report::Id(objects[0]))));
                                            }
                                        }
                                    }
                                },
                                Err(e) => {
                                    let error = e.to_string();
                                    warn!("Failed to query ({}, {}, {}): {}", msg_id, category, name, &error);
                                    context.responses.push(Message::Report((msg_id, Report::QueryFailed(error))));
                                }
                            }
                        },
                        category => {
                            warn!("Unexpected category for Find {:?}", category);
                        }
                    }
                },
                Message::Check(category) => {
                    match category {
                        Category::Object(id) =>{
                            if !models::ObjectsApi::exist(&conn, &id) {
                                context.resolver.push(Message::Receive(Api::Object(id)));
                            }
                        },
                        Category::System(id) => {
//                            if !models::system::System::exist(&conn, &id) {
                                context.resolver.push(Message::Receive(Api::System(id)));
//                            }
                        }
                        Category::Constellation(id) => {
                            if !models::constellation::Constellation::exist(&conn, &id) {
                                context.resolver.push(Message::Receive(Api::Constellation(id)));
                            }
                        },
                        Category::Stargate(id) => {
                            if !models::stargate::Stargate::exist(&conn, &id) {
                                context.resolver.push(Message::Receive(Api::Stargate(id)));
                            }
                        },
                        model => {
                            warn!("Exist not implemented for {:?}", model)
                        }
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
