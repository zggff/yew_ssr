use std::convert::Infallible;
use std::error::Error;

use axum::body::{Body, StreamBody};
use axum::extract::State;
use axum::handler::Handler;
use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get_service;
use axum::Router;
use futures::stream::{self, StreamExt};
use tower_http::services::ServeDir;

mod cache;

#[derive(Clone)]
struct YewRendererState {
    index_html_before: String,
    index_html_after: String,
}

async fn render_yew_app(
    State(state): State<YewRendererState>,
    url: Request<Body>,
) -> impl IntoResponse {
    let renderer =
        yew::ServerRenderer::<client::ServerApp>::with_props(move || client::ServerAppProps {
            url: url.uri().to_string().into(),
        });

    StreamBody::new(
        stream::once(async move { state.index_html_before })
            .chain(renderer.render_stream())
            .chain(stream::once(async move { state.index_html_after }))
            .map(Result::<_, Infallible>::Ok),
    )
}

async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    env_logger::init();

    let port = match std::env::var("PORT") {
        Ok(port) => port.parse().unwrap_or(3000),
        _ => 3000,
    };
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));

    let index_html_s = tokio::fs::read_to_string("./dist/index.html")
        .await
        .expect("failed to read index.html");
    let (index_html_before, index_html_after) = index_html_s.split_once("<body>").unwrap();
    let mut index_html_before = index_html_before.to_owned();
    index_html_before.push_str("<body>");

    let index_html_after = index_html_after.to_owned();
    let state = YewRendererState {
        index_html_before,
        index_html_after,
    };

    let renderer = render_yew_app.with_state(state);

    let serve_dir = ServeDir::new("./dist");
    let serve_dir = get_service(serve_dir)
        .handle_error(handle_error)
        .layer(cache::Cache::new(1));

    let app = Router::new()
        .nest_service("/dist", serve_dir)
        .fallback_service(renderer);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

