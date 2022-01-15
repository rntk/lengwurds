mod api;
mod storage;
mod telegram;
mod translate;
mod user;

use log::{error, info};
use std::env;
use std::net::{SocketAddr, SocketAddrV4};
use std::sync::{Arc, RwLock};

use user::user::UserWords;

use env_logger;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //export RUST_LOG=info
    env_logger::init();
    let host = env::var("LW_HOST").expect("No LW_HOST");
    let telegram_token = env::var("LW_TELEGRAM").expect("No LW_TELEGRAM");
    let translate_token = env::var("LW_TRANSLATE").expect("No LW_TRANSLATE");
    let db_path = env::var("LW_DB").expect("No LW_DB");

    let addr = SocketAddrV4::from(host.parse().expect("Invalid host address"));

    let storage = match storage::Storage::new(db_path.as_str()) {
        Ok(storage) => Arc::new(RwLock::new(storage)),
        Err(e) => {
            panic!("Can't open DB {}", e);
        }
    };
    let user_words = Arc::new(RwLock::new(UserWords::new(
        storage.clone(),
        translate::google::Client::new(translate_token.as_str()),
    )));
    let telegram_user_words = user_words.clone();

    std::thread::spawn(|| {
        telegram::updates::updates_processing(telegram_user_words, telegram_token)
    });

    let make_svc = make_service_fn(move |_conn| {
        let user_h = user_words.clone();
        async { Ok::<_, hyper::Error>(service_fn(move |req| api::router(req, user_h.clone()))) }
    });
    let server = Server::bind(&SocketAddr::from(addr)).serve(make_svc);
    info!("Start server: {}", host);
    if let Err(e) = server.await {
        error!("Server error: {}", e);
    }

    Ok(())
}
