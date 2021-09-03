use chrono::Local;
use clap::{crate_version, App, Arg};
use env_logger::{Builder, Target};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Server};
use log::LevelFilter;
use std::io::Write;

mod config;
mod post;
mod server;
mod transform;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let opts = App::new("mongo_alerts_2teams")
        .version(crate_version!())
        .author("Daniel F. <dan@findelabs.com>")
        .about(
            "Simple rust webserver to forward Mongo Atlas Ops Manager alerts to a Microsoft Teams",
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .required(true)
                .value_name("FILE")
                .help("Config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .help("Set port to listen on")
                .required(false)
                .default_value("8000")
                .takes_value(true),
        )
        .get_matches();

    // Initialize log Builder
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{{\"date\": \"{}\", \"level\": \"{}\", \"message\": \"{}\"}}",
                Local::now().format("%Y-%m-%dT%H:%M:%S:%f"),
                record.level(),
                record.args()
            )
        })
        .target(Target::Stdout)
        .filter_level(LevelFilter::Error)
        .parse_default_env()
        .init();

    // Read in config file
    let config = config::parse(&opts.value_of("config").unwrap())?;
    let port: u16 = opts.value_of("port").unwrap().parse().unwrap_or_else(|_| {
        eprintln!("specified port isn't in a valid range, setting to 8080");
        8080
    });

    let addr = ([0, 0, 0, 0], port).into();

    let service = make_service_fn(move |_| {
        let config = config.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req: Request<Body>| {
                server::echo(req, config.clone())
            }))
        }
    });

    let server = Server::bind(&addr).serve(service);

    println!(
        "Starting mongo_alerts_2teams:{} on http://{}",
        crate_version!(),
        addr
    );

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
