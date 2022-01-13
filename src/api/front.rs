use hyper::{Body, Response, Error};

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
    <script src="/static/fron.js"></script>
</head>
<body>
    <h1>Index</h1>
</body>
</html>
    "#;
    Ok(Response::new(html.into()))
}