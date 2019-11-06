use crate::services::{AppContext, Command, Message};

use actix_rt;
use actix_web::{web, App, HttpServer, HttpResponse};

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


fn killmail(info: web::Path<i32>, context: web::Data<AppContext>) -> HttpResponse {
    info!("/killmail/{}", info);
    let id = *info.as_ref();
    context.database.push(Message::LoadKill(id));
    let mut response = String::new();
    while let Some(msg) = context.responses.pop() {
        if let Message::ReportKill(report) = msg {
            if report.killmail_id == id {
                response = format!("{}", report);
                break;
            } else {
                context.responses.push(Message::ReportKill(report));
            }
        } else if let Message::NotFound(id) = msg {
                response = format!("Killmail {} was not found in database", id);
                break;
        } else {
            warn!("Unexpected {:?}", &msg);
            context.responses.push(msg);
        }
    }

    HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .header("X-Header", "zkb")
        .body(response)
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
