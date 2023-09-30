use clap::*;
use log::info;
use rust_embed::RustEmbed;
use std::env;
use std::net::ToSocketAddrs;
use warp::Filter;

#[derive(RustEmbed)]
#[folder = "data"]
struct Data;

#[tokio::main]
async fn main() {
    let matches = Command::new("Embedded test")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("warp-embed test server")
        .arg(
            Arg::new("listen")
                .index(1)
                .help("Listen host:port")
                .default_value("127.0.0.1:8080"),
        )
        .arg(
            Arg::new("prefix")
                .short('p')
                .long("prefix")
                .help("server prefix"),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .action(clap::ArgAction::Count),
        )
        .get_matches();

    match matches.get_count("verbose") {
        1 => env::set_var("RUST_LOG", "info"),
        2 => env::set_var("RUST_LOG", "debug"),
        3 => env::set_var("RUST_LOG", "trace"),
        _ => {
            if env::var("RUST_LOG").is_err() {
                env::set_var("RUST_LOG", "warn")
            }
        }
    }
    pretty_env_logger::init();

    let static_file = warp_embed::embed(&Data {});

    let server = static_file.with(warp::log("http"));

    let server = if let Some(x) = matches.get_one::<String>("prefix") {
        warp::path(x.to_string()).and(server).boxed()
    } else {
        server.boxed()
    };

    let listen = matches
        .get_one::<String>("listen")
        .unwrap()
        .to_socket_addrs()
        .unwrap();

    let binded: Vec<_> = listen
        .map(|x| {
            let binded = warp::serve(server.clone()).bind(x);
            info!("binded: {}", x);
            tokio::spawn(binded)
        })
        .collect();
    for one in binded {
        one.await.unwrap();
    }
}
