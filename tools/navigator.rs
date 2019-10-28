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
use crossbeam_queue::SegQueue;
use crossbeam_utils::thread::scope;


use std::sync::Mutex;
use std::fmt::Write;
use std::thread;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Message<T> {
    Quit,
    Save(T),
    Wait(u64),
    Resolve(T),
}

type Queue = SegQueue<Message<KillMail>>;

struct AppContext {
    connection: Mutex<Connection>,
    server: String,
    client: String,
    timeout: u64,
    characters: Mutex<HashMap<String, i32>>,
    corporation: Mutex<HashMap<String, i32>>,
    alliance: Mutex<HashMap<String, i32>>,
    systems: Mutex<HashMap<String, i32>>,
    ships: Mutex<HashMap<String, i32>>,
    monitor: Queue,
    saver: Queue,
    resolver: Queue,
}
impl AppContext {
    pub fn new(connection: Connection) -> Self {
       // @todo implement complete ctor
        Self {
            connection: Mutex::new(connection),
            server: String::from("127.0.0.1:8088"),
            client: String::from("seb_odessa"),
            timeout: 10,
            characters: Mutex::new(HashMap::new()),
            corporation: Mutex::new(HashMap::new()),
            alliance: Mutex::new(HashMap::new()),
            systems: Mutex::new(HashMap::new()),
            ships: Mutex::new(HashMap::new()),
            monitor: Queue::new(),
            saver: Queue::new(),
            resolver: Queue::new(),
        }
    }
}

fn resolve_system(context: &web::Data<AppContext>, killmail: &KillMail, msg: &str) {
    if let Some(system) = Object::new(&killmail.solar_system_id) {
        if let Ok(ref mut systems) = context.systems.try_lock() {
            systems.entry(system.name).or_insert(system.id);
        } else {
            warn!("{}",msg);
        }
    }
}

fn resolve_ships(context: &web::Data<AppContext>, killmail: &KillMail, msg: &str) {
    let mut objs = Vec::new();

    if let Some(ship) = Object::new(&killmail.victim.ship_type_id) {
        objs.push((ship.name, ship.id));
    }

    for attacker in &killmail.attackers {
        if let Some(ship_id) = attacker.ship_type_id {
            if let Some(ship) = Object::new(&ship_id) {
                objs.push((ship.name, ship.id));
            }
        }
    }
    if let Ok(ref mut ships) = context.ships.try_lock() {
        for obj in objs.into_iter() {
                ships.entry(obj.0).or_insert(obj.1);
            }
    } else {
        warn!("{}",msg);
    }
}

fn resolve_characters(context: &web::Data<AppContext>, killmail: &KillMail, msg: &str) {
    let mut objs = Vec::new();

    if let Some(id) = killmail.victim.character_id {
        if let Some(character) = Object::new(&id) {
            objs.push((character.name, character.id));
        }
    }

    for attacker in &killmail.attackers {
        if let Some(id) = attacker.character_id {
            if let Some(character) = Object::new(&id) {
                objs.push((character.name, character.id));
            }
        }
    }

    if let Ok(ref mut characters) = context.characters.try_lock() {
        for obj in objs.into_iter() {
                characters.entry(obj.0).or_insert(obj.1);
            }
    } else {
        warn!("{}",msg);
    }
}

fn resolve_corporations(context: &web::Data<AppContext>, killmail: &KillMail, msg: &str) {
    let mut objs = Vec::new();

    if let Some(id) = killmail.victim.corporation_id {
        if let Some(corporation) = Object::new(&id) {
            objs.push((corporation.name, corporation.id));
        }
    }

    for attacker in &killmail.attackers {
        if let Some(id) = attacker.corporation_id {
            if let Some(corporation) = Object::new(&id) {
                objs.push((corporation.name, corporation.id));
            }
        }
    }

    if let Ok(ref mut corporation) = context.corporation.try_lock() {
        for obj in objs.into_iter() {
                corporation.entry(obj.0).or_insert(obj.1);
            }
    } else {
        warn!("{}",msg);
    }
}

fn resolve_alliances(context: &web::Data<AppContext>, killmail: &KillMail, msg: &str) {
    let mut objs = Vec::new();

    if let Some(id) = killmail.victim.alliance_id {
        if let Some(alliance) = Object::new(&id) {
            objs.push((alliance.name, alliance.id));
        }
    }

    for attacker in &killmail.attackers {
        if let Some(id) = attacker.alliance_id {
            if let Some(alliance) = Object::new(&id) {
                objs.push((alliance.name, alliance.id));
            }
        }
    }

    if let Ok(ref mut alliance) = context.alliance.try_lock() {
        for obj in objs.into_iter() {
                alliance.entry(obj.0).or_insert(obj.1);
            }
    } else {
        warn!("{}",msg);
    }
}

