use std::error;
use std::fmt;

use hyper::{Body, Request};
use serde::Deserialize;
use serde_qs;

#[derive(Deserialize)]
pub struct UserId {
    pub user_id: i64,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Error {
    description: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Response: {}", &self.description)
    }
}

impl error::Error for Error {
    /*fn description(&self) -> &str {
        let m = format!("{}", &self);
        m.as_str()
    }*/

    /*fn cause(&self) -> Option<&(dyn error::Error)> {
        None
    }*/

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub fn user_id(req: &Request<Body>) -> Result<UserId, Box<dyn error::Error>> {
    let q = match req.uri().query() {
        Some(q) => q,
        None => {
            return Err(Box::new(Error {
                description: "No query".to_string(),
            }))
        }
    };
    let u_id: UserId = serde_qs::from_str(q)?;

    Ok(u_id)
}
