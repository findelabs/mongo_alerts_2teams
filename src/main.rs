use chrono::Local;
use clap::Clap;
use env_logger::Builder;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Server};
use log::LevelFilter;
use std::collections::HashMap;
use std::io::Write;
use std::sync::{Arc, Mutex};

mod config;
mod post;
mod server;
mod transform;

#[derive(Clap, Clone)]
#[clap(version = "0.1", author = "Verticaleap <dan@findelabs.com>")]
struct Opts {
    #[clap(short, long)]
    config: String,
    #[clap(short, long, default_value = "8000")]
    port: u16,
}

type ConfigHash = Arc<Mutex<HashMap<String, String>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let opts: Opts = Opts::parse();

    // Initialize log Builder
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S:%f"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    // Read in config file
    let config = config::parse(&opts.config)?;

    let addr = ([0, 0, 0, 0], opts.port).into();

    let service = make_service_fn(move |_| {
        let config = config.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req: Request<Body>| {
                server::echo(req, config.clone())
            }))
        }
    });

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
