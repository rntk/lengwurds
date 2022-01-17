use std::sync::{Arc, RwLock};

use crate::api::params;
use crate::UserWords;

use hyper::header::HeaderValue;
use hyper::{Body, Error, Request, Response, StatusCode};
use log::{error, warn};
use serde::Serialize;
use serde_json;

pub fn list_words(
    user_words: Arc<RwLock<UserWords>>,
    req: &Request<Body>,
) -> Result<Response<Body>, Error> {
    let mut resp = match params::user_id(&req) {
        Ok(user_id) => {
            let user_w = user_words.read().unwrap();
            match user_w.list_words(user_id.user_id, "") {
                Ok(words) => json_response(&words),
                Err(e) => {
                    error!("Can't get words list: {}", e);
                    internal_server_error_response()
                }
            }
        }
        Err(e) => {
            warn!("Params parse error: {}", e);
            unauthorized_response()
        }
    };
    resp.headers_mut().insert(
        "Content-Type",
        HeaderValue::from_str("application/json; charset=utf-8").unwrap(),
    );

    Ok(resp)
}

fn internal_server_error_response() -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(
            "{\"error\": \"Internal server error\"}".to_string(),
        ))
        .unwrap()
}

fn unauthorized_response() -> Response<Body> {
    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(Body::from(
            "{\"error\": \"Unauthorized request\"}".to_string(),
        ))
        .unwrap()
}

pub fn json_response(data: &impl Serialize) -> Response<Body> {
    match serde_json::to_vec(data) {
        Ok(b) => Response::new(Body::from(b)),
        Err(e) => {
            error!("Can't serialize data: {}", e);
            internal_server_error_response()
        }
    }
}
