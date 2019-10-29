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
pub enum Message{
    Quit,
    Save(KillMail),
    Wait(u64),
    Resolve(i32),
}

type Queue = SegQueue<Message>;

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
    command_queue: Queue,
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
            command_queue: Queue::new(),
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
            error!("failed to acquire self.saver_unparker");
        }
    }
    pub fn park_resolver(&self) {
        if let Ok(parker) = self.resolver_parker.try_lock() {
            parker.park();
        } else {
            error!("failed to acquire self.resolver_parker");
        }
    }
    pub fn unpark_resolver(&self) {
        if let Ok(unparker) = self.resolver_unparker.try_lock() {
            unparker.unpark();
        } else {
            error!("failed to acquire self.resolver_unparker");
        }
    }
}

fn resolver(context: web::Data<AppContext>) {
    info!("resolver started");
    loop {
        context.park_resolver();
        if let Ok(msg) = context.command_queue.pop() {
            if Message::Quit == msg {
                info!("resolver received Message::Quit");
                context.command_queue.push(Message::Quit);
                context.unpark_resolver();
                break;
            }
        }
        if let Ok(msg) = context.resolver_queue.pop() {
            match msg {
                Message::Resolve(id) => {
                    let exist: bool = context.connection.try_lock()
                                        .map(|conn|ObjectsApi::exist(&conn, &id))
                                        .unwrap_or(false);
                    let mut success = false;
                    if !exist {
                        if let Some(object) = Object::new(&id) {
                            success = context.connection.try_lock()
                                        .map(|conn| ObjectsApi::save(&conn, &object).ok())
                                        .unwrap_or_default()
                                        .unwrap_or(false);
                            if (success) {
                                info!("resolver saved {} {} {}. Queue length {}",
                                    object.id,
                                    object.name,
                                    object.category,
                                    context.resolver_queue.len());
                            } esle {
                                warn!("resolver was failed to save object with id {}. Queue length {}",
                                    object.id,
                                    context.resolver_queue.len());
                            }
                        }
                    }
                    if !success {
                        context.resolver_queue.push(Message::Resolve(id));
                    }
                },
                _ => {
                    warn!("Unexpected message");
                }
            }
        }
    }
    info!("resolver ended");
}

fn enqueue(queue: &Queue, id: &i32) {
    queue.push(Message::Resolve(*id));
}

fn try_enqueue(queue: &Queue, id: &Option<i32>) {
    if let Some(id) = id {
        queue.push(Message::Resolve(*id));
    }
}

fn saver(context: web::Data<AppContext>) {
    info!("saver started");
    loop {
        context.park_saver();
        if let Ok(msg) = context.command_queue.pop() {
            if Message::Quit == msg {
                info!("saver received Message::Quit");
                context.command_queue.push(Message::Quit);
                context.unpark_resolver();
                break;
            }
        }
        if let Ok(msg) = context.saver_queue.pop() {
            match msg {
                Message::Save(killmail) => {
                    if let Ok(ref conn) = context.connection.try_lock() {
                        if !DB::exists(&conn, killmail.killmail_id) {
                            match DB::save(&conn, &killmail) {
                                Ok(()) => info!("saver saved killmail {}", killmail.killmail_id),
                                Err(e) => error!("saver was not able to save killmail: {}", e)
                            }
                        }
                    } else {
                        warn!("saver was not able to acquire connection.");
                        context.saver_queue.push(Message::Save(killmail.clone()));
                        context.park_saver();
                    }

                    enqueue(&context.resolver_queue, &killmail.solar_system_id);
                    try_enqueue(&context.resolver_queue, &killmail.moon_id);
                    try_enqueue(&context.resolver_queue, &killmail.war_id);
                    enqueue(&context.resolver_queue, &killmail.victim.ship_type_id);
                    try_enqueue(&context.resolver_queue, &killmail.victim.character_id);
                    try_enqueue(&context.resolver_queue, &killmail.victim.corporation_id);
                    try_enqueue(&context.resolver_queue, &killmail.victim.alliance_id);
                    try_enqueue(&context.resolver_queue, &killmail.victim.faction_id);
                    for attacker in &killmail.attackers {
                        try_enqueue(&context.resolver_queue, &attacker.ship_type_id);
                        try_enqueue(&context.resolver_queue, &attacker.character_id);
                        try_enqueue(&context.resolver_queue, &attacker.corporation_id);
                        try_enqueue(&context.resolver_queue, &attacker.alliance_id);
                        try_enqueue(&context.resolver_queue, &attacker.faction_id);
                        try_enqueue(&context.resolver_queue, &attacker.weapon_type_id);
                    }
                    context.unpark_resolver();
                },
                _ => {

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
            if let Ok(msg) = context.command_queue.pop() {
                if Message::Quit == msg {
                    info!("monitor received Message::Quit");
                    context.command_queue.push(Message::Quit);
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
    context.command_queue.push(Message::Quit);
    actix_rt::System::current().stop();
    format!("Quit\n")
}

fn stat(context: web::Data<AppContext>) -> String {
    let mut result = String::new();
    write!(&mut result, "Statistics:\n").unwrap();
    write!(&mut result, "Known systems: {}\n", context.systems.try_lock().ok().map(|s|s.len()).unwrap_or_default()).unwrap();
    write!(&mut result, "Known ships: {}\n", context.ships.try_lock().ok().map(|s|s.len()).unwrap_or_default()).unwrap();
    write!(&mut result, "Known characters: {}\n", context.characters.try_lock().ok().map(|s|s.len()).unwrap_or_default()).unwrap();
    write!(&mut result, "Known corporation: {}\n", context.corporation.try_lock().ok().map(|s|s.len()).unwrap_or_default()).unwrap();
    write!(&mut result, "Known alliance: {}\n", context.alliance.try_lock().ok().map(|s|s.len()).unwrap_or_default()).unwrap();
    return result;
}

fn out(stream: &mut String, map: &HashMap<String, i32>) {
    for key in map.keys() {
        write!(stream, "{}\n", key).unwrap();
    }
}

fn systems(context: web::Data<AppContext>) -> String {
    let mut stream = String::new();
    write!(&mut stream, "Known systems:\n").unwrap();
    context.systems.try_lock().ok().map(|map| out(&mut stream, &map)).unwrap();
    return stream;
}

fn ships(context: web::Data<AppContext>) -> String {
    let mut stream = String::new();
    write!(&mut stream, "Known systems:\n").unwrap();
    context.ships.try_lock().ok().map(|map| out(&mut stream, &map)).unwrap();
    return stream;
}

fn characters(context: web::Data<AppContext>) -> String {
    let mut stream = String::new();
    write!(&mut stream, "Known characters:\n").unwrap();
    context.characters.try_lock().ok().map(|map| out(&mut stream, &map)).unwrap();
    return stream;
}

fn corporation(context: web::Data<AppContext>) -> String {
    let mut stream = String::new();
    write!(&mut stream, "Known corporation:\n").unwrap();
    context.corporation.try_lock().ok().map(|map| out(&mut stream, &map)).unwrap();
    return stream;
}

fn alliance(context: web::Data<AppContext>) -> String {
    let mut stream = String::new();
    write!(&mut stream, "Known alliance\n:").unwrap();
    context.alliance.try_lock().ok().map(|map| out(&mut stream, &map)).unwrap();
    return stream;
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
            .route("/systems", web::get().to(systems))
            .route("/ships", web::get().to(ships))
            .route("/characters", web::get().to(characters))
            .route("/corporation", web::get().to(corporation))
            .route("/alliance", web::get().to(alliance))
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
