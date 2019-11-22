use crate::api;
use crate::services::*;
use crate::models;
use crate::services::{AppContext, Command, Message, Category, Report};
use models::Connection;
use std::collections::HashSet;

fn enqueue_check(queue: &Queue, id: &i32) {
    queue.push(Message::Check(Category::Object(*id)));
}

fn try_enqueue_check(queue: &Queue, id: &Option<i32>) {
    if let Some(id) = id {
        enqueue_check(queue, id);
    }
}

fn get_name_if_none(queue: &Queue, name: &Option<String>, id: i32) {
    if name.is_none() {
        queue.push(Message::Receive(Api::Object(id)));
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
}

pub fn run(conn: Connection, context: actix_web::web::Data<AppContext>) {
    info!("Started");
    let mut known = HashSet::new();
    let mut objects = HashSet::new();
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
                                info!("Object {} - '{}' saved, queue length: {}", object.id, &object.name, context.database.len());
                            }
                        },
                        Model::System(object) => {
                            if let Err(err) = models::system::System::save(&conn, &object) {
                                warn!("was not able to save system: {}", err);
                            } else {
                                info!("System {} - '{}' saved, queue length: {}", object.system_id, &object.name, context.database.len());
                            }
                        },
                        Model::Constellation(object) => {
                            if let Err(err) = models::constellation::Constellation::save(&conn, &object) {
                                warn!("was not able to save constellation: {}", err);
                            } else {
                                info!("Constellation {} - '{}' saved, queue length: {}", object.constellation_id, &object.name, context.database.len());
                            }
                        },
                        Model::Stargate(object) => {
                            if let Err(err) = models::stargate::Stargate::save(&conn, &object) {
                                warn!("was not able to save stargate: {}", err);
                            } else {
                                info!("Stargate {} - '{}' saved, queue length: {}", object.stargate_id, &object.name, context.database.len());
                            }
                        },
                    };
                },
                Message::Find((msg_id, ref category)) => {
                    match category {
                        Category::System(id) => {
                            match models::system::SystemNamed::load(&conn, &id) {
                                Ok(object) => {
                                    info!("loaded system {} queue length: {}", id, context.database.len());
                                    get_name_if_none(&context.resolver, &object.system_name, object.system_id);
                                    get_name_if_none(&context.resolver, &object.constellation_name, object.constellation_id);
                                    get_name_if_none(&context.resolver, &object.region_name, object.region_id);
                                    context.responses.push(Message::Report((msg_id, Report::System(object))));
                                },
                                Err(e) => {
                                    warn!("was not able to load system: {}", e);
                                    context.database.push(Message::Check(Category::System(*id)));
                                    context.responses.push(Message::Report((msg_id, Report::NotFoundId(*id))));
                                }
                            }
                        },
                        Category::Systems((area, filter)) => {
                            let query = match filter {
                                Filter::WithJovianObservatoryOnly => models::system::QuerySystem::WithJovianObservatoryOnly,
                                Filter::Any => models::system::QuerySystem::Any
                            };
                            let systems = match area {
                                Area::System(id) => models::system::SystemNamed::load(&conn, &id).and_then(|system| Ok(vec![system])),
                                Area::Region(id) => models::system::SystemNamed::load_from_region(&conn, &id, query),
                                Area::Constellation(id) => models::system::SystemNamed::load_from_constellation(&conn, &id, query),
                            };
                            match systems {
                                Ok(systems) => {
                                    info!("loaded {} systems, queue length: {}", systems.len(), context.database.len());
                                    context.responses.push(Message::Report((msg_id, Report::Systems(systems))));
                                },
                                Err(e) => {
                                    warn!("was not able to load systems: {}", e);
                                    context.responses.push(Message::Report((msg_id, Report::QueryFailed(e.to_string()))));
                                }
                            }
                        },
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
                                    info!("loaded victim for KM {} queue length: {}", id, context.database.len());
                                    context.responses.push(Message::Report((msg_id, Report::Victim(object))));
                                },
                                Err(e) => {
                                    warn!("was not able to load killmail: {}", e);
                                    context.responses.push(Message::Report((msg_id, Report::NotFoundId(*id))));
                                }
                            }
                        },
                        Category::Attakers(id) => {
                            match models::attacker::AttackerNamed::load(&conn, &id) {
                                Ok(object) => {
                                    info!("loaded attakers for KM {} queue length: {}", id, context.database.len());
                                    context.responses.push(Message::Report((msg_id, Report::Attakers(object))));
                                },
                                Err(e) => {
                                    warn!("was not able to load killmail: {}", e);
                                    context.responses.push(Message::Report((msg_id, Report::NotFoundId(*id))));
                                }
                            }
                        },
                        Category::History((area, minutes)) => {
                            let history = match area {
                                Area::System(id) => models::killmail::KillmailNamed::load_system_history(&conn, &id, &minutes),
                                Area::Region(id) => models::killmail::KillmailNamed::load_region_history(&conn, &id, &minutes),
                                Area::Constellation(id) => models::killmail::KillmailNamed::load_constellation_history(&conn, &id, &minutes)
                            };
                            match history {
                                Ok(killmails) => {
                                    info!("loaded {} history records for last {} minutes, queue length: {}", killmails.len(), minutes, context.database.len());
                                    context.responses.push(Message::Report((msg_id, Report::History(killmails))));
                                },
                                Err(e) => {
                                    warn!("was not able to load history: {}", e);
                                    context.responses.push(Message::Report((msg_id, Report::QueryFailed(e.to_string()))));
                                }
                            }
                        },
                        Category::HistoryCount((area, minutes)) => {
                            let count = match area {
                                Area::System(id) => {
                                    models::killmail::KillmailNamed::load_system_history_count(&conn, &id, &minutes)
                                },
                                Area::Region(id) => {
                                    models::killmail::KillmailNamed::load_region_history_count(&conn, &id, &minutes)
                                },
                                Area::Constellation(id) => {
                                    models::killmail::KillmailNamed::load_constellation_history_count(&conn, &id, &minutes)
                                },
                            };
                            match count {
                                Ok(count) => {
                                    info!("loaded history count for last {} minutes, queue length: {}", minutes, context.database.len());
                                    context.responses.push(Message::Report((msg_id, Report::HistoryCount(count as i32))));
                                },
                                Err(e) => {
                                    warn!("was not able to load history count: {}", e);
                                    context.responses.push(Message::Report((msg_id, Report::QueryFailed(e.to_string()))));
                                }
                            }
                        }
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
                        Category::Neighbors(area) => {
                            match area {
                                Area::System(id) => {
                                    match models::system::SystemNeighbors::load(&conn, &id) {
                                        Ok(neighbors) => {
                                            info!("loaded {} neighbors, queue length: {}", neighbors.len(), context.database.len());
                                            context.responses.push(Message::Report((msg_id, Report::SystemNeighbors(neighbors))));
                                        },
                                        Err(e) => {
                                            warn!("was not able to load neighbors: {}", e);
                                            context.responses.push(Message::Report((msg_id, Report::QueryFailed(e.to_string()))));
                                        }
                                    }                                },
                                Area::Region(id) => {
                                    match models::region::RegionNeighbors::load(&conn, &id) {
                                        Ok(neighbors) => {
                                            info!("loaded {} neighbors, queue length: {}", neighbors.len(), context.database.len());
                                            context.responses.push(Message::Report((msg_id, Report::RegionNeighbors(neighbors))));
                                        },
                                        Err(e) => {
                                            warn!("was not able to load neighbors: {}", e);
                                            context.responses.push(Message::Report((msg_id, Report::QueryFailed(e.to_string()))));
                                        }
                                    }
                                },
                                Area::Constellation(id) => {
                                    match models::constellation::ConstellationNeighbors::load(&conn, &id) {
                                        Ok(neighbors) => {
                                            info!("loaded {} neighbors, queue length: {}", neighbors.len(), context.database.len());
                                            context.responses.push(Message::Report((msg_id, Report::ConstellationNeighbors(neighbors))));
                                        },
                                        Err(e) => {
                                            warn!("was not able to load neighbors: {}", e);
                                            context.responses.push(Message::Report((msg_id, Report::QueryFailed(e.to_string()))));
                                        }
                                    }
                                },
                            };
                        },
                        category => {
                            warn!("Unexpected category for Find {:?}", category);
                        }
                    }
                },
                Message::Check(category) => {
                    match category {
                        Category::Object(id) => {
                            if objects.contains(&id) && !models::ObjectsApi::exist(&conn, &id) {
                                context.resolver.push(Message::Receive(Api::Object(id)));
                                objects.insert(id);
                            }
                        },
                        Category::System(id) => {
                            if !known.contains(&id)
                             && !models::system::System::exist(&conn, &id)
                            {
                                context.resolver.push(Message::Receive(Api::System(id)));
                                known.insert(id);
                            }
                        }
                        Category::Constellation(id) => {
                            if !known.contains(&id) && !models::constellation::Constellation::exist(&conn, &id)
                            {
                                context.resolver.push(Message::Receive(Api::Constellation(id)));
                                known.insert(id);
                            }
                        },
                        Category::Stargate(id) => {
                            if !known.contains(&id)
                            && !models::stargate::Stargate::exist(&conn, &id)
                            {
                                context.resolver.push(Message::Receive(Api::Stargate(id)));
                                known.insert(id);
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