fn resolver(context: web::Data<AppContext>) {
    info!("resolver started");
    let mut enabled = true;
    while enabled {
        while let Ok(msg) = context.resolver.pop() {
            match msg {
                Message::Quit => {
                    info!("resolver received Message::Quit");
                    enabled = false;
                    break;
                },
                Message::Resolve(killmail) => {
                    resolve_system(&context, &killmail, "resolver was not able to acquire context.systems");
                    resolve_ships(&context, &killmail, "resolver was not able to acquire context.ships");
                    resolve_characters(&context, &killmail, "resolver was not able to acquire context.characters");
                    resolve_corporations(&context, &killmail, "resolver was not able to acquire context.corporations");
                    resolve_alliances(&context, &killmail, "resolver was not able to acquire context.alliances");
                },
                Message::Wait(timeout) => {
                    info!("resolver will suspended {} sec", timeout);
                    thread::sleep(std::time::Duration::from_secs(timeout));
                },
                _ => {
                    error!("resolver received unexpected message!");
                }
            }
        }
        if !enabled {
            break;
        }
        let timeout = context.timeout.into();
        info!("resolver will suspended {} sec", timeout);
        thread::sleep(std::time::Duration::from_secs(timeout));
    }
    info!("resolver ended");
}

fn saver(context: web::Data<AppContext>) {
    info!("saver started");
    let mut enabled = true;
    while enabled {
        while let Ok(msg) = context.saver.pop() {
            match msg {
                Message::Quit => {
                    info!("saver received Message::Quit");
                    context.resolver.push(Message::Quit);
                    enabled = false;
                    break;
                },
                Message::Save(killmail) => {
                    if let Ok(ref conn) = context.connection.try_lock() {
                        if !DB::exists(&conn, killmail.killmail_id) {
                            match DB::save(&conn, &killmail)
                            {
                                Ok(()) => info!("saver saved killmail {}", killmail.killmail_id),
                                Err(e) => error!("saver was not able to save killmail: {}", e)
                            }
                        }
                        context.resolver.push(Message::Resolve(killmail));
                    } else {
                        warn!("saver was not able to acquire connection.");
                        context.saver.push(Message::Save(killmail));
                    }
                },
                Message::Wait(timeout) => {
                    info!("saver will suspended {} sec", timeout);
                    thread::sleep(std::time::Duration::from_secs(timeout));
                },
                _ => {
                    error!("saver received unexpected message!");
                }
            }
        }
        if !enabled {
            break;
        }
        let timeout = context.timeout.into();
        info!("saver will suspended {} sec", timeout);
        thread::sleep(std::time::Duration::from_secs(timeout));
    }
    info!("saver ended");
}

fn monitor(context: web::Data<AppContext>) {
    info!("monitor started");
    let mut enabled = true;
    while enabled {
        while let Some(package) = gw::get_package(&context.client) {
            if let Ok(msg) = context.monitor.pop() {
                if Message::Quit == msg {
                    info!("monitor received Message::Quit");
                    context.saver.push(Message::Quit);
                    enabled = false;
                    break;
                }
            }

            if let Some(content) = package.content {
                let killmail = content.killmail;
                info!("monitor received {} : {} {} {:>12}/{:<12} {}",
                    killmail.killmail_id,
                    killmail.killmail_time.time().to_string(),
                    killmail.href(),
                    killmail.get_dropped_sum(),
                    killmail.get_total_sum(),
                    killmail.get_system_full_name()
                );
                context.saver.push(Message::Save(killmail));
            }
        }
        if !enabled {
            break;
        }
        let timeout = context.timeout.into();
        context.saver.push(Message::Wait(timeout));
        info!("monitor will suspended {} sec", timeout);
        thread::sleep(std::time::Duration::from_secs(timeout));
    }
    info!("monitor ended");
}

fn quit(context: web::Data<AppContext>) -> String {
    info!("server received Message::Quit");
    context.monitor.push(Message::Quit);
    actix_rt::System::current().stop();
    format!("Quit\n")
}

fn stat(context: web::Data<AppContext>) -> String {
    let mut result = String::new();
    write!(&mut result, "Statistics:\n").unwrap();
    write!(&mut result, "Known systems: {}\n", context.systems.try_lock().ok().map(|s|s.len()).unwrap_or_default()).unwrap();
    write!(&mut result, "Known ships: {}\n", context.ships.try_lock().ok().map(|s|s.len()).unwrap_or_default()).unwrap();
    return result;
}

fn system(info: web::Path<String>, context: web::Data<AppContext>) -> Result<String> {
    info!("/system/{}", info);
    if let Ok(systems) = context.systems.try_lock() {
        let name = info.into_inner();
        let id: i32 = systems.get(&name).cloned().unwrap_or_default();
        Ok(format!("The '{}' system has id {}!\n", name, id))
    } else {
        Ok(format!("The '{}' was not found!\n", info))
    }
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
    let context = web::Data::new(AppContext::new(conn));

    scope(|scope| {
        scope.spawn(|_| server(context.clone()));
        scope.spawn(|_| monitor(context.clone()));
        scope.spawn(|_| saver(context.clone()));
        scope.spawn(|_| resolver(context.clone()));
    })
    .unwrap();


}
