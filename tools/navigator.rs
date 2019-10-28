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
use crossbeam_utils::sync::Parker;
use crossbeam_utils::sync::Unparker;

use std::sync::Mutex;
use std::fmt::Write;
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
    monitor_queue: Queue,
    saver_queue: Queue,
    saver_parker: Mutex<Parker>,
    saver_unparker: Mutex<Unparker>,
    resolver_queue: Queue,
    resolver_parker: Mutex<Parker>,
    resolver_unparker: Mutex<Unparker>,
}
impl AppContext {
    pub fn new(connection: Connection) -> Self {
        let saver_parker = Parker::new();
        let saver_unparker = saver_parker.unparker().clone();
        let resolver_parker = Parker::new();
        let resolver_unparker = resolver_parker.unparker().clone();

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
            monitor_queue: Queue::new(),
            saver_queue: Queue::new(),
            saver_parker: Mutex::new(saver_parker),
            saver_unparker: Mutex::new(saver_unparker),
            resolver_queue: Queue::new(),
            resolver_parker: Mutex::new(resolver_parker),
            resolver_unparker: Mutex::new(resolver_unparker),
        }
    }
    pub fn park_saver(&self) {
        if let Ok(parker) = self.saver_parker.try_lock() {
            parker.park();
        } else {
            error!("failed to asquire self.saver_parker");
        }
    }
    pub fn unpark_saver(&self) {
        if let Ok(unparker) = self.saver_unparker.try_lock() {
            unparker.unpark();
        } else {
            error!("failed to asquire self.saver_unparker");
        }
    }
    pub fn park_resolver(&self) {
        if let Ok(parker) = self.resolver_parker.try_lock() {
            parker.park();
        } else {
            error!("failed to asquire self.resolver_parker");
        }
    }
    pub fn unpark_resolver(&self) {
        if let Ok(unparker) = self.resolver_unparker.try_lock() {
            unparker.unpark();
        } else {
            error!("failed to asquire self.resolver_unparker");
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
        context.park_resolver();
        while let Ok(msg) = context.resolver_queue.pop() {
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
                _ => {
                    error!("resolver received unexpected message!");
                }
            }
        }
    }
    info!("resolver ended");
}

fn saver(context: web::Data<AppContext>) {
    info!("saver started");
    let mut enabled = true;
    while enabled {
        context.park_saver();
        while let Ok(msg) = context.saver_queue.pop() {
            match msg {
                Message::Quit => {
                    info!("saver received Message::Quit");
                    context.resolver_queue.push(Message::Quit);
                    context.unpark_resolver();
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
                        context.resolver_queue.push(Message::Resolve(killmail));
                        context.unpark_resolver();
                    } else {
                        warn!("saver was not able to acquire connection.");
                        context.saver_queue.push(Message::Save(killmail));
                    }
                },
                _ => {
                    error!("saver received unexpected message!");
                }
            }
        }
    }
    info!("saver ended");
}

fn monitor(context: web::Data<AppContext>) {
    info!("monitor started");
    let mut enabled = true;
    while enabled {
        while let Some(package) = gw::get_package(&context.client) {
            if let Ok(msg) = context.monitor_queue.pop() {
                if Message::Quit == msg {
                    info!("monitor received Message::Quit");
                    context.saver_queue.push(Message::Quit);
                    context.unpark_saver();
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
                context.saver_queue.push(Message::Save(killmail));
                context.unpark_saver();
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
    info!("server received Message::Quit");
    context.monitor_queue.push(Message::Quit);
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
