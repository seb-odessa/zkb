use crate::services::*;
use crate::models::*;
use crate::api::object::Object;
use crate::services::{AppContext, Command, Message};
use crossbeam_utils::sync::Parker;

fn enqueue_check(queue: &Queue, id: &i32) {
    queue.push(Message::CheckObject(*id));
}

fn try_enqueue_check(queue: &Queue, id: &Option<i32>) {
    if let Some(id) = id {
        enqueue_check(queue, id);
    }
}

pub fn run(context: actix_web::web::Data<AppContext>) {
    info!("saver started");
    loop {
        if let Some(msg) = context.commands.pop() {
            if Command::Quit == msg {
                info!("saver received Command::Quit");
                context.commands.push(Command::Quit);
                break;
            }
        }
        if let Ok(msg) = context.saver_queue.pop() {
            match msg {
                Message::Killmail(killmail) => {
                    if let Ok(ref conn) = context.connection.try_lock() {
                        if !DB::exists(&conn, killmail.killmail_id) {
                            match DB::save(&conn, &killmail) {
                                Ok(()) => info!("saver saved killmail {} queue length: {}", killmail.killmail_id, context.saver_queue.len()),
                                Err(e) => error!("saver was not able to save killmail: {}", e)
                            }
                        }
                    } else {
                        warn!("saver was not able to acquire connection.");
                        context.saver_queue.push(Message::Killmail(killmail.clone()));
                    }

                    enqueue_check(&context.saver_queue, &killmail.solar_system_id);
                    try_enqueue_check(&context.saver_queue, &killmail.moon_id);
                    try_enqueue_check(&context.saver_queue, &killmail.war_id);
                    enqueue_check(&context.saver_queue, &killmail.victim.ship_type_id);
                    try_enqueue_check(&context.saver_queue, &killmail.victim.character_id);
                    try_enqueue_check(&context.saver_queue, &killmail.victim.corporation_id);
                    try_enqueue_check(&context.saver_queue, &killmail.victim.alliance_id);
                    try_enqueue_check(&context.saver_queue, &killmail.victim.faction_id);
                    for attacker in &killmail.attackers {
                        try_enqueue_check(&context.saver_queue, &attacker.ship_type_id);
                        try_enqueue_check(&context.saver_queue, &attacker.character_id);
                        try_enqueue_check(&context.saver_queue, &attacker.corporation_id);
                        try_enqueue_check(&context.saver_queue, &attacker.alliance_id);
                        try_enqueue_check(&context.saver_queue, &attacker.faction_id);
                        try_enqueue_check(&context.saver_queue, &attacker.weapon_type_id);
                    }
                },
                Message::CheckObject(id) => {
                    if let Ok(ref conn) = context.connection.try_lock() {
                        if !ObjectsApi::exist(conn, &id) {
                            context.unresolved.push(Message::Resolve((id, true)));
                        }
                    }
                }
                Message::Object(object) => {
                    if let Ok(ref conn) = context.connection.try_lock() {
                        if !ObjectsApi::exist(&conn, &object.id) {
                            match ObjectsApi::save(&conn, &object) {
                                Ok(_) => info!("saver saved object {} queue length: {}", object.id, context.saver_queue.len()),
                                Err(e) => error!("saver was not able to save object: {}", e)
                            }
                        }
                    } else {
                        warn!("saver was not able to acquire connection.");
                        context.saver_queue.push(Message::Object(object));
                    }
                },
                message => {
                    warn!("saver received unexpected message: {:?} ", message);
                }
            }
        }
        if 0 == context.saver_queue.len() {
            let timeout = context.timeout.into();
            info!("saver will suspended {} sec", timeout);
            Parker::new().park_timeout(std::time::Duration::from_secs(timeout))
        }

    }
    info!("saver ended");
}
