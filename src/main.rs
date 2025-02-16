use axum::{response::Html, routing::get, Router};
use std::fs;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<String> {
    let html_content = fs::read_to_string("../src/static/index.html")
        .expect("Should have been able to read the file");
    Html(html_content)
}
