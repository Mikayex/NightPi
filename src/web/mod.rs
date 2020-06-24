use warp::Filter;

mod api;
mod assets;
mod pages;
mod templates;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let assets = warp::path("assets")
        .and(warp::path::tail())
        .and_then(assets::serve);
    let other_static = warp::path::tail().and_then(assets::serve);

    assets.or(pages::pages()).or(api::api()).or(other_static)
}
