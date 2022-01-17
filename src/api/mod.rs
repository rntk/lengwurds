pub mod api;
pub mod front;
pub mod params;

use std::sync::{Arc, RwLock};

use crate::UserWords;

use hyper::{Body, Error, Method, Request, Response, StatusCode};

pub async fn router(
    req: Request<Body>,
    user_h: Arc<RwLock<UserWords>>,
) -> Result<Response<Body>, Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => front::index(),
        (&Method::GET, "/static/front.js") => front::front_js(),
        (&Method::GET, "/static/front.css") => front::front_css(),
        (&Method::GET, "/api/words") => api::list_words(user_h, &req),
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}
