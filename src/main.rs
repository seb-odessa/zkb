#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel_migrations;
extern crate dns_lookup;

use actix_web::web;
use crossbeam_utils::thread::scope;

use lib::services::*;
use lib::models::DB;

embed_migrations!("migrations");

fn main() {
    env_logger::init();

//    std::env::set_var("DATABASE_URL", ":memory:");
    let conn = DB::connection();
    info!("Connection established");
    embedded_migrations::run(&conn).expect("Database migration failed");
    info!("Database migration complete");
    let api_id = format!("{}", dns_lookup::get_hostname().unwrap_or(String::from("seb_odessa")));
    info!("ZKB API ID: {}", api_id);
    let context = web::Data::new(AppContext::new("127.0.0.1:8088", &api_id, 15));
    info!("Application context constructed");
    scope(|scope| {
        scope.builder()
             .name("API Server".to_string())
             .spawn(|_| server::run(context.clone()))
             .expect("Failed to create API Server");
     //    scope.builder()
     //         .name("Monitor".to_string())
     //         .spawn(|_| monitor::run(context.clone()))
     //         .expect("Failed to create Monitor");
        scope.builder()
             .name("Name Resolver".to_string())
             .spawn(|_| resolver::run(context.clone()))
             .expect("Failed to create Name Resolver");
        scope.builder()
             .name("Name Resolver".to_string())
             .spawn(|_| resolver::run(context.clone()))
             .expect("Failed to create Name Resolver");
        scope.builder()
             .name("DB provider".to_string())
             .spawn(|_| database::run(conn, context.clone()))
             .expect("Failed to create database");
    })
    .unwrap();
    info!("Application finished");
}
