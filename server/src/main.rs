use std::fs::File;

use actix_web::{self, get, middleware, App, HttpResponse, HttpServer, Responder};
use client::{self, add_1};
use models::{self, Person};
use simplelog::{CombinedLogger, LevelFilter, WriteLogger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Debug,
        simplelog::Config::default(),
        File::create("wifi_rs.log").unwrap(),
    )]) {
        Ok(_) => log::debug!("Logger initialized"),
        Err(e) => log::debug!("Logger failed to initialize: {}", e),
    }

    // Start the web server.
    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default())
        // .route("/", web::get().to(root))
    })
    .bind(("127.0.0.1", 8000))
    .unwrap_or_else(|_| {
        log::warn!("Can not bind to port 8081");
        std::process::exit(1)
    })
    .run()
    .await
}

#[get("/")]
async fn root() -> impl Responder {
    // Return './templates/index.html'.
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../templates/index.html"))
}
