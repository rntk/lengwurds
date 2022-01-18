use std::fs;

use hyper::header::HeaderValue;
use hyper::{Body, Error, Response, StatusCode};
use log::warn;

pub fn index() -> Result<Response<Body>, Error> {
    let path = "front/web/index.html";
    file_response(path)
}

// TODO: path escaping etc.
pub fn front_static_files(path: &str) -> Result<Response<Body>, Error> {
    let filepath = format!("front/build/web{}", path);
    file_response(&filepath)
}

// TODO: tokio::fs+Body::wrap_stream
fn file_response(filename: &str) -> Result<Response<Body>, Error> {
    let resp = match fs::read(filename) {
        Ok(content) => {
            let mut resp = Response::new(content.into());
            resp.headers_mut()
                .insert("Content-Type", path_to_mime(&filename));
            resp
        }
        Err(e) => {
            warn!("Can't open file: {}. {}", filename, e);
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("Not found".into())
                .unwrap()
        }
    };

    Ok(resp)
}

fn path_to_mime<'a>(path: &str) -> HeaderValue {
    let s = path.to_lowercase();
    let default: &str = "text/plain;charset=UTF-8";
    let mut ext = "";
    if let Some(p) = s.rfind(".") {
        if &p + 1 < s.len() {
            ext = &s[p + 1..]
        }
    }
    if ext == "" {
        return HeaderValue::from_str(default).unwrap();
    }
    let mime: String = match ext {
        "html" => "text/html;charset=UTF-8".to_string(),
        "css" => "text/css;charset=UTF-8".to_string(),
        "js" => "text/javascript;charset=UTF-8".to_string(),
        "gif" => "image/gif".to_string(),
        "jpg" | "jpeg" => "image/jpeg".to_string(),
        "png" => "image/jpeg".to_string(),
        "svg" => "image/svg+xml".to_string(),
        "webp" => "image/webp".to_string(),
        "woff" | "ttf" | "otf" => format!("font/{}", ext),
        _ => default.to_string(),
    };

    HeaderValue::from_str(&mime).unwrap()
}
