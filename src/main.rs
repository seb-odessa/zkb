#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel_migrations;

use actix_rt;
use actix_web::{web, App, HttpServer, Result};
use crossbeam_utils::thread::scope;

use std::fmt::Write;
use std::collections::HashMap;
use lib::services::*;
use lib::models::*;

fn quit(context: web::Data<AppContext>) -> String {
    info!("server received Command::Quit");
    context.commands.push(Command::Quit);
    actix_rt::System::current().stop();
    format!("Quit\n")
}

fn stat(context: web::Data<AppContext>) -> String {
    let mut result = String::new();
    write!(&mut result, "Statistics:\n").unwrap();
    return result;
}

fn system(info: web::Path<String>, context: web::Data<AppContext>) -> Result<String> {
    info!("/system/{}", info);
    // if let Ok(systems) = context.systems.try_lock() {
    //     let name = info.into_inner();
    //     let id: i32 = systems.get(&name).cloned().unwrap_or_default();
    //     Ok(format!("The '{}' system has id {}!\n", name, id))
    // } else {
    //     Ok(format!("The '{}' was not found!\n", info))
    // }
    Ok(format!("The '{}' was queried!\n", info))
}

fn server(context: web::Data<AppContext>) {
    let address = context.server.clone();
    let timeout = context.timeout;
    info!("address: {}", address);
    HttpServer::new(move || {
        App::new()
            .register_data(context.clone())
            .route("/quit", web::get().to(quit))
            .route("/stat", web::get().to(stat))
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
//    std::env::set_var("DATABASE_URL", ":memory:");
    env_logger::init();
    let conn =  DB::connection();
    embedded_migrations::run(&conn).expect("In Memory DB migration failed");
    let context = web::Data::new(AppContext::new(conn, "127.0.0.1:8088", "seb_odessa", 5));

    scope(|scope| {
        scope.builder()
             .name("API Server".to_string())
             .spawn(|_| server(context.clone()))
             .expect("Failed to create API Server");
        scope.builder()
             .name("Monitor".to_string())
             .spawn(|_| monitor::run(context.clone()))
             .expect("Failed to create Monitor");
        scope.builder()
             .name("Saver".to_string())
             .spawn(|_| saver::run(context.clone()))
             .expect("Failed to create Saver");
        scope.builder()
             .name("Name Resolver".to_string())
             .spawn(|_| resolver::run(context.clone()))
             .expect("Failed to create Name Resolver");
        scope.builder()
             .name("Name Resolver".to_string())
             .spawn(|_| resolver::run(context.clone()))
             .expect("Failed to create Name Resolver");
    })
    .unwrap();

}
