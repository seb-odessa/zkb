use crate::services::{AppContext, Command, Message};

use actix_rt;
use actix_web::{web, App, HttpServer, HttpResponse};

fn quit(context: web::Data<AppContext>) -> String {
    info!("/quit");
    context.commands.push(Command::Quit);
    context.database.push(Message::Ping);
    context.resolver.push(Message::Ping);
    context.responses.push(Message::Ping);
    actix_rt::System::current().stop();
    format!("Quit\n")
}

fn ping(context: web::Data<AppContext>) -> String {
    info!("/ping");
    context.commands.push(Command::Quit);
    context.database.push(Message::Ping);
    context.resolver.push(Message::Ping);
    context.responses.push(Message::Ping);
    format!("Ping\n")
}

fn response(body: String) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .header("X-Header", "zkb")
        .body(body)
}

fn ids(info: web::Path<String>, _context: web::Data<AppContext>) -> HttpResponse {
    use crate::reports::Names;
    info!("/ids/{}", info);

    response(
        if let Some(names) = Names::new(info.as_ref()) {
            format!("{}", names)
        } else {
            format!("Pattern '{}' was not found", info.as_ref())
        }
    )

}

fn system(info: web::Path<i32>, _context: web::Data<AppContext>) -> HttpResponse {
    use crate::reports::System;
    info!("/system/{}", info);
    response(
        if let Some(system) = System::new(info.as_ref()) {
            format!("{}", system)
        } else {
            format!("System {} not found", info.as_ref())
        }
    )
}

fn killmail(info: web::Path<i32>, context: web::Data<AppContext>) -> HttpResponse {
    info!("/killmail/{}", info);
    let id = *info.as_ref();
    context.database.push(Message::LoadKill(id));
    let mut body = String::new();
    while let Some(msg) = context.responses.pop() {
        if let Message::ReportKill(report) = msg {
            if report.killmail_id == id {
                body = format!("{}", report);
                break;
            } else {
                context.responses.push(Message::ReportKill(report));
            }
        } else if let Message::NotFound(id) = msg {
                body = format!("Killmail {} was not found in database", id);
                break;
        } else {
            warn!("Unexpected {:?}", &msg);
            context.responses.push(msg);
        }
    }
    response(body)
}

pub fn run(context: web::Data<AppContext>) {
    let address = context.server.clone();
    let timeout = context.timeout;
    info!("address: {}", address);
    HttpServer::new(move || {
        App::new()
            .register_data(context.clone())
            .route("/navigator/quit", web::get().to(quit))
            .route("/navigator/ping", web::get().to(ping))
            .route("/navigator/killmail/{id}", web::get().to(killmail))
            .route("/navigator/system/{id}", web::get().to(system))
            .route("/navigator/ids/{name}", web::get().to(ids))
    })
    .bind(address)
    .unwrap()
    .shutdown_timeout(timeout)
    .run()
    .unwrap();
}
