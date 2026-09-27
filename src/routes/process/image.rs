use crate::routes::route::Route;
use axum::{
    body::Body,
    extract::Request,
    response::Response,
    routing::get,
};

async fn handler(_request: Request<Body>) -> Response {
    Response::builder()
        .status(200)
        .header("Content-Type", "text/plain")
        .body(Body::from("Hello!"))
        .unwrap()
}

pub fn route() -> Route {
    Route::new(
        String::from("/process/image"),
        get(handler)
    )
}
