mod config;

use std::collections::HashMap;

use actix_files::Files;
use actix_htmx::{Htmx, HtmxMiddleware};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use askama::Template;
use serde::Deserialize;

#[derive(Template)]
#[template(path = "components/card.html")]
struct CardTemplate<'a> {
    card_title: &'a str,
}

#[derive(Template)]
#[template(path = "blog_entry.html")]
struct BlogEntry {
    content: String,
}

#[derive(Template)]
#[template(path = "editor.html")]
struct EditorTemplate;

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate<'a> {
    cards: &'a str,
}

#[derive(Template)]
#[template(path = "receive_body.html")]
struct ReceiveBodyTemplate {
    body: String,
}

#[derive(Deserialize)]
struct Content {
    content: String,
}

async fn receive_body(content: web::Json<Content>) -> impl Responder {
    let body_content: String = content.content.clone();
    println!("received: {}", &body_content);

    let response_template = ReceiveBodyTemplate { body: body_content };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(response_template.render().unwrap())
}

async fn hello(htmx: Htmx) -> impl Responder {
    if htmx.is_htmx {
        println!("{:?}", htmx.target())
    }
    let titles = vec![
        "Headlines",
        "Lists",
        "Citations",
        "Highlighting",
        "Links",
        "Tables",
        "Inline Codes",
        "Blog",
    ];
    let mut cards_html = String::new();

    for title in titles {
        let card = CardTemplate { card_title: title };
        cards_html.push_str(&card.render().unwrap());
    }

    let base = DashboardTemplate { cards: &cards_html };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(base.render().unwrap())
}

async fn editor() -> impl Responder {
    let editor = EditorTemplate;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(editor.render().unwrap())
}

async fn blog_entry(path: web::Path<(String,)>) -> impl Responder {
    let title = path.0.to_lowercase();
    let config: HashMap<String, String> = config::get_config().to_map();
    let content = config.get(&title);
    println!("This is the content of {}", title);
    let blog_entry = BlogEntry {
        content: content.expect("Blog Entry is broken").to_string(),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(blog_entry.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let current_dir = std::env::current_dir()?;
    println!("Current directory is: {:?}", current_dir);
    HttpServer::new(|| {
        App::new()
            .wrap(HtmxMiddleware)
            .service(Files::new("/static", "./static"))
            .route("/", web::get().to(hello))
            .route("/editor", web::get().to(editor))
            .route("/receive-body", web::post().to(receive_body))
            .route("/blog-entry/{title}", web::get().to(blog_entry))
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
