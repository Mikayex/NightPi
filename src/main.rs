use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path!(String).map(|path| format!("You called /{}", path));
    warp::serve(hello).run(([0, 0, 0, 0], 3030)).await;
}
