use rust_embed::RustEmbed;
use warp::{http::header::HeaderValue, path::Tail, reply::Response, Rejection, Reply};

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/assets"]
struct Asset;

pub async fn serve(path: Tail) -> Result<impl Reply, Rejection> {
    let asset = Asset::get(path.as_str()).ok_or_else(warp::reject::not_found)?;
    let mime = mime_guess::from_path(path.as_str()).first_or_octet_stream();

    let mut res = Response::new(asset.into());
    res.headers_mut().insert(
        "content-type",
        HeaderValue::from_str(mime.as_ref()).unwrap(),
    );
    Ok(res)
}
