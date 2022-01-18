pub mod api;
pub mod front;
pub mod params;

use std::sync::{Arc, RwLock};

use crate::UserWords;

use crate::api::front::front_static_files;
use hyper::{Body, Error, Method, Request, Response, StatusCode};

// TODO: tokio RWLock
pub async fn router(
    req: Request<Body>,
    user_h: Arc<RwLock<UserWords>>,
) -> Result<Response<Body>, Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => front::index(),
        (&Method::GET, "/api/words") => api::list_words(user_h, &req),
        (&Method::GET, "/api/langs") => api::list_langs(user_h, &req),
        _ => {
            if req.method() == Method::GET {
                front_static_files(req.uri().path())
            } else {
                let mut not_found = Response::default();
                *not_found.status_mut() = StatusCode::NOT_FOUND;
                Ok(not_found)
            }
        }
    }
}
