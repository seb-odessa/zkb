use crate::api;
use crate::services::*;
use crate::models;
use crate::services::{AppContext, Command, Message, Category, Report};
use models::Connection;
//use std::collections::HashSet;

fn enqueue_check(queue: &Queue, id: &i32) {
    queue.push(Message::Check(Category::Object(*id)));
}

fn try_enqueue_check(queue: &Queue, id: &Option<i32>) {
    if let Some(id) = id {
        enqueue_check(queue, id);
    }
}

pub fn get_name_if_none(queue: &Queue, name: &Option<String>, id: i32) {
    if name.is_none() {
        info!("Queue Object with id {}", id);
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
//    let mut known = HashSet::new();
//    let mut objects = HashSet::new();
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
                        Model::Observatory(id) => {
                            if let Err(err) = models::observatory::Observatory::save(&conn, &id) {
                                warn!("was not able to save observatory: {}", err);
                            } else {
                                info!("Observatory in system {} saved, queue length: {}", id, context.database.len());
                            }
                        },
                    };
                },
                Message::Delete(model) => {
                    match model {
                        Model::Observatory(id) => {
                            if let Err(err) = models::observatory::Observatory::delete(&conn, &id) {
                                warn!("was not able to delete observatory: {}", err);
                            } else {
                                info!("Observatory in system {} deleted, queue length: {}", id, context.database.len());
                            }
                        },
                        model => warn!("Delete operation is not implemented for {:?}", model)
                    }
                }
                Message::Find((msg_id, ref category)) => {
                    match category {
                        Category::Object(id) => {
                            match models::object::Object::load(&conn, &id) {
                                Ok(object) => {
                                    info!("loaded object {} queue length: {}", id, context.database.len());
                                    context.responses.push(Message::Report((msg_id, Report::Object(object))));
                                },
                                Err(e) => {
                                    warn!("was not able to load object: {}", e);
                                    context.database.push(Message::Check(Category::Object(*id)));
                                    context.responses.push(Message::Report((msg_id, Report::NotFoundId(*id))));
                                }
                            }
                        }
                        Category::System(id) => {
                            match models::system::SystemNamed::load(&conn, &id) {
                                Ok(system) => {
                                    info!("loaded system {} queue length: {}", id, context.database.len());
                                    get_name_if_none(&context.resolver, &system.system_name, system.system_id);
                                    get_name_if_none(&context.resolver, &system.constellation_name, system.constellation_id);
                                    get_name_if_none(&context.resolver, &system.region_name, system.region_id);
                                    context.responses.push(Message::Report((msg_id, Report::System(system))));
                                },
                                Err(e) => {
                                    warn!("was not able to load system: {}", e);
                                    context.database.push(Message::Check(Category::System(*id)));
                                    context.responses.push(Message::Report((msg_id, Report::NotFoundId(*id))));
                                }
                            }
                        },
                        Category::Region(id) => {
                            match models::region::RegionNamed::load(&conn, &id) {
                                Ok(region) => {
                                    info!("loaded region {} queue length: {}", id, context.database.len());
                                    get_name_if_none(&context.resolver, &region.region_name, region.region_id);
                                    context.responses.push(Message::Report((msg_id, Report::Region(region))));
                                },
                                Err(e) => {
                                    warn!("was not able to load region: {}", e);
                                    context.database.push(Message::Check(Category::Region(*id)));
                                    context.responses.push(Message::Report((msg_id, Report::NotFoundId(*id))));
                                }
                            }
                        }
                        Category::Constellation(id) => {
                            match models::constellation::ConstellationNamed::load(&conn, &id) {
                                Ok(constellation) => {
                                    info!("loaded constellation {} queue length: {}", id, context.database.len());
                                    get_name_if_none(&context.resolver, &constellation.region_name, constellation.region_id);
                                    get_name_if_none(&context.resolver, &constellation.constellation_name, constellation.constellation_id);
                                    context.responses.push(Message::Report((msg_id, Report::Constellation(constellation))));
                                },
                                Err(e) => {
                                    warn!("was not able to load constellation: {}", e);
                                    context.database.push(Message::Check(Category::Constellation(*id)));
                                    context.responses.push(Message::Report((msg_id, Report::NotFoundId(*id))));
                                }
                            }
                        },
                        Category::Systems((area, filter)) => {
                            use models::system::SystemNamed;
                            let systems = match area {
                                Area::System(id) => SystemNamed::load(&conn, &id).and_then(|system| Ok(vec![system])),
                                Area::Constellation(id) => SystemNamed::load_from_constellation(&conn, &id, filter),
                                Area::Region(id) => SystemNamed::load_from_region(&conn, &id, filter),
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
                        Category::Constellations(area) => {
                            use models::constellation::ConstellationNamed;
                            let constellations = match area {
                                Area::System(_) => Ok(Vec::new()),
                                Area::Constellation(id) => ConstellationNamed::load(&conn, &id).and_then(|constellation| Ok(vec![constellation])),
                                Area::Region(id) => ConstellationNamed::load_from_region(&conn, &id),
                            };
                            match constellations {
                                Ok(constellations) => {
                                    info!("loaded {} constellations, queue length: {}", constellations.len(), context.database.len());
                                    context.responses.push(Message::Report((msg_id, Report::Constellations(constellations))));
                                },
                                Err(e) => {
                                    warn!("was not able to load constellations: {}", e);
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
                                    get_name_if_none(&context.resolver, &object.ship_name, object.ship_id);
                                    if let Some(id) = object.character_id {
                                        get_name_if_none(&context.resolver, &object.character_name, id);
                                    }
                                    if let Some(id) = object.corporation_id {
                                        get_name_if_none(&context.resolver, &object.corporation_name, id);
                                    }
                                    if let Some(id) = object.alliance_id {
                                        get_name_if_none(&context.resolver, &object.alliance_name, id);
                                    }
                                    if let Some(id) = object.faction_id {
                                        get_name_if_none(&context.resolver, &object.faction_name, id);
                                    }
                                    context.responses.push(Message::Report((msg_id, Report::Victim(object))));
                                },
                                Err(e) => {
                                    warn!("was not able to load killmail: {}", e);
                                    context.responses.push(Message::Report((msg_id, Report::NotFoundId(*id))));
                                }
                            }
                        },
                        Category::Attackers(id) => {
                            match models::attacker::AttackerNamed::load(&conn, &id) {
                                Ok(objects) => {
                                    info!("loaded attakers for KM {} queue length: {}", id, context.database.len());
                                    for object in &objects {
                                        if let Some(id) = object.ship_id {
                                            get_name_if_none(&context.resolver, &object.ship_name, id);
                                        }
                                        if let Some(id) = object.character_id {
                                            get_name_if_none(&context.resolver, &object.character_name, id);
                                        }
                                        if let Some(id) = object.corporation_id {
                                            get_name_if_none(&context.resolver, &object.corporation_name, id);
                                        }
                                        if let Some(id) = object.alliance_id {
                                            get_name_if_none(&context.resolver, &object.alliance_name, id);
                                        }
                                        if let Some(id) = object.faction_id {
                                            get_name_if_none(&context.resolver, &object.faction_name, id);
                                        }
                                        if let Some(id) = object.weapon_id {
                                            get_name_if_none(&context.resolver, &object.weapon_name, id);
                                        }
                                    }
                                    context.responses.push(Message::Report((msg_id, Report::Attackers(objects))));
                                },
                                Err(e) => {
                                    warn!("was not able to load killmail: {}", e);
                                    context.responses.push(Message::Report((msg_id, Report::NotFoundId(*id))));
                                }
                            }
                        },
                        Category::Items(id) => {
                            match models::item::ItemNamed::load(&conn, &id) {
                                Ok(objects) => {
                                    info!("loaded items for KM {} queue length: {}", id, context.database.len());
                                    for object in &objects {
                                        get_name_if_none(&context.resolver, &object.item_type_name, object.item_type_id);
                                    }
                                    context.responses.push(Message::Report((msg_id, Report::Items(objects))));
                                },
                                Err(e) => {
                                    warn!("was not able to load killmail: {}", e);
                                    context.responses.push(Message::Report((msg_id, Report::NotFoundId(*id))));
                                }
                            }
                        },
                        Category::ObservatoryPath(id) => {
                            match models::system::ObservatoryPath::load(&conn, &id) {
                                Ok(objects) => {
                                    info!("loaded paths for {} queue length: {}", id, context.database.len());
                                    context.responses.push(Message::Report((msg_id, Report::ObservatoryPath(objects))));
                                },
                                Err(e) => {
                                    warn!("was not able to load killmail: {}", e);
                                    context.responses.push(Message::Report((msg_id, Report::NotFoundId(*id))));
                                }
                            }
                        },
                        Category::Wins((actor, minutes)) => {
                            let history = match actor {
                                Actor::Character(id) => models::killmail::KillmailNamed::load_character_history_wins(&conn, &id, &minutes),
                                Actor::Corporation(id) => models::killmail::KillmailNamed::load_corporation_history_wins(&conn, &id, &minutes),
                                Actor::Alliance(id) => models::killmail::KillmailNamed::load_alliance_history_wins(&conn, &id, &minutes),
                                Actor::Faction(id) => models::killmail::KillmailNamed::load_faction_history_wins(&conn, &id, &minutes),
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
                        Category::Losses((actor, minutes)) => {
                            let history = match actor {
                                Actor::Character(id) => models::killmail::KillmailNamed::load_character_history_losses(&conn, &id, &minutes),
                                Actor::Corporation(id) => models::killmail::KillmailNamed::load_corporation_history_losses(&conn, &id, &minutes),
                                Actor::Alliance(id) => models::killmail::KillmailNamed::load_alliance_history_losses(&conn, &id, &minutes),
                                Actor::Faction(id) => models::killmail::KillmailNamed::load_faction_history_losses(&conn, &id, &minutes),
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
                        Category::History((area, minutes)) => {
                            let history = match area {
                                Area::System(id) => models::killmail::KillmailNamed::load_system_history(&conn, &id, &minutes),
                                Area::Constellation(id) => models::killmail::KillmailNamed::load_constellation_history(&conn, &id, &minutes),
                                Area::Region(id) => models::killmail::KillmailNamed::load_region_history(&conn, &id, &minutes),
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
                                            for object in &neighbors {
                                                get_name_if_none(&context.resolver, &object.neighbor_name, object.neighbor_id);
                                            }
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
                                            for object in &neighbors {
                                                get_name_if_none(&context.resolver, &object.neighbor_name, object.neighbor_id);
                                            }
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
                                            for object in &neighbors {
                                                get_name_if_none(&context.resolver, &object.neighbor_name, object.neighbor_id);
                                            }
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
                            if
//                            objects.contains(&id) &&
                              !models::ObjectsApi::exist(&conn, &id) {
                                context.resolver.push(Message::Receive(Api::Object(id)));
//                                objects.insert(id);
                            }
                        },
                        Category::System(id) => {
                            if
                            //    !known.contains(&id) &&
                               !models::system::System::exist(&conn, &id)
                            {
                                context.resolver.push(Message::Receive(Api::System(id)));
                                // known.insert(id);
                            }
                        }
                        Category::Region(_) => {
                            // Nothing to do here
                        },
                        Category::Constellation(id) => {
                            if
                            //    !known.contains(&id) &&
                               !models::constellation::Constellation::exist(&conn, &id)
                            {
                                context.resolver.push(Message::Receive(Api::Constellation(id)));
                                // known.insert(id);
                            }
                        },
                        Category::Stargate(id) => {
                            if
                            //    !known.contains(&id) &&
                               !models::stargate::Stargate::exist(&conn, &id)
                            {
                                context.resolver.push(Message::Receive(Api::Stargate(id)));
                                // known.insert(id);
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
