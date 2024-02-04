use warp::{Filter, query};
use serde::Deserialize;


#[derive(Debug, Deserialize)]
struct MyQuery {
    key: String,
    value: i32,
}

#[tokio::main]
async fn main() {
    let query_route = warp::path("query")
        .and(query::<MyQuery>())
        .map(|params: MyQuery| format!("Received query params: {:?}", params));

    warp::serve(query_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
