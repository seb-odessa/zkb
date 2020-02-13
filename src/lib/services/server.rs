use crate::services::{Context, Command, Message};
use crate::reports;
use crate::reports::Reportable;
use crate::reports::ReportableEx;
use serde::{Deserialize, Serialize};


use actix_rt;
use actix_web::{web, App, HttpServer, HttpResponse};


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
    ctx.notify("navigator/cmd");
    ctx.database.push(Message::Ping);
    ctx.resolver.push(Message::Ping);
    ctx.responses.push(Message::Ping);
    format!("Ping\n")
}

fn statistic(ctx: &Context) -> String {
    ctx.notify("navigator/cmd");
    let mut output = String::new();
    match ctx.get_visits() {
        Some(map) => {
            for (path, count) in &map {
                reports::div(&mut output, format!("{}: {}", path, count));
            }
        },
        None => {
            reports::div(&mut output, "Was not able to acquire statustic map");

        }
    }
    return output;
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
    ctx.notify("navigator/find");
    response(Names::report(info.as_ref(), &ctx))
}

fn history(info: web::Path<(String, i32, i32)>, ctx: Context) -> HttpResponse {
    info!("/history/{:?}", info);
    
    let route = &info.0;
    let id = info.1;
    let minutes = info.2;
    ctx.notify(format!("navigator/history/{}", route));
    
    
    let body = match route.as_ref() {
        "system" => reports::History::system(&id, &minutes, &ctx),
        "region" => reports::History::region(&id, &minutes, &ctx),
        "constellation" => reports::History::constellation(&id, &minutes, &ctx),
        _=> format!("Unknown Area Type {} ", route)
    };

    HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .header("X-Header", "zkb")
        .body(body)
}

fn report(info: web::Path<(String, String, i32, i32)>, ctx: Context) -> HttpResponse {
    info!("/report/{:?}", info);    
    let category = &info.0;
    let class = &info.1;
    let id = info.2;
    let minutes = info.3;
    ctx.notify(format!("navigator/report/{}/{}", category, class));

    let body = reports::History::report(category, class, &id, &minutes, &ctx);

    HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .header("X-Header", "zkb")
        .body(body)
}

fn desc(info: web::Path<(String, i32)>, ctx: Context) -> HttpResponse {
    ctx.notify(format!("navigator/desc/{}", &info.0));
    let body = match info.0.as_ref() {
        "alliance" => reports::Alliance::description(&info.1, &ctx),
        "corporation" => reports::Corporation::description(&info.1, &ctx),
        _=> format!("Unknown Type {} ", info.0)
    };

    HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .header("X-Header", "zkb")
        .body(body)
}

fn stat(info: web::Path<(String, i32)>, ctx: Context) -> HttpResponse {
    let (route, id) = info.into_inner();
    ctx.notify(format!("navigator/stat/{}", route));
    let body = match route.as_ref() {
        "alliance" => reports::Alliance::stat(&id, &ctx),
        "corporation" => reports::Corporation::stat(&id, &ctx),
        "character" => reports::Character::stat(&id, &ctx),
        _=> format!("Unknown route {} ", route)
    };

    HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .header("X-Header", "zkb")
        .body(body)
}

fn api(info: web::Path<(String, String)>, ctx: Context) -> HttpResponse {
    let (route, id) = info.into_inner();
    info!("/api/{}/{}", &route, &id);
    ctx.notify(format!("navigator/api/{}", route));
    let body = match route.as_ref() {
        "constellation" => reports::Constellation::report(&id, &ctx),
        "constellation_brief" => reports::Constellation::brief(&id, &ctx),
        "region" => reports::Region::report(&id, &ctx),
        "region_brief" => reports::Region::brief(&id, &ctx),
        "system" => reports::System::report(&id, &ctx),
        "system_brief" => reports::System::brief(&id, &ctx),
        "killmail_brief" => reports::Killmail::brief(&id, &ctx),
        "killmail" => reports::Killmail::report(&id, &ctx),
        "character" => reports::Character::report(&route, &id, &ctx),
        "corporation" => reports::Corporation::report(&route, &id, &ctx),
        "alliance" => reports::Alliance::report(&route, &id, &ctx),
        "faction" => reports::Faction::report(&route, &id, &ctx),
        _=> format!("Unknown Type {} ", route)
    };

    HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .header("X-Header", "zkb")
        .body(body)
}

