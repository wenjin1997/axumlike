mod handlers;

use std::net::SocketAddr;
use axumlike::{
    handler::get, Router,
};
use tower_http::set_header::SetRequestHeaderLayer;
use hyper::Body;
use http::{HeaderValue, header::USER_AGENT};
use color_eyre::Report;
use tracing::info;
use tracing_subscriber::EnvFilter;
use crate::handlers::{handler, page_handler};

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;

    info!("Axumlike init ...");
    // build our application with a route
    let app =
        Router::new()
            .route("/", get(handler))
            // curl http://127.0.0.1:3000/page?page=2&per_page=30
            .route("/page", get(page_handler))
            .layer(SetRequestHeaderLayer::<_, Body>::overriding(
                USER_AGENT,
                HeaderValue::from_static("tower-http demo")
            ));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axumlike::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
