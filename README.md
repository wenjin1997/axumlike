# axumlike
`axumlike`是一个 Web 异步框架。
* Route 使用无宏API，实现从请求到处理
* 使用提取器可以解析请求参数
* 充分利用 `tower` 和 `tower-http` 的中间件、`Service`等生态

## Usage example
```rust
use std::net::SocketAddr;

use axumlike::{
    handler::get, 
    Router,
    extract::builtin::typed_header::TypedHeader,
    extract::Query,
    response::IntoResponse,
};

use tower_http::set_header::SetRequestHeaderLayer;
use hyper::Body;
use http::{HeaderValue, header::USER_AGENT};
use color_eyre::Report;
use tracing::info;
use tracing_subscriber::EnvFilter;

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

#[derive(Deserialize, Debug)]
pub struct Pagination {
    page: usize,
    per_page: usize,
}

async fn page_handler(pagination: Query<Pagination>) -> &'static str {
    let pagination: Pagination = pagination.0;

    info!(?pagination, "Got a connection!");

    "<h1>Hello, World!</h1>"
}

async fn handler(user_agent: Option<TypedHeader<headers::UserAgent>>) -> impl IntoResponse {
    let url = "localhost";
    if let Some(TypedHeader(user_agent)) = user_agent {
        info!(%url, user_agent = ?user_agent.as_str(), "Got a connection!");
    }

    let res = "<h1>Hello, World!</h1>".into_response();
    res
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
```