use warp::Filter;

pub fn api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let reboot = warp::path!("api" / "system" / "reboot")
        .and(warp::post())
        .and_then(handlers::reboot);

    let poweroff = warp::path!("api" / "system" / "poweroff")
        .and(warp::post())
        .and_then(handlers::poweroff);

    reboot.or(poweroff)
}

mod handlers {
    use crate::system;
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn reboot() -> Result<impl warp::Reply, Infallible> {
        if system::reboot().is_ok() {
            Ok(StatusCode::OK)
        } else {
            Ok(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }

    pub async fn poweroff() -> Result<impl warp::Reply, Infallible> {
        if system::poweroff().is_ok() {
            Ok(StatusCode::OK)
        } else {
            Ok(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
