use warp::Filter;

#[tokio::main]
async fn main() {
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(|websocket| async {
                // Handle websocket connections here
            })
        });

    warp::serve(ws_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
