use hyper::service::{make_service_fn, service_fn};
use hyper::{Server, Request, Body};
use env_logger::Builder;
use log::LevelFilter;
use clap::Clap;
use chrono::Local;
use std::io::Write;

mod transform;
mod server;
mod post;

#[derive(Clap, Clone)]
#[clap(version = "0.1", author = "Verticaleap <dan@findelabs.com>")]
struct Opts {
    #[clap(short, long )]
    url: String,
    #[clap(short, long )]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let opts: Opts = Opts::parse();

    // Initialize log Builder
    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S:%f"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    let addr = ([0, 0, 0, 0], opts.port).into();

    let service = make_service_fn(move |_| { 
        let opts = opts.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req: Request<Body>| {
                server::echo(req, opts.url.clone())
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
