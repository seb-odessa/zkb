#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel_migrations;

use actix_web::web;
use crossbeam_utils::thread::scope;

use lib::services::*;
use lib::models::DB;


embed_migrations!("migrations");

fn main() {
//    std::env::set_var("DATABASE_URL", ":memory:");
    env_logger::init();
    let conn =  DB::connection();
    embedded_migrations::run(&conn).expect("In Memory DB migration failed");
    let context = web::Data::new(AppContext::new(conn, "127.0.0.1:8088", "seb_odessa", 10));

    scope(|scope| {
        scope.builder()
             .name("API Server".to_string())
             .spawn(|_| server::run(context.clone()))
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
