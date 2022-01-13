use std::error;

use hyper;
use hyper::body::HttpBody;
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
    id: i64,
    username: String,
}

#[derive(Deserialize, Serialize)]
pub struct Update {
    update_id: i64,
    message: Message,
}

#[derive(Deserialize, Serialize)]
pub struct Chat {
    id: i64,
}

#[derive(Deserialize, Serialize)]
pub struct Message {
    message_id: i64,
    text: String,
    from: User,
    chat: Chat,
}

impl Client {
    pub fn new(token: &str) -> Client {
        Client {
            token: token.to_string(),
            last_update: 0,
        }
    }

    pub async fn get_updates(
        self,
        long_poll_seconds: u32,
    ) -> Result<UpdatesResponse, Box<dyn error::Error>> {
        unimplemented!("yo");
    }

    async fn get_updates_(
        self,
        long_poll_seconds: u32,
    ) -> Result<UpdatesResponse, Box<dyn error::Error>> {
        let client = hyper::Client::new();
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
        )
        .parse()?;
        let mut resp = client.get(uri).await?;
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
