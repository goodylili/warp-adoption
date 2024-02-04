use warp::Filter;

#[tokio::main]
async fn main() {
    let auth = warp::header::<String>("authorization");

    let auth_route = warp::path("protected")
        .and(auth)
        .map(|auth_header: String| format!("Authorized: {}", auth_header));

    warp::serve(auth_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
