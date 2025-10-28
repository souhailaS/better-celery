use axum::{routing::{get, post}, Router};
// ...
let  app = Router::new()
    .route("/health", get(api::health))
    .route("/v1/jobs", post(api::create_job))
    .route("/v1/jobs/:id", get(api::get_job))
    .with_state(state)
    .layer(TraceLayer::new_for_http());