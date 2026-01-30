use axum::{
    Router,
    routing::{get, post},
};
use tokio::net::TcpListener;

mod handlers;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    let app = create_app();

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub(crate) fn create_app() -> Router {
    Router::new()
        .route("/", get(handlers::root))
        .route("/about", get(handlers::about))
        .route("/name/{name}", get(handlers::name))
        .route("/static/{file_name}", get(handlers::file))
        .route("/js", post(handlers::js))
}
