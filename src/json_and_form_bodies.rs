use warp::{Filter, body::json};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct MyData {
    key: String,
    value: i32,
}

#[tokio::main]
async fn main() {
    let json_route = warp::post()
        .and(warp::path("json"))
        .and(json())
        .map(|data: MyData| format!("Received JSON data: {:?}", data));

    warp::serve(json_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
