use actix_cors::Cors;
use actix_web::HttpRequest;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use std::error::Error;
use std::fs;
mod cache;

#[actix_web::get("{tail}*")]
async fn render_yew_app(req: HttpRequest) -> impl Responder {
    log::debug!("{:?}", req);
    let index_html_fs = fs::read_to_string("./dist/index.html").unwrap();
    let props = client::ServerAppProps {
        url: req.uri().to_string().into(),
    };
    let content = yew::ServerRenderer::<client::ServerApp>::with_props(move || props)
        .render()
        .await;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(index_html_fs.replace("<body>", &format!("<body>{}", content)))
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    env_logger::init();

    let port = match std::env::var("PORT") {
        Ok(port) => port.parse().unwrap_or(3000),
        _ => 3000,
    };

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .service(
                web::scope("/dist")
                    .wrap(cache::CacheInterceptor::new(30))
                    .service(actix_files::Files::new("", "./dist").use_last_modified(true)),
            )
            .service(render_yew_app)
            .wrap(cors)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;

    Ok(())
}
