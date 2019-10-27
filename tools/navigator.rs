#[macro_use]
extern crate log;
use lib::api::gw;
use lib::api::killmail::KillMail;

use actix_rt;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use crossbeam_queue::SegQueue;
use crossbeam_utils::thread::scope;

use std::sync::Mutex;
use std::thread;

#[derive(Debug, PartialEq)]
pub enum Message<T> {
    Quit,
    Work(T),
}

type Queue = SegQueue<Message<KillMail>>;

struct AppContext {
    counter: Mutex<u32>,
    server: String,
    client: String,
    timeout: u64,
    monitor_input: Queue,
    monitor_output: Queue,
    navigator_input: Queue,
    navigator_output: Queue,
}
impl AppContext {
    pub fn new() -> Self {
       // @todo implement complete ctor 
        Self { 
            counter: Mutex::new(0),
            server: String::from("127.0.0.1:8088"),
            client: String::from("seb_odessa"),
            timeout: 10,
            monitor_input: Queue::new(),
            monitor_output: Queue::new(),
            navigator_input: Queue::new(),
            navigator_output: Queue::new(),

        }
    }
}

fn quit(context: web::Data<AppContext>) -> String {
    info!("/quit");
    let mut counter = context.counter.lock().unwrap();
    *counter += 1;
    context.monitor_input.push(Message::Quit);
    context.navigator_input.push(Message::Quit);
    actix_rt::System::current().stop();
    format!("Quit\nRequest number: {}\n", counter)
}

fn system(info: web::Path<String>, context: web::Data<AppContext>) -> Result<String> {
    info!("/system/{}", info);
    let mut counter = context.counter.lock().unwrap();
    *counter += 1;
    Ok(format!("Welcome {}!\n", info))
}

fn monitor(context: web::Data<AppContext>) {
    let mut enabled = true;
    while enabled {
        while let Some(package) = gw::get_package(&context.client) {
            if let Some(content) = package.content {
                let killmail = content.killmail;
                info!("{} {} {:>12}/{:<12} {}",
                    killmail.killmail_time.time().to_string(),
                    killmail.href(),
                    killmail.get_dropped_sum(),
                    killmail.get_total_sum(),
                    killmail.get_system_full_name()
                );
                context.monitor_output.push(Message::Work(killmail));
            }
            while let Ok(msg) = context.monitor_input.pop() {
                if Message::Quit == msg {
                    info!("Received Message::Quit");
                    enabled = false;
                }
            }
            if !enabled {
                break;
            }
        }
        thread::sleep(std::time::Duration::from_secs(context.timeout.into()));
    }
}

fn navigator(context: web::Data<AppContext>) {
    let mut enabled = true;
    let mut data = Vec::new();
    while enabled {
        if let Ok(msg) = context.monitor_output.pop() {
            match msg {
                Message::Quit => {
                    info!("Received Message::Quit");
                    enabled = false;
                    break;
                },
                Message::Work(killmail) => {
                    info!("Received Message::Work({})", killmail.killmail_id);
                    data.push(killmail);
                }
            }
        }
        if let Ok(msg) = context.navigator_input.pop() {
            match msg {
                Message::Quit => {
                    info!("Received Message::Quit");
                    enabled = false;
                    break;
                },
                Message::Work(killmail) => {
                    info!("Received Message::Work({})", killmail.killmail_id);
                }
            }
        }
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
            .route("/system/{id}", web::get().to(system))
    })
    .bind(address)
    .unwrap()
    .shutdown_timeout(timeout)
    .run()
    .unwrap();
}

fn main() {
    env_logger::init();
    let context = web::Data::new(AppContext::new());

    scope(|scope| {
        scope.spawn(|_| monitor(context.clone()));
        scope.spawn(|_| server(context.clone()));
    })
    .unwrap();

    
}
