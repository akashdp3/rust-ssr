use axum::{
    Router,
    extract::Path,
    http::header,
    response::{IntoResponse, Response},
    routing::get,
};
use std::{fs, path::PathBuf};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/about", get(about))
        .route("/users/{name}", get(user))
        .route("/static/{*path}", get(file));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello World"
}

async fn about() -> &'static str {
    "About Hello World"
}

async fn user(Path(name): Path<String>) -> String {
    name
}

async fn file(Path(path): Path<PathBuf>) -> Response {
    let path_str = PathBuf::from("./static").join(&path);

    let content_type = match path.extension().and_then(|f| f.to_str()) {
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        _ => "text/plain",
    };

    match fs::read_to_string(path_str) {
        Ok(content) => ([(header::CONTENT_TYPE, content_type)], content).into_response(),
        Err(_) => (axum::http::StatusCode::NOT_FOUND, "404 Not Found").into_response(),
    }
}
