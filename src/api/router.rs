use axum::{routing, Router};

async fn foo() -> &'static str {
    "foo"
}

pub async fn make_router() -> Router {
    let app = Router::new().route("/", routing::get(foo));
    app
}
