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

use std::sync::Mutex;
use std::fmt::Write;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Message{
    Quit,
    Killmail(KillMail),
    Object(Object),
    CheckObject(i32),
    Wait(u64),
    Resolve((i32, bool)),
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
    resolver_queue: Queue,
}
impl AppContext {
    pub fn new(connection: Connection) -> Self {
        Self {
            connection: Mutex::new(connection),
            server: String::from("127.0.0.1:8088"),
            client: String::from("seb_odessa"),
            timeout: 5,
            characters: Mutex::new(HashMap::new()),
            corporation: Mutex::new(HashMap::new()),
            alliance: Mutex::new(HashMap::new()),
            systems: Mutex::new(HashMap::new()),
            ships: Mutex::new(HashMap::new()),
            command_queue: Queue::new(),
            saver_queue: Queue::new(),
            resolver_queue: Queue::new(),
        }
    }
}

fn resolver(context: web::Data<AppContext>) {
    info!("resolver started");
    loop {
        if let Ok(msg) = context.command_queue.pop() {
            if Message::Quit == msg {
                info!("resolver received Message::Quit");
                context.command_queue.push(Message::Quit);
                break;
            }
        }
        if let Ok(msg) = context.resolver_queue.pop() {
            match msg {
                Message::Resolve((id, first_try)) => {
                    if let Some(object) = Object::new(&id) {
                        context.saver_queue.push(Message::Object(object.clone()));
                        info!("resolver received {} '{}' '{}'. Queue length {}",
                                    object.id,
                                    object.name,
                                    object.category,
                                    context.resolver_queue.len());
                    } else {
                        warn!("resolver was failed to query object with id {}. Queue length {}",
                                    id,
                                    context.resolver_queue.len());
                        if first_try {
                            context.resolver_queue.push(Message::Resolve((id, false)));
                        }

                    }
                },
                _ => {
                    warn!("Unexpected message");
                }
            }
        }
        if 0 == context.resolver_queue.len() {
            let timeout = context.timeout.into();
            info!("resolver will suspended {} sec", timeout);
            Parker::new().park_timeout(std::time::Duration::from_secs(timeout))
        }
    }
    info!("resolver ended");
}

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
        if let Ok(msg) = context.command_queue.pop() {
            if Message::Quit == msg {
                info!("saver received Message::Quit");
                context.command_queue.push(Message::Quit);
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
                            context.resolver_queue.push(Message::Resolve((id, true)));
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
            if let Ok(msg) = context.command_queue.pop() {
                if Message::Quit == msg {
                    info!("monitor received Message::Quit");
                    context.command_queue.push(Message::Quit);
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
        scope.spawn(|_| resolver(context.clone()));
        scope.spawn(|_| resolver(context.clone()));
    })
    .unwrap();

}
