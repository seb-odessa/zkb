use crate::services::{Context, Command, Message};
use uuid::Uuid;

use actix_rt;
use actix_web::{web, App, HttpServer, HttpResponse};


fn quit(context: Context) -> String {
    info!("/quit");
    context.commands.push(Command::Quit);
    context.database.push(Message::Ping);
    context.resolver.push(Message::Ping);
    context.responses.push(Message::Ping);
    actix_rt::System::current().stop();
    format!("Quit\n")
}

fn ping(context: Context) -> String {
    info!("/ping");
    context.database.push(Message::Ping);
    context.resolver.push(Message::Ping);
    context.responses.push(Message::Ping);
    format!("Ping\n")
}

fn response<S: Into<String>>(body: S) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .header("X-Header", "zkb")
        .body(body.into())
}

pub fn get_root(context: &Context) -> String {
    format!("http://{}/navigator", &context.server)
}

fn load<S: Into<String>>(url: S, context: &Context) -> String {
    let id = Uuid::new_v4();
    format!(r##"
        <div id ="{id}"/>
        <script>
            document.getElementById("{id}").innerHTML='<object type="text/html" data="{root}/{api}"/>';
        </script>"##,
        id=id,
        root=get_root(&context),
        api=url.into())
}

fn inner_page(info: web::Path<String>, _context: Context) -> HttpResponse {
    info!("/inner_page/{:?}", info);
    let content = format!("<div>Inner Page {}</div>", info);
    response(content)
}

fn page(info: web::Path<String>, ctx: Context) -> HttpResponse {
    info!("/page/{:?}", info);

    let template = load("inner_page/a",&ctx);

    response(template)
}

fn find(info: web::Path<String>, ctx: Context) -> HttpResponse {
    use crate::reports::Names;
    info!("/find/{}", info);
    response(Names::report(info.as_ref(), &ctx))
}

fn system(info: web::Path<i32>, _context: Context) -> HttpResponse {
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

fn killmail(info: web::Path<i32>, context: Context) -> HttpResponse {
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

fn history(info: web::Path<(i32, i32)>, context: Context) -> HttpResponse {
    info!("/history/{:?}", info);
    let system = info.0;
    let minutes = info.1;
    context.database.push(Message::LoadHistory((system, minutes)));
    let mut body = String::new();
    if let Some(msg) = context.responses.pop() {
        if let Message::ReportHistory(history) = msg {
            body = format!("{}", history);
        }
    }
    response(body)
}

pub fn run(context: Context) {
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
            .route("/navigator/find/{name}", web::get().to(find))
            .route("/navigator/history/{system}/{minutes}", web::get().to(history))

            .route("/navigator/page/{a}", web::get().to(page))
            .route("/navigator/inner_page/{a}", web::get().to(inner_page))

    })
    .bind(address)
    .unwrap()
    .shutdown_timeout(timeout)
    .run()
    .unwrap();
}
