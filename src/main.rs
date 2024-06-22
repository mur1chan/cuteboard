use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use askama::Template;
use actix_files::Files;


#[derive(Template)]
#[template(path = "components/card.html")]
struct CardTemplate<'a> {
    card_title: &'a str,
}

#[derive(Template)]
#[template(path = "base.html")]
struct BaseTemplate<'a> {
    cards: &'a str,
}

async fn hello() -> impl Responder {
    let titles = vec!["Card 1"];
    let mut cards_html = String::new();

    for title in titles {
        let card = CardTemplate { card_title: title };
        cards_html.push_str(&card.render().unwrap());
    }

    let base = BaseTemplate { cards: &cards_html };
    HttpResponse::Ok().content_type("text/html").body(base.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "./static"))
            .route("/", web::get().to(hello))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
