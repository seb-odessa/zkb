#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel_migrations;

use lib::api::gw;
use lib::api::object::Object;
use lib::api::killmail::KillMail;
use lib::models::*;

use actix_rt;
use actix_web::{web, App, HttpServer, Result};
use crossbeam_utils::thread::scope;
use crossbeam_utils::sync::Parker;


use std::fmt::Write;
use std::collections::HashMap;

use lib::services::*;




fn enqueue_check(queue: &Queue, id: &i32) {
    queue.push(Message::CheckObject(*id));
}

fn try_enqueue_check(queue: &Queue, id: &Option<i32>) {
    if let Some(id) = id {
        enqueue_check(queue, id);
    }
}

fn saver(context: web::Data<AppContext>) {
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

fn monitor(context: web::Data<AppContext>) {
    info!("monitor started");
    let mut enabled = true;
    while enabled {
        while let Some(package) = gw::get_package(&context.client) {
            if let Some(msg) = context.commands.pop() {
                if Command::Quit == msg {
                    info!("monitor received Command::Quit");
                    context.commands.push(Command::Quit);
                    enabled = false;
                    break;
                }
            }

            if let Some(content) = package.content {
                let killmail = content.killmail;
                info!("monitor {} {} {:>12}/{:>12} {}",
                    killmail.killmail_time.time().to_string(),
                    killmail.href(),
                    killmail.get_dropped_sum(),
                    killmail.get_total_sum(),
                    killmail.get_system_full_name()
                );
                context.saver_queue.push(Message::Killmail(killmail));
            }
        }
        if !enabled {
            break;
        }
        let timeout = context.timeout.into();
        info!("monitor will suspended {} sec", timeout);
        Parker::new().park_timeout(std::time::Duration::from_secs(timeout))
    }
    info!("monitor ended");
}

fn quit(context: web::Data<AppContext>) -> String {
    info!("server received Command::Quit");
    context.commands.push(Command::Quit);
    actix_rt::System::current().stop();
    format!("Quit\n")
}

fn stat(context: web::Data<AppContext>) -> String {
    let mut result = String::new();
    write!(&mut result, "Statistics:\n").unwrap();
    return result;
}

fn out(stream: &mut String, map: &HashMap<String, i32>) {
    for key in map.keys() {
        write!(stream, "{}\n", key).unwrap();
    }
}

fn system(info: web::Path<String>, context: web::Data<AppContext>) -> Result<String> {
    info!("/system/{}", info);
    // if let Ok(systems) = context.systems.try_lock() {
    //     let name = info.into_inner();
    //     let id: i32 = systems.get(&name).cloned().unwrap_or_default();
    //     Ok(format!("The '{}' system has id {}!\n", name, id))
    // } else {
    //     Ok(format!("The '{}' was not found!\n", info))
    // }
    Ok(format!("The '{}' was queried!\n", info))
}

fn server(context: web::Data<AppContext>) {
    let address = context.server.clone();
    let timeout = context.timeout;
    info!("address: {}", address);
    HttpServer::new(move || {
        App::new()
            .register_data(context.clone())
            .route("/quit", web::get().to(quit))
            .route("/stat", web::get().to(stat))
            .route("/system/{id}", web::get().to(system))
    })
    .bind(address)
    .unwrap()
    .shutdown_timeout(timeout)
    .run()
    .unwrap();
}

embed_migrations!("migrations");

fn main() {
//    std::env::set_var("DATABASE_URL", ":memory:");
    env_logger::init();
    let conn =  DB::connection();
    embedded_migrations::run(&conn).expect("In Memory DB migration failed");
    let context = web::Data::new(AppContext::new(conn, "127.0.0.1:8088", "seb_odessa", 5));

    scope(|scope| {
        scope.spawn(|_| server(context.clone()));
        scope.spawn(|_| monitor(context.clone()));
        scope.spawn(|_| saver(context.clone()));
        scope.spawn(|_| resolver::run(context.clone()));
        scope.spawn(|_| resolver::run(context.clone()));
        scope.spawn(|_| resolver::run(context.clone()));
    })
    .unwrap();

}
