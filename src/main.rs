use tower_http::{ services::{ ServeDir, ServeFile }, trace::TraceLayer };
use axum::{ Router, routing::{get_service, get, post}, response::IntoResponse, http::StatusCode };

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();

    let frontend = Router::new()
        .fallback(get_service(ServeDir::new("public")).handle_error(handle_error))
        .layer(TraceLayer::new_for_http());
    // let frontend = ServeDir::new("frontend/\\.svelte-kit/output/server")
    //     .fallback(ServeFile::new("frontend/\\.svelte-kit/output/prerendered/fallback.html"));

    axum::Server::bind(&addr)
        .serve(frontend.into_make_service())
        .await
        .unwrap();
}

async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "could not access static files")
}
