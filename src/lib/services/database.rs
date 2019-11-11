use crate::api;
use crate::services::*;
use crate::models::*;
use crate::reports::*;
use crate::services::{AppContext, Command, Message, Category, Report};

fn enqueue_check(queue: &Queue, id: &i32) {
    queue.push(Message::Exist(Category::Object(*id)));
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
                            match KillmailsApi::save(&conn, &killmail) {
                                Ok(()) => info!("saved killmail {} queue length: {}", killmail.killmail_id, context.database.len()),
                                Err(e) => warn!("was not able to save killmail: {}", e)
                            }
                            handle_killmail(&context.database, &killmail);
                        },
                        Model::Object(object) => {
                            match ObjectsApi::save(&conn, &object) {
                                Ok(_) => info!("saved {:?}. Queue length {}", object, context.resolver.len()),
                                Err(e) => warn!("was not able to save object: {}", e)
                            }
                        },
                        model => {
                            warn!("Save not implemented for model: {:?}", model)
                        }
                    }
                },
                Message::Load(category) => {
                    match category {
                        Category::Killmail(id) =>{
                            match Killmail::load(&conn, &id) {
                                Ok(killmail) => {
                                    info!("loaded killmail {} queue length: {}", killmail.killmail_id, context.database.len());
                                    context.responses.push(Message::Report(Report::Killmail(killmail)));
                                },
                                Err(e) => {
                                    warn!("was not able to load killmail: {}", e);
                                    context.responses.push(Message::NotFound(id));
                                }
                            }
                        },
                        Category::History((system_id, minutes)) => {
                            let history = history::History::load(&conn, &system_id, &minutes);
                            info!("loaded {} minutes history for {}, queue length: {}", minutes, system_id, context.database.len());
                            context.responses.push(Message::Report(Report::History(history)));
                        },
                        model => {
                            warn!("Save not implemented for {:?}", model)
                        }
                    }
                },
                Message::Exist(category) => {
                    match category {
                        Category::Object(id) =>{
                            if !ObjectsApi::exist(&conn, &id) {
                                context.resolver.push(Message::Receive(Api::Object(id)));
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
