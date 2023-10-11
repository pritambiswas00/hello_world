mod handlers;
mod test_router;
use axum::{Router, routing::{get, post}};



pub fn create_router()->Router {
    Router::new().route("/helloworld", get(handlers::hello_world ))
    .route("/test_post", post(test_router::test_handler))
}