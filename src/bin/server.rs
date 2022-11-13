use actix_cors::Cors;
use actix_web::HttpRequest;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use once_cell::sync::OnceCell;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::error::Error;
use std::fs::{self, File};
use std::io::BufReader;
mod cache;

static STATIC_DIRECTORY: OnceCell<String> = OnceCell::new();

#[actix_web::get("/{tail:.*}")]
async fn render_yew_app(req: HttpRequest) -> impl Responder {
    log::debug!("{:?}", req);
    let index_html_fs = fs::read_to_string("./dist/index.html").unwrap();
    let props = yew_app::ServerAppProps {
        url: req.uri().to_string().into(),
    };
    let content = yew::ServerRenderer::<yew_app::ServerApp>::with_props(move || props)
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

    STATIC_DIRECTORY
        .set(std::env::var("STATIC_DIR").unwrap_or("./dist".into()))
        .expect("failed to set global variable");
    let port = match std::env::var("PORT") {
        Ok(port) => port.parse().unwrap_or(3000),
        _ => 3000,
    };
    let ip = std::env::var("IP").unwrap_or("0.0.0.0".into());

    let ssl_key = std::env::var("SSL_KEY");
    let ssl_cert = std::env::var("SSL_CERT");

    let server = HttpServer::new(move || {
        let cors = Cors::permissive();
        let files =
            actix_files::Files::new("", STATIC_DIRECTORY.get().unwrap()).use_last_modified(true);

        App::new()
            .service(
                web::scope("/dist")
                    .wrap(cache::CacheInterceptor)
                    .service(files),
            )
            .service(render_yew_app)
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(cors)
    });

    if let (Ok(ssl_key), Ok(ssl_cert)) = (ssl_key, ssl_cert) {
        let config = load_rustls_config(&ssl_cert, &ssl_key);
        server
            .bind((ip.clone(), port))?
            .bind_rustls(format!("{ip}:443"), config)?
            .run()
            .await?;
    } else {
        server.bind((ip.clone(), port))?.run().await?;
    }

    Ok(())
}

fn load_rustls_config(cert: &str, key: &str) -> rustls::ServerConfig {
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open(cert).unwrap());
    let key_file = &mut BufReader::new(File::open(key).unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
