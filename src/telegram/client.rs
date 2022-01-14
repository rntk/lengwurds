use std::error;

use hyper;
use hyper::body::HttpBody;
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};

const API_URL: &str = "https://api.telegram.org";

#[derive(Deserialize, Serialize)]
pub struct Client {
    token: String,
    last_update: i64,
}

#[derive(Deserialize, Serialize)]
pub struct UpdatesResponse {
    ok: bool,
    result: Vec<Update>,
}

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}

#[derive(Deserialize, Serialize)]
pub struct Update {
    update_id: i64,
    pub message: Message,
}

#[derive(Deserialize, Serialize)]
pub struct Chat {
    pub id: i64,
}

#[derive(Deserialize, Serialize)]
pub struct Message {
    pub message_id: i64,
    pub text: String,
    pub from: User,
    pub chat: Chat,
}

impl Client {
    pub fn new(token: &str) -> Client {
        Client {
            token: token.to_string(),
            last_update: 0,
        }
    }

    pub async fn get_updates(
        &mut self,
        long_poll_seconds: u32,
    ) -> Result<Vec<Update>, Box<dyn error::Error>> {
        let updates = self.get_updates_(long_poll_seconds).await?;
        let last = updates.result.len();
        if last > 0 {
            self.last_update = updates.result[last - 1].update_id
        }

        Ok(updates.result)
    }

    async fn get_updates_(
        &self,
        long_poll_seconds: u32,
    ) -> Result<UpdatesResponse, Box<dyn error::Error>> {
        let mut timeout = "".to_string();
        if long_poll_seconds > 0 {
            timeout = format!("&timeout={}", long_poll_seconds);
        }
        let uri = format!(
            "{}/bot{}/getUpdates?offset={}{}",
            API_URL,
            self.token,
            self.last_update + 1,
            timeout
        );
        let https = HttpsConnector::new();
        let client = hyper::Client::builder().build::<_, hyper::Body>(https);
        let mut resp = client.get(uri.parse()?).await?;
        let mut body: Vec<u8> = vec![];
        while let Some(chunk) = resp.body_mut().data().await {
            let bt = chunk?;
            for b in bt.iter() {
                body.push(*b)
            }
        }
        let res: UpdatesResponse = serde_json::from_slice(body.as_slice())?;

        Ok(res)
    }
}
