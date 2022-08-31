use tracing::info;
use serde::Deserialize;
use axumlike::extract::builtin::typed_header::TypedHeader;
use axumlike::extract::Query;
use axumlike::response::IntoResponse;

#[derive(Deserialize, Debug)]
pub struct Pagination {
    page: usize,
    per_page: usize,
}

pub async fn page_handler(pagination: Query<Pagination>) -> &'static str {
    let pagination: Pagination = pagination.0;

    info!(?pagination, "Got a connection!");

    "<h1>Hello, World!</h1>"
}

// pub async fn handler() -> &'static str {
//     "<h1>Hello, World!</h1>"
// }

pub async fn handler(user_agent: Option<TypedHeader<headers::UserAgent>>) -> impl IntoResponse {
    let url = "localhost";
    if let Some(TypedHeader(user_agent)) = user_agent {
        info!(%url, user_agent = ?user_agent.as_str(), "Got a connection!");
    }

    let res = "<h1>Hello, World!</h1>".into_response();
    // info!(%url, content_type = ?res.headers().get(USER_AGENT), "Got a response!");
    res
}
