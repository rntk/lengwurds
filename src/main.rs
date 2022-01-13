mod api;
mod storage;
mod telegram;
mod translate;

use log::{error, info};
use std::sync::{Arc, RwLock};

use std::env;
use std::net::{SocketAddr, SocketAddrV4};

use env_logger;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;

#[tokio::main]
async fn main() {
    //export RUST_LOG=info
    env_logger::init();
    let host = env::var("LW_HOST").expect("No LW_HOST");
    let token = env::var("LW_TOKEN").expect("No LW_TOKEN");
    let db_path = env::var("LW_DB").expect("No LW_DB");

    let addr = SocketAddrV4::from(host.parse().expect("Invalid host address"));

    let storage = match storage::Storage::new(db_path.as_str()) {
        Ok(storage) => Arc::new(RwLock::new(storage)),
        Err(e) => {
            panic!("Can't open DB {}", e);
        }
    };
    let translator = Arc::new(RwLock::new(translate::Translator::new(token.as_str())));

    let make_svc = make_service_fn(move |_conn| {
        let stor = storage.clone();
        let tran = translator.clone();
        async {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                api::router(req, stor.clone(), tran.clone())
            }))
        }
    });
    let server = Server::bind(&SocketAddr::from(addr)).serve(make_svc);
    info!("Start server: {}", host);
    if let Err(e) = server.await {
        error!("Server error: {}", e);
    }
}
