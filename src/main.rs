use actix_web::{middleware::Logger, web::{get, Data}, App, Error as ActError, HttpResponse, HttpServer};
use dotenv::dotenv;
use handlebars::Handlebars;
use std::env;
use serde_json::json;
use actix_files as fs;

struct State {
    // the current round: column
    round: u8,
    // the current match: row
    r#match: u16,
}

async fn index(hb: Data<Handlebars<'_>>) -> HttpResponse {
    let round1 = vec![
        "Bob", "John", "Jim", "Richard", "Sam", "Thor", "Askeladd", "Thorkell",
    ];

    let round2 = vec![
        "", "", "", "",
    ];

    let round3 = vec![
        "", "",
    ];

    let round4 = vec![
        ""
    ];

    let data = json!({ "round1": round1, "round2": round2, "round3": round3, "round4": round4 });
    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> Result<(), ActError> {
    env_logger::init();
    dotenv().ok();
    let url = env::var("URL").unwrap();

    // handlebars
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./templates")
        .unwrap();
    let handlebars_ref = Data::new(handlebars);

    // SERVER BUILDER
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(handlebars_ref.clone())
            .service(fs::Files::new("/static", "./static"))
            .route("/", get().to(index))
    })
    .workers(1)
    .bind(url)?
    .run();

    server.await?;

    Ok(())
}
