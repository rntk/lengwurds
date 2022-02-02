use std::fmt;
use std::{error, time};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_message: Option<Message>,
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

#[derive(Serialize, Debug)]
pub struct Answer {
    pub reply_to_message_id: i64,
    //reply_markup
    pub chat_id: i64,
    pub text: String,
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Chat: {}. To messge: {}. Text: {}",
            self.chat_id, self.reply_to_message_id, self.text
        )
    }
}

impl Answer {
    /*pub fn from_update(msg: &str, update: &Update) -> Answer {
        Answer {
            reply_to_message_id: update.message.message_id,
            chat_id: update.message.chat.id,
            text: msg.to_string(),
        }
    }*/
    pub fn from_message(msg: &str, message: &Message) -> Answer {
        Answer {
            reply_to_message_id: message.message_id,
            chat_id: message.chat.id,
            text: msg.to_string(),
        }
    }
}

#[derive(Deserialize)]
struct SendMessageResponse {
    ok: bool,
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

impl Client {
    pub fn new(token: &str) -> Client {
        Client {
            token: token.to_string(),
            last_update: 0,
        }
    }

    pub async fn get_updates(
        &mut self,
        long_poll: time::Duration,
    ) -> Result<Vec<Update>, Box<dyn error::Error>> {
        let updates = self.get_updates_(long_poll).await?;
        let last = updates.result.len();
        if last > 0 {
            self.last_update = updates.result[last - 1].update_id
        }

        Ok(updates.result)
    }

    async fn get_updates_(
        &self,
        long_poll: time::Duration,
    ) -> Result<UpdatesResponse, Box<dyn error::Error>> {
        let mut timeout = "".to_string();
        if long_poll.as_secs() > 0 {
            timeout = format!("&timeout={}", long_poll.as_secs());
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
        match serde_json::from_slice(body.as_slice()) {
            Ok(r) => Ok(r),
            Err(e) => Err(Box::new(Error {
                description: format!("{}. {}", e, String::from_utf8(body)?),
            })),
        }
    }

    pub async fn send_msg(&self, msg: &Answer) -> Result<(), Box<dyn error::Error>> {
        let form = serde_qs::to_string(msg)?;

        let url = format!("{}/bot{}/sendMessage", API_URL, self.token);
        let https = HttpsConnector::new();
        let client = hyper::Client::builder().build::<_, hyper::Body>(https);
        let req = hyper::Request::builder()
            .method(hyper::Method::POST)
            .uri(url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(hyper::Body::from(form.as_bytes().to_vec()))?;
        let mut resp = client.request(req).await?;
        let mut body: Vec<u8> = vec![];
        while let Some(chunk) = resp.body_mut().data().await {
            let bt = chunk?;
            for b in bt.iter() {
                body.push(*b)
            }
        }
        //println!("{:?} \n {}", String::from_utf8(body.to_vec()), &form);
        let res: SendMessageResponse = serde_json::from_slice(body.as_slice())?;
        if !res.ok {
            return Err(Box::new(Error {
                description: String::from_utf8(body.to_vec())?,
            }));
        }

        Ok(())
    }
}
