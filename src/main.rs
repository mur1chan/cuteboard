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
    body: String
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
        "Blog Entry 1",
        "Blog Entry 2",
        "Blog Entry 3",
        "Blog Entry 4",
        "Blog Entry 5",
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(HtmxMiddleware)
            .service(Files::new("/static", "./static"))
            .route("/", web::get().to(hello))
            .route("/editor", web::get().to(editor))
            .route("/receive-body", web::post().to(receive_body))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