fn route(info: web::Path<(String, String, String)>, ctx: Context) -> HttpResponse {
    let (route, departure, destination) = info.into_inner();
    let body = reports::System::route_named(route, departure, destination, &ctx);
    ctx.notify("navigator/api/route");
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
        "statistic" => statistic(&ctx),
        _ => format!("/cmd/{}", info)
    }
}

fn services(info: web::Path<(String, i32, i32)>, ctx: Context) -> HttpResponse {
    info!("/services/{}/{}/{}", info.0, info.1, info.2);
    ctx.notify("navigator/services");
    let body = match info.0.as_ref() {
        "route" => reports::System::route(info.1, info.2, &ctx),
        _ => format!("/services/{}/{}/{}", info.0, info.1, info.2)
    };

    HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .header("X-Header", "zkb")
        .body(body)
}

fn hidden(info: web::Path<(String, i32, String)>, ctx: Context) -> HttpResponse {
    let (area, id, cmd) = info.into_inner();
    ctx.notify("navigator/api/hidden");
    let body = match (area.as_ref(), id, cmd.as_ref()){
        ("system", id, "add") => reports::System::observatory_add(&id, &ctx),
        ("system", id, "del") => reports::System::observatory_del(&id, &ctx),
        _ => String::new()
    };

    HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .header("X-Header", "zkb")
        .body(body)
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
            .route("/navigator/api/{type}/{id}/{cmd}", web::get().to(hidden))
            .route("/navigator/api/route/{safety}/{src}/{dst}", web::get().to(route))
            .route("/navigator/desc/{route}/{id}", web::get().to(desc))
            .route("/navigator/stat/{route}/{id}", web::get().to(stat))
            .route("/navigator/cmd/{cmd}", web::get().to(cmd))
            .route("/navigator/services/{type}/{first}/{second}", web::get().to(services))
            .route("/navigator/history/{route}/{id}/{minutes}", web::get().to(history))
            .route("/navigator/report/{category}/{class}/{id}/{minutes}", web::get().to(report))
            .route("/navigator/json/nodes/{id}", web::get().to(nodes))
            .route("/navigator/json/edges/{id}", web::get().to(edges))
    })
    .bind(address)
    .unwrap()
    .shutdown_timeout(timeout)
    .run()
    .unwrap();
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
struct Node {
    id: i32,
    label: String
}
impl Node {
    pub fn new<S: Into<String>>(id: i32, label: S) -> Self { Self{id: id, label: label.into()} }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
struct Edge {
    from: i32,
    to: i32
}
impl Edge {
    pub fn new(from: i32, to: i32) -> Self { Self{from, to} }
}

fn nodes(_info: web::Path<String>, _ctx: Context) -> HttpResponse {

    let nodes = vec![Node::new(1, "Node 1"), Node::new(2, "Node 2"), Node::new(3, "Node 3"), Node::new(4, "Node 4"), Node::new(5, "Node 5")];
    let json = serde_json::to_string(&nodes).ok().unwrap_or_default();
    HttpResponse::Ok()
        .content_type("application/json; charset=UTF-8")
        .header("X-Header", "zkb")
        .body(json)
}

fn edges(_info: web::Path<String>, _ctx: Context) -> HttpResponse {

    let edges = vec![
        Edge::new(1, 3), Edge::new(1, 2),Edge::new(2, 4),Edge::new(2, 5),Edge::new(3, 2),Edge::new(3, 5),Edge::new(5, 1),
        Edge::new(3, 1), Edge::new(2, 1),Edge::new(4, 2),Edge::new(5, 2),Edge::new(2, 3),Edge::new(5, 3),Edge::new(1, 5),
        ];
    let json = serde_json::to_string(&edges).ok().unwrap_or_default();
    HttpResponse::Ok()
        .content_type("application/json; charset=UTF-8")
        .header("X-Header", "zkb")
        .body(json)

}
