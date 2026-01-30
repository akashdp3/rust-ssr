use crate::create_app;
use axum::{body::Body, extract::Request, http::StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_root() {
    let app = create_app();

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"Hello World");
}

#[tokio::test]
async fn test_about() {
    let app = create_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/about")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"About");
}

#[tokio::test]
async fn test_name() {
    let app = create_app();
    let first_request = Request::builder()
        .uri("/name/foo")
        .body(Body::empty())
        .unwrap();
    let second_request = Request::builder()
        .uri("/name/bar")
        .body(Body::empty())
        .unwrap();

    let first_response = app.clone().oneshot(first_request).await.unwrap();
    let second_response = app.clone().oneshot(second_request).await.unwrap();

    assert_eq!(first_response.status(), StatusCode::OK);
    assert_eq!(second_response.status(), StatusCode::OK);

    let first_body = first_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let second_body = second_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    assert_eq!(&first_body[..], b"foo");
    assert_eq!(&second_body[..], b"bar");
}

#[tokio::test]
async fn test_file() {
    let app = create_app();
    let expected_result = tokio::fs::read("static/style.css").await.unwrap();

    // Valid Request
    let first_request = Request::builder()
        .uri("/static/style.css")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(first_request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], expected_result);

    // Invalid Request
    let invalid_request = Request::builder()
        .uri("/static/test.test")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(invalid_request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"File Not Found");
}

#[tokio::test]
async fn test_js() {
    let app = create_app();
    let code = "let two = 1 + 1;let definitely_not_four = two + \"2\";definitely_not_four";

    let request = Request::builder()
        .method("POST")
        .uri("/js")
        .header("content-type", "text/plain")
        .body(Body::from(code))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let expected = format!("{}\n\"22\"", code);
    println!("actual: {:?}\nexpected: {}", &body, expected);
    assert_eq!(&body[..], expected.as_bytes());
}
