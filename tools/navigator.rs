#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel_migrations;

use lib::api::gw;
use lib::api::killmail::KillMail;
use lib::models::*;

use actix_rt;
use actix_web::{web, App, HttpServer, Result};
use crossbeam_queue::SegQueue;
use crossbeam_utils::thread::scope;


use std::sync::Mutex;
use std::thread;

#[derive(Debug, PartialEq)]
pub enum Message<T> {
    Quit,
    Save(T),
    Wait(u64),
}

type Queue = SegQueue<Message<KillMail>>;

struct AppContext {
    counter: Mutex<u32>,
    connection: Mutex<Connection>,
    server: String,
    client: String,
    timeout: u64,
    queue: Queue,
}
impl AppContext {
    pub fn new(connection: Connection) -> Self {
       // @todo implement complete ctor
        Self {
            counter: Mutex::new(0),
            connection: Mutex::new(connection),
            server: String::from("127.0.0.1:8088"),
            client: String::from("seb_odessa"),
            timeout: 10,
            queue: Queue::new(),
        }
    }
}

fn registrant(context: web::Data<AppContext>) {
    info!("registrant started");
    let mut enabled = true;
    while enabled {
        while let Ok(msg) = context.queue.pop() {
            match msg {
                Message::Quit => {
                    info!("registrant received Message::Quit");
                    context.queue.push(Message::Quit);
                    enabled = false;
                    break;
                },
                Message::Save(killmail) => {
                    if let Ok(ref conn) = context.connection.try_lock() {
                        if !DB::exists(&conn, killmail.killmail_id) {
                            match DB::save(&conn, &killmail)
                            {
                                Ok(()) => info!("registrant saved killmail {}", killmail.killmail_id),
                                Err(e) => error!("registrant was not able to save killmail: {}", e)
                            }
                        }
                    } else {
                        warn!("registrant was not able to acquire connection.");
                        context.queue.push(Message::Save(killmail));
                    }
                },
                Message::Wait(timeout) => {
                    info!("registrant will suspended {} sec", timeout);
                    thread::sleep(std::time::Duration::from_secs(timeout));
                }
            }
        }
    }
    info!("registrant ended");
}

fn monitor(context: web::Data<AppContext>) {
    info!("monitor started");
    let mut enabled = true;
    while enabled {
        while let Some(package) = gw::get_package(&context.client) {
            if let Ok(msg) = context.queue.pop() {
                if Message::Quit == msg {
                    info!("monitor received Message::Quit");
                    context.queue.push(Message::Quit);
                    enabled = false;
                    break;
                } else {
                    context.queue.push(msg);
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
                context.queue.push(Message::Save(killmail));
            }
        }
        let timeout = context.timeout.into();
        context.queue.push(Message::Wait(timeout));
        info!("monitor will suspended {} sec", timeout);
        thread::sleep(std::time::Duration::from_secs(timeout));
    }
    info!("monitor ended");
}

fn quit(context: web::Data<AppContext>) -> String {
    info!("server received Message::Quit");
    let mut counter = context.counter.lock().unwrap();
    *counter += 1;
    context.queue.push(Message::Quit);
    actix_rt::System::current().stop();
    format!("Quit\nRequest number: {}\n", counter)
}

fn system(info: web::Path<String>, context: web::Data<AppContext>) -> Result<String> {
    info!("/system/{}", info);
    let mut counter = context.counter.lock().unwrap();
    *counter += 1;
    Ok(format!("Welcome {}!\n", info))
}

fn server(context: web::Data<AppContext>) {
    let address = context.server.clone();
    let timeout = context.timeout;
    info!("address: {}", address);
    HttpServer::new(move || {
        App::new()
            .register_data(context.clone())
            .route("/quit", web::get().to(quit))
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
    std::env::set_var("DATABASE_URL", ":memory:");
    env_logger::init();
    let conn =  DB::connection();
    embedded_migrations::run(&conn).expect("In Memory DB migration failed");
    let context = web::Data::new(AppContext::new(conn));

    scope(|scope| {
        scope.spawn(|_| server(context.clone()));
        scope.spawn(|_| monitor(context.clone()));
        scope.spawn(|_| registrant(context.clone()));
    })
    .unwrap();


}
