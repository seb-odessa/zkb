use crate::services::{AppContext, Command, Message};

use actix_rt;
use actix_web::{web, App, HttpServer, Result};

// use std::fmt::Write;

fn quit(context: web::Data<AppContext>) -> String {
    info!("server received Command::Quit");
    context.commands.push(Command::Quit);    
    context.database.push(Message::Ping);
    context.resolver.push(Message::Ping);
    context.responses.push(Message::Ping);
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
    let id = *info.as_ref();
    context.database.push(Message::LoadKill(id));
    let mut response = String::from("Not found");
    if let Some(Message::Respond(report)) = context.responses.pop() {
        if report.killmail_id == id {
            response = format!("{:?}\n", report);
        } else {
            context.responses.push(Message::Respond(report))
        }
    }
    Ok(response)
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
