use crate::services::{AppContext, Command, Message};

use actix_rt;
use actix_web::{web, App, HttpServer, Result};

use std::fmt::Write;

fn quit(context: web::Data<AppContext>) -> String {
    info!("server received Command::Quit");
    context.commands.push(Command::Quit);
    actix_rt::System::current().stop();
    format!("Quit\n")
}

fn ping(context: web::Data<AppContext>) -> String {
    context.database.push(Message::Ping);
    context.resolver.push(Message::Ping);    
    format!("Ping\n")
}

fn killmail(info: web::Path<i32>, context: web::Data<AppContext>) -> Result<String> {
    info!("/killmail/{}", info);
    context.database.push(Message::LoadKill(*info.as_ref()));
    // if let Ok(systems) = context.systems.try_lock() {
    //     let name = info.into_inner();
    //     let id: i32 = systems.get(&name).cloned().unwrap_or_default();
    //     Ok(format!("The '{}' system has id {}!\n", name, id))
    // } else {
    //     Ok(format!("The '{}' was not found!\n", info))
    // }
    Ok(format!("The '{}' was queried!\n", info))
}

pub fn run(context: web::Data<AppContext>) {
    let address = context.server.clone();
    let timeout = context.timeout;
    info!("address: {}", address);
    HttpServer::new(move || {
        App::new()
            .register_data(context.clone())
            .route("/quit", web::get().to(quit))
            .route("/ping", web::get().to(ping))
            .route("/killmail/{id}", web::get().to(killmail))
    })
    .bind(address)
    .unwrap()
    .shutdown_timeout(timeout)
    .run()
    .unwrap();
}
