use serde_json::json;
use warp::{filters::BoxedFilter, reply::json, Filter, Rejection, Reply};

/// Check that the server is alive
async fn health() -> Result<impl Reply, Rejection> {
    Ok(json(&json!({"ok": true})))
}

pub fn make_routes() -> BoxedFilter<(impl Reply,)> {
    let health = warp::path::end().and_then(health);

    health.boxed()
}
