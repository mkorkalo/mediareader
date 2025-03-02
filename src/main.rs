use axum::{
    response::{Html, IntoResponse},
    extract::Path,
    routing::get,
    body::Bytes,
    Router,
    http::StatusCode
};
use std::{
    path::Path as StdPath,
    fs,
    net::SocketAddr,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { Html("Hello") }))
        .route("/_matrix/media/v1/download/{server}/{id}", get(get_media));
let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_media(Path((server, id)): Path<(String, String)>) -> impl IntoResponse {
    println!("Handling request: {} {}", server, id);
    if id.len() != "aabbcccccccccccccccccccc".len() {
        return (StatusCode::BAD_REQUEST, Bytes::from("Invalid ID length"));
    }
    let id_a = &id[0..2];
    let id_b = &id[2..4];
    let id_c = &id[4..];
    let root_path = StdPath::new("/media/local_content");
    let media_path = root_path.
        join(id_a).join(id_b).join(id_c);
    println!("Reading file: {}", media_path.display());
    let exists = fs::exists(media_path.as_path()).unwrap();
    if !exists {
        return (StatusCode::NOT_FOUND, Bytes::from("File does not exist"));
    };
    return match fs::read(media_path) {
        Ok(data) => (StatusCode::OK, Bytes::from(data)),
        Err(err) => (StatusCode::NOT_FOUND, Bytes::from(err.to_string())),
    };
}
