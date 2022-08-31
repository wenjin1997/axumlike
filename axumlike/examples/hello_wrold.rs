use axumlike;

// Handler or Endpoint
async fn hello() -> &'static str {
    "</h1>Hello, World!</h1>"
}

#[tokio::main]
async fn main() {
    let app = axumlike::app()
        .layer(tower_http::authenticate)
        .route("/", get(hello))
        .route("/users", get(users));
    axumlike::start(([127, 0, 0, 1], 3000))
        .server(app.into_make_service())
        .await
        .unwrap();
}