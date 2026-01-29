use axum::{
    extract::Path,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use std::{env::current_dir, path::PathBuf};

pub async fn root() -> &'static str {
    "Hello World"
}

pub async fn about() -> &'static str {
    "About"
}

pub async fn name(Path(name): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, format!("{}", name))
}

pub async fn file(Path(file_name): Path<PathBuf>) -> impl IntoResponse {
    let current_dir = current_dir().unwrap();
    let static_dir = current_dir.join(PathBuf::from("static"));

    let file_path = match static_dir.join(file_name).canonicalize() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Error while canonicalizing the file path: {err}");
            return (
                StatusCode::NOT_FOUND,
                [("content-type", "text/plain")],
                b"File Not Found".to_vec(),
            );
        }
    };

    if !file_path.strip_prefix(static_dir).is_ok() {
        return (
            StatusCode::FORBIDDEN,
            [("content-type", "text/plain")],
            b"Invalid Path".to_vec(),
        );
    }

    let mime_type = match file_path.extension().and_then(|s| s.to_str()) {
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        _ => "text/plain",
    };

    let file_content = tokio::fs::read(file_path).await.unwrap();

    (StatusCode::OK, [("content-type", mime_type)], file_content)
}
