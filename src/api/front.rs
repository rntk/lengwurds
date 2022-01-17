use hyper::header::HeaderValue;
use hyper::{Body, Error, Response};

pub fn front_js() -> Result<Response<Body>, Error> {
    let bytes = include_str!("../../front/front.js");
    Ok(Response::new(bytes.into()))
}

pub fn front_css() -> Result<Response<Body>, Error> {
    let bytes = include_str!("../../front/front.css");
    Ok(Response::new(bytes.into()))
}

pub fn index() -> Result<Response<Body>, Error> {
    let html = r#"
<DOCTYPE html>
<html>
<head>
    <script src="/static/front.js"></script>
</head>
<body>
    <h1>Index</h1>
</body>
</html>
    "#;
    let mut resp = Response::new(html.into());

    resp.headers_mut().insert(
        "Content-Type",
        HeaderValue::from_str("text/html; charset=utf-8").unwrap(),
    );

    Ok(resp)
}
