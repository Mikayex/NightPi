use crate::config::Config;
use std::error;
use std::net::IpAddr;
use std::path::PathBuf;
use structopt::StructOpt;
use warp::Filter;

mod assets;
mod config;
mod web;

#[derive(Debug, StructOpt)]
#[structopt(about = "NightPi camera server.")]
struct Arguments {
    /// Config file
    #[structopt(short, long, parse(from_os_str))]
    config: Option<PathBuf>,
}

fn configuration(args: &Arguments) -> Config {
    match &args.config {
        None => Config::default(),
        Some(config_file) => {
            let file_content = std::fs::read_to_string(config_file).unwrap();
            toml::from_str(&file_content).unwrap()
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let args = Arguments::from_args();
    let config = configuration(&args);

    let http_logger = warp::log("http");

    let index = warp::path::end().map(|| web::IndexTemplate::new());
    let assets = warp::path("assets")
        .and(warp::path::tail())
        .and_then(assets::serve);

    let routes = assets.or(index).with(http_logger);

    let address: IpAddr = config.server.ip.parse()?;
    warp::serve(routes).run((address, config.server.port)).await;

    Ok(())
}
