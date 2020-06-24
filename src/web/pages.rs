use super::templates;
use warp::Filter;

pub fn pages() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let index = warp::path::end().map(templates::IndexTemplate::new);
    let settings = warp::path("settings").map(templates::SettingsTemplate::new);

    index.or(settings)
}
