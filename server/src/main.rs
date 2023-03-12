use axum::body::{boxed, Body};
use axum::extract::Path;
use axum::http::{Response, StatusCode};
use axum::Json;
use axum::{response::IntoResponse, routing::get, Router};
use cache::get_or_add_stock;
use clap::Parser;
use helper_structs::ResponseCache;
use once_cell::sync::Lazy;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::ops::Deref;
use std::path::PathBuf;
use std::str::FromStr;
use stock::Stock;
use tokio::sync::Mutex;
use tokio::{fs, signal};
use tower::{ServiceBuilder, ServiceExt};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use crate::screener::Screener;

static CACHE: Lazy<Mutex<Vec<Stock>>> = Lazy::new(|| Mutex::new(cache::state_from_json()));
static RESPONSES: Lazy<Mutex<Vec<ResponseCache>>> = Lazy::new(|| Mutex::new(vec![]));

mod cache;
mod helper_functions;
mod helper_structs;
mod metrics;
mod screener;
mod statements;
mod stock;
mod utils;
mod other;

// Setup the command line interface with clap.
#[derive(Parser, Debug)]
#[clap(name = "server", about = "A server for our wasm project!")]
struct Opt {
    /// set the log level
    #[clap(short = '1', long = "log", default_value = "debug")]
    log_level: String,

    /// set the listen addr
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: String,

    /// set the listen port
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,

    /// set the directory where static files are to be found
    #[clap(long = "static-dir", default_value = "./dist")]
    static_dir: String,
}

#[tokio::main]
async fn main() {
    // let mut scr = Screener::new();
    // scr.init_screen().await;
    // scr.index_everything().await;

    let opt = Opt::parse();

    // Setup logging & RUST_LOG from args
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }
    // enable console logging
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/api/screeners/:name", get(get_screener_results))
        .route("/api/stock/:name", get(get_stock))
        .fallback_service(get(|req| async move {
            match ServeDir::new(&opt.static_dir).oneshot(req).await {
                Ok(res) => {
                    let status = res.status();
                    match status {
                        StatusCode::NOT_FOUND => {
                            let index_path = PathBuf::from(&opt.static_dir).join("index.html");
                            let index_content = match fs::read_to_string(index_path).await {
                                Err(_) => {
                                    return Response::builder()
                                        .status(StatusCode::NOT_FOUND)
                                        .body(boxed(Body::from("index file not found")))
                                        .unwrap()
                                }
                                Ok(index_content) => index_content,
                            };

                            Response::builder()
                                .status(StatusCode::OK)
                                .body(boxed(Body::from(index_content)))
                                .unwrap()
                        }
                        _ => res.map(boxed),
                    }
                }
                Err(err) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Body::from(format!("error: {err}"))))
                    .expect("error response"),
            }
        }))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    log::info!("listening on http://{}", sock_addr);

    axum::Server::bind(&sock_addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown())
        .await;
}

async fn get_screener_results(Path(name): Path<String>) -> impl IntoResponse {
    if name == "Buffetology" {
        let mut responses = RESPONSES.lock().await;
        let endpoint_in_cache = responses.iter().find(|res| res.endpoint == name);

        match endpoint_in_cache {
            Some(res) => {
                println!("USING RESPONSE CACHE...");
                return res.data.clone();
            }
            None => {
                let mut scr = Screener::new();
                scr.init_screen().await;
                let buffetology_stocks = scr.buffetology_screener().await;
                let mut stocks_from_index = Vec::new();

                for i in buffetology_stocks {
                    stocks_from_index.push(CACHE.lock().await.get(i).unwrap().to_owned());
                }

                println!("I GOT HERE!");

                responses.push(ResponseCache {
                    endpoint: name,
                    data: Json(stocks_from_index.clone()),
                });

                return Json(stocks_from_index);
            }
        }
    }

    Json(vec![])
}

async fn get_stock(Path(name): Path<String>) -> impl IntoResponse {
    let mut responses = RESPONSES.lock().await;
    let endpoint_in_cache = responses.iter().find(|res| res.endpoint == name);

    match endpoint_in_cache {
        Some(res) => {
            println!("USING RESPONSE CACHE...");
            return res.data.clone();
        }
        None => {
            
            let mut stock = get_or_add_stock(name.clone()).await; 
            stock.get_all().await;

            responses.push(ResponseCache {
                endpoint: name,
                data: Json(vec![stock.deref().to_owned()]),
            });

            return Json(vec![stock.deref().to_owned()]);
        }
    }
}

async fn shutdown() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    cache::save().await;
    println!("signal received, starting graceful shutdown");
}
