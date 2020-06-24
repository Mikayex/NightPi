use crate::config::Config;
use log::{error, info, warn};
use std::error;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use structopt::StructOpt;
use warp::Filter;

mod config;
mod root;
mod system;
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

    if !root::user_is_root() {
        error!("This program must be run as root");
        panic!();
    }
    match root::drop_root(&config) {
        Ok(_) => {
            if root::user_is_root() {
                warn!("Still running with root privileges");
            } else {
                info!(
                    "Changed user/group to {}:{}",
                    config.user.unwrap(),
                    config.group.unwrap()
                )
            }
        }
        Err(err) => warn!("Can't drop root ({}), continuing with root privileges", err),
    };

    let http_logger = warp::log("http");

    let routes = web::routes().with(http_logger);

    let address: SocketAddr = (config.server.ip.parse::<IpAddr>()?, config.server.port).into();
    let server = root::execute_as_root(|| warp::serve(routes).bind(address))?;
    log::info!("listening on http://{}", address);
    tokio::task::spawn(server).await?;

    Ok(())
}
