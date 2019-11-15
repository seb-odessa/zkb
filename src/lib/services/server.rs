use crate::services::{Context, Command, Message};
use crate::reports;

use actix_rt;
use actix_web::{web, App, HttpServer, HttpResponse};


pub fn root(context: &Context) -> String {
    format!("http://{}/navigator", &context.server)
}

fn quit(ctx: &Context) -> String {
    ctx.commands.push(Command::Quit);
    ctx.commands.push(Command::Quit);
    ctx.commands.push(Command::Quit);
    ctx.commands.push(Command::Quit);
    ctx.database.push(Message::Ping);
    ctx.resolver.push(Message::Ping);
    ctx.responses.push(Message::Ping);
    actix_rt::System::current().stop();
    format!("Quit\n")
}

fn ping(ctx: &Context) -> String {
    ctx.database.push(Message::Ping);
    ctx.resolver.push(Message::Ping);
    ctx.responses.push(Message::Ping);
    format!("Ping\n")
}

fn response<S: Into<String>>(body: S) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .header("X-Header", "zkb")
        .body(body.into())
}

fn find(info: web::Path<String>, ctx: Context) -> HttpResponse {
    use crate::reports::Names;
    info!("/find/{}", info);
    response(Names::report(info.as_ref(), &ctx))
}

fn history(info: web::Path<(i32, i32)>, ctx: Context) -> HttpResponse {
    info!("/history/{:?}", info);
    let system = info.0;
    let minutes = info.1;
    response(reports::History::report(&system, &minutes, &ctx))
}

fn api(info: web::Path<(String, String)>, ctx: Context) -> HttpResponse {
    info!("/api/{}/{}", info.0, info.1);
    let body = match info.0.as_ref() {
        "constellation" => reports::Constellation::report(&info.1, &ctx),
        "region" => reports::Region::report(&info.1, &ctx),
        "system" => reports::System::report(&info.1, &ctx),
        "system_brief" => reports::System::brief(&info.1, &ctx),
        "stargate" => reports::Stargate::report(&info.1, &ctx),
        "killmail_brief" => reports::Killmail::brief(&info.1, &ctx),
        "killmail" => reports::Killmail::brief(&info.1, &ctx),
        _=> format!("Unknown Type {} ", info.0)
    };

    HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .header("X-Header", "zkb")
        .body(body)
}

fn cmd(info: web::Path<String>, ctx: Context) -> String {
    info!("/cmd/{}", info);

    match info.as_ref().as_ref() {
        "ping" => ping(&ctx),
        "quit" => quit(&ctx),
        _ => format!("/cmd/{}", info)
    }
}

fn services(info: web::Path<(String, String)>, _ctx: Context) -> String {
    info!("/services/{}/{}", info.0, info.1);
    format!("/services/{}/{}", info.0, info.1)
}

pub fn run(context: Context) {
    let address = context.server.clone();
    let timeout = context.timeout;
    info!("address: {}", address);
    HttpServer::new(move || {
        App::new()
            .register_data(context.clone())
            .route("/navigator/find/{name}", web::get().to(find))
            .route("/navigator/api/{type}/{id}", web::get().to(api))
            .route("/navigator/cmd/{cmd}", web::get().to(cmd))
            .route("/navigator/services/{type}/{id}", web::get().to(services))
            .route("/navigator/history/{system}/{minutes}", web::get().to(history))
    })
    .bind(address)
    .unwrap()
    .shutdown_timeout(timeout)
    .run()
    .unwrap();
}
