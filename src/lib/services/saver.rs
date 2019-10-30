use crate::services::*;
use crate::models::*;

use crate::services::{AppContext, Command, Message};

fn enqueue_check(queue: &Queue, id: &i32) {
    queue.push(Message::CheckObject(*id));
}

fn try_enqueue_check(queue: &Queue, id: &Option<i32>) {
    if let Some(id) = id {
        enqueue_check(queue, id);
    }
}

pub fn run(context: actix_web::web::Data<AppContext>) {
    info!("Started");
    loop {
        if let Some(Command::Quit) = context.commands.pop() {
            context.commands.push(Command::Quit);
            context.resolver.push(Message::Ping);
            info!("received Command::Quit");            
            break;
        }
        if let Some(msg) = context.saver.pop() {
            match msg {
                Message::Killmail(killmail) => {
                    if let Ok(ref conn) = context.connection.try_lock() {
                        if !DB::exists(&conn, killmail.killmail_id) {
                            match DB::save(&conn, &killmail) {
                                Ok(()) => info!("saved killmail {} queue length: {}", killmail.killmail_id, context.saver.len()),
                                Err(e) => error!("was not able to save killmail: {}", e)
                            }
                        }
                    } else {
                        warn!("was not able to acquire connection.");
                        context.saver.push(Message::Killmail(killmail.clone()));
                    }

                    enqueue_check(&context.saver, &killmail.solar_system_id);
                    try_enqueue_check(&context.saver, &killmail.moon_id);
                    try_enqueue_check(&context.saver, &killmail.war_id);
                    enqueue_check(&context.saver, &killmail.victim.ship_type_id);
                    try_enqueue_check(&context.saver, &killmail.victim.character_id);
                    try_enqueue_check(&context.saver, &killmail.victim.corporation_id);
                    try_enqueue_check(&context.saver, &killmail.victim.alliance_id);
                    try_enqueue_check(&context.saver, &killmail.victim.faction_id);
                    for attacker in &killmail.attackers {
                        try_enqueue_check(&context.saver, &attacker.ship_type_id);
                        try_enqueue_check(&context.saver, &attacker.character_id);
                        try_enqueue_check(&context.saver, &attacker.corporation_id);
                        try_enqueue_check(&context.saver, &attacker.alliance_id);
                        try_enqueue_check(&context.saver, &attacker.faction_id);
                        try_enqueue_check(&context.saver, &attacker.weapon_type_id);
                    }
                },
                Message::CheckObject(id) => {
                    if let Ok(ref conn) = context.connection.try_lock() {
                        if !ObjectsApi::exist(conn, &id) {
                            context.resolver.push(Message::Resolve((id, true)));
                        }
                    }
                },
                Message::Object(object) => {
                    if let Ok(ref conn) = context.connection.try_lock() {
                        if !ObjectsApi::exist(&conn, &object.id) {
                            match ObjectsApi::save(&conn, &object) {
                                Ok(_) => info!("saved {:?}. Queue length {}", object, context.resolver.len()),
                                Err(e) => error!("was not able to save object: {}", e)
                            }
                        }
                    } else {
                        warn!("was not able to acquire connection.");
                        context.saver.push(Message::Object(object));
                    }
                },
                Message::Ping => {
                    info!("received Message::Ping");
                },
                message => {
                    warn!("saver received unexpected message: {:?} ", message);
                }
            }
        }
    }
    info!("saver ended");
}
