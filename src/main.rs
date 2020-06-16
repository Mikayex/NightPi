use warp::Filter;

mod assets;

#[tokio::main]
async fn main() {
    let hello = warp::path!(String).map(|path| format!("You called /{}", path));
    let assets = warp::path("assets")
        .and(warp::path::tail())
        .and_then(assets::serve);

    let routes = assets.or(hello);
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
