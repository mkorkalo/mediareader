use axum::{
    body::Bytes, extract::Path, http::StatusCode, response::{Html, IntoResponse}, routing::get, Router
};
use std::{
    fs, net::SocketAddr, path::Path as StdPath
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

fn get_error(error: String) -> (StatusCode, [(&'static str, &'static str); 1], axum::body::Bytes) {
    return (StatusCode::BAD_REQUEST, [("Content-Type", "text/html")], Bytes::from(error));
}

fn get_data_response(data: Vec<u8>) -> (StatusCode, [(&'static str, &'static str); 1], axum::body::Bytes) {
    let mime_type = match infer::get(&data) {
        Some(v) => v.mime_type(),
        None => "application/octet-stream",
    };
    return (StatusCode::OK, [("Content-Type", mime_type)], Bytes::from(data));
}

async fn get_media(Path((server, id)): Path<(String, String)>) -> (StatusCode, [(&'static str, &'static str); 1], impl IntoResponse) {
    println!("Handling request: {} {}", server, id);
    if id.len() != "aabbcccccccccccccccccccc".len() {
        return get_error(String::from("Invalid ID length"));
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
        return get_error(String::from("File does not exist"));
    };
    let data = match fs::read(media_path) {
        Ok(data) => data,
        Err(err) => return get_error(err.to_string()),
    };
    return get_data_response(data);
}
