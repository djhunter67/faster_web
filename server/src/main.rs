use std::fs::File;

use actix_files::NamedFile;
use actix_web::{
    self, get, guard, http, middleware, web, App, Error, HttpResponse, HttpServer, Responder,
};
use askama::Template;
use client::{self, add_1};
use models::{self, History, Person};
use simplelog::{
    ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: &'static str,
    people: Person,
}

#[derive(Template)]
#[template(path = "history.html")]
struct HistoryTemplate {
    title: &'static str,
    history: History,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Stdout,
            ColorChoice::Always,
        ),
        WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            File::create("faster_tracker.log")?,
        ),
    ]) {
        // This is a macro that allows for multiple loggers to be used at once
        Ok(_) => log::debug!("Logger initialized."),
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1)
        }
    }

    // Start the web server.
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(actix_files::Files::new("/static", "static").show_files_listing())
            .service(root)
            .service(history)
            .service(
                web::resource("/favicon.ico")
                    .guard(guard::Method(http::Method::GET))
                    .to(send_favicon_icon),
            )
            .service(
                web::resource("/index.css")
                    .guard(guard::Method(http::Method::GET))
                    .to(send_css_file),
            )
            .service(
                web::resource("/bulma-helpers.min.css")
                    .guard(guard::Method(http::Method::GET))
                    .to(serve_bulma_css),
            )
    })
    .bind(("127.0.0.1", 8080))
    .unwrap_or_else(|_| {
        log::warn!("Can not bind to port 8080");
        std::process::exit(1)
    })
    .run()
    .await
}

#[get("/")]
async fn root() -> impl Responder {
    // Return './templates/index.html'.

    let people = IndexTemplate {
        title: "Faster Tracker",
        people: Person {
            name: "Hunter, Christerpher".to_string(),
            age: 34,
            email: "djhunter67@gmail.com".to_string(),
        },
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(people.render().unwrap())
}

#[get("/history")]
async fn history() -> impl Responder {
    let history = HistoryTemplate {
        title: "Faster Tracker",
        history: History {
            title: "History",
            history: vec![
                "This is the first line".to_string(),
                "This is the second line".to_string(),
                "This is the third line".to_string(),
            ],
        },
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(history.render().unwrap())
}

pub async fn send_favicon_icon() -> Result<NamedFile, Error> {
    Ok(NamedFile::open(
        "static/images/do-not-eat-or-drink-here.svg",
    )?)
}

async fn send_css_file() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/css")
        .body(include_str!("../static/css/index.css"))
}

async fn serve_bulma_css() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/css")
        .body(include_str!("../static/css/bulma-helpers.min.css"))
}
