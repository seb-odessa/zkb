use crate::services::{Context, Command, Message};
use crate::reports;
use crate::reports::network;
use crate::reports::Reportable;
use crate::reports::ReportableEx;

use actix_rt;
use actix_web::{web, App, HttpServer, HttpResponse};

fn wrap<S: Into<String>>(content: S) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .header("X-Header", "zkb")
        .body(content.into())
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
    ctx.notify("navigator/cmd");
    ctx.database.push(Message::Ping);
    ctx.resolver.push(Message::Ping);
    ctx.responses.push(Message::Ping);
    format!("Ping\n")
}

fn statistic(_info: web::Path<String>, ctx: Context) -> HttpResponse {
    ctx.notify("navigator/cmd");
    let mut output = String::new();
    match ctx.get_visits() {
        Some(map) => {
            for (path, count) in &map {
                reports::div(&mut output, format!("{}: {}", path, count));
            }
        },
        None => {
            reports::div(&mut output, "Was not able to acquire statistic map");
        }
    }

    return wrap(output);
}


fn find(info: web::Path<String>, ctx: Context) -> HttpResponse {
    use crate::reports::Names;
    info!("/find/{}", info);
    ctx.notify("navigator/find");
    wrap(Names::report(info.as_ref(), &ctx))
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

    wrap(body)
}

fn report(info: web::Path<(String, String, i32, i32)>, ctx: Context) -> HttpResponse {
    info!("/report/{:?}", info);
    let category = &info.0;
    let class = &info.1;
    let id = info.2;
    let minutes = info.3;
    ctx.notify(format!("navigator/report/{}/{}", category, class));

    wrap(reports::History::report(category, class, &id, &minutes, &ctx))
}

fn desc(info: web::Path<(String, i32)>, ctx: Context) -> HttpResponse {
    ctx.notify(format!("navigator/desc/{}", &info.0));
    let body = match info.0.as_ref() {
        "alliance" => reports::Alliance::description(&info.1, &ctx),
        "corporation" => reports::Corporation::description(&info.1, &ctx),
        _=> format!("Unknown Type {} ", info.0)
    };
    wrap(body)
}

fn stat(info: web::Path<(String, i32)>, ctx: Context) -> HttpResponse {
    let (route, id) = info.into_inner();
    ctx.notify(format!("navigator/stat/{}", route));
    let body = match route.as_ref() {
        "alliance" => reports::Alliance::stat(&id, &ctx),
        "corporation" => reports::Corporation::stat(&id, &ctx),
        "character" => reports::Character::stat(&id, &ctx),
        "system" => reports::System::stat(&id, &ctx),
        "region" => reports::Region::stat(&id, &ctx),

        _=> format!("Unknown route {} ", route)
    };
    wrap(body)
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
        "system_hint" => reports::System::hint(&id, &ctx),
        "killmail_brief" => reports::Killmail::brief(&id, &ctx),
        "killmail" => reports::Killmail::report(&id, &ctx),
        "character" => reports::Character::report(&route, &id, &ctx),
        "corporation" => reports::Corporation::report(&route, &id, &ctx),
        "alliance" => reports::Alliance::report(&route, &id, &ctx),
        "faction" => reports::Faction::report(&route, &id, &ctx),
        _=> format!("Unknown Type {} ", route)
    };

    wrap(body)
}

fn route(info: web::Path<(String, String, String)>, ctx: Context) -> HttpResponse {
    let (route, departure, destination) = info.into_inner();
    ctx.notify("navigator/api/route");
    let body = reports::System::route_named(route, departure, destination, &ctx);
    wrap(body)
}


fn cmd(info: web::Path<String>, ctx: Context) -> String {
    info!("/cmd/{}", info);

    match info.as_ref().as_ref() {
        "ping" => ping(&ctx),
        "quit" => quit(&ctx),
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

    wrap(body)
}

fn hidden(info: web::Path<(String, i32, String)>, ctx: Context) -> HttpResponse {
    let (area, id, cmd) = info.into_inner();
    ctx.notify("navigator/api/hidden");
    let body = match (area.as_ref(), id, cmd.as_ref()){
        ("system", id, "add") => reports::System::observatory_add(&id, &ctx),
        ("system", id, "del") => reports::System::observatory_del(&id, &ctx),
        _ => String::new()
    };

    wrap(body)
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
            .route("/navigator/cmd/statistic/{arg}", web::get().to(statistic))
            .route("/navigator/services/{type}/{first}/{second}", web::get().to(services))
            .route("/navigator/history/{route}/{id}/{minutes}", web::get().to(history))
            .route("/navigator/report/{category}/{class}/{id}/{minutes}", web::get().to(report))
            .route("/navigator/json/nodes/{area}/{id}/{deep}", web::get().to(nodes))
            .route("/navigator/json/edges/{area}/{id}/{deep}", web::get().to(edges))
            .route("/navigator/js/{script}", web::get().to(script))
    })
    .bind(address)
    .unwrap()
    .shutdown_timeout(timeout)
    .run()
    .unwrap();
}

fn script(info: web::Path<String>, ctx: Context) -> HttpResponse {
    use std::fs;
    let filename = info.into_inner().replace("..", "").replace("//", "/");
    let path = ctx.storage.clone() + &filename;
    if let Ok(content) = fs::read_to_string(path) {
        let content_type = if filename.ends_with(".js") {
            "text/javascript; charset=UTF-8"
        } else if filename.ends_with(".css") {
            "text/css; charset=UTF-8"
        } else {
            "text/plain; charset=UTF-8"
        };
        HttpResponse::Ok().content_type(content_type).header("X-Header", "zkb").body(content)
    } else {
        let content = format!("Failed to open {}", filename);
        HttpResponse::Ok().content_type("text/plain; charset=UTF-8").body(content)
    }
}

fn nodes(info: web::Path<(String, i32, u32)>, ctx: Context) -> HttpResponse {
    let (area, id, deep) = info.into_inner();
    info!("/json/nodes/{}/{}", &area, &id);
    ctx.notify(format!("navigator/json/nodes/{}", area));

    let nodes = match area.as_ref() {
        "system" => network::get_system_network_nodes(&id, deep, &ctx).values().into_iter().cloned().collect(),
//        "constellation" => reports::get_constellation_nodes(&id, &ctx),
        _ => Vec::new()
    };

    HttpResponse::Ok()
        .content_type("application/json; charset=UTF-8")
        .header("X-Header", "zkb")
        .body(serde_json::to_string(&nodes).ok().unwrap_or_default())
}

fn edges(info: web::Path<(String, i32, u32)>, ctx: Context) -> HttpResponse {
    let (area, id, deep) = info.into_inner();
    info!("/json/edges/{}/{}", &area, &id);
    ctx.notify(format!("navigator/json/edges/{}", area));

    let edges = match area.as_ref() {
        "system" => network::get_system_network_edges(&id, deep, &ctx),
//        "constellation" => reports::get_constellation_edges(&id, &ctx),
        _ => Vec::new()
    };

    HttpResponse::Ok()
        .content_type("application/json; charset=UTF-8")
        .header("X-Header", "zkb")
        .body(serde_json::to_string(&edges).ok().unwrap_or_default())

}
