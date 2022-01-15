use crate::translate::{Lang, Translate, Word};
use std::error::Error;

use hyper;
use hyper::body::HttpBody;
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use tokio::runtime::Builder;

const API_URL: &str = "https://translation.googleapis.com/language/translate/v2";

#[derive(Serialize)]
struct Query {
    q: String,
    target: String,
    //format: String,
    source: String,
    //model: String,
    key: String,
}

#[derive(Deserialize)]
struct TranslatesResponse {
    pub data: TranslateTextResponseList,
}

#[derive(Deserialize)]
struct TranslateTextResponseList {
    pub translations: Vec<TranslateTextResponseTranslation>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TranslateTextResponseTranslation {
    //detectedSourceLanguage: String,
    //model: String,
    pub translated_text: String,
}

pub struct Client {
    token: String,
}

impl Client {
    pub fn new(token: &str) -> Client {
        Client {
            token: token.to_string(),
        }
    }
}

impl Client {
    pub async fn async_translate(
        &self,
        word: &Word,
        to: &Lang,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let q = Query {
            q: word.word.to_string(),
            target: to.lang.to_string(),
            source: word.lang.lang.to_string(),
            key: self.token.to_string(),
        };
        let https = HttpsConnector::new();
        let client = hyper::Client::builder().build::<_, hyper::Body>(https);
        let url = format!("{}?key={}", API_URL, self.token);
        let req = hyper::Request::builder()
            .method(hyper::Method::POST)
            .uri(url)
            .body(hyper::Body::from(serde_json::to_string(&q)?))?;
        let mut resp = client.request(req).await?;
        let mut body: Vec<u8> = vec![];
        while let Some(chunk) = resp.body_mut().data().await {
            let bt = chunk?;
            for b in bt.iter() {
                body.push(*b)
            }
        }
        /*println!(
            "{:?} \n {}",
            String::from_utf8(body.to_vec()),
            serde_json::to_string(&q)?
        );*/
        let res: TranslatesResponse = serde_json::from_slice(body.as_slice())?;

        let mut trs: Vec<String> = vec![];
        for t in res.data.translations {
            trs.push(t.translated_text.to_string())
        }

        Ok(trs)
    }
}

impl Translate for Client {
    fn translate(&self, word: &Word, to: &Lang) -> Result<Vec<String>, Box<dyn Error>> {
        let rt = Builder::new_current_thread().enable_all().build().unwrap();
        rt.block_on(self.async_translate(word, to))
    }
    fn translate_to_langs(
        &self,
        word: &Word,
        langs: Vec<Lang>,
    ) -> Result<Vec<Word>, Box<dyn Error>> {
        let mut res = vec![];
        for lang in langs {
            let trs = self.translate(word, &lang)?;
            for w in trs {
                res.push(Word {
                    word: w,
                    lang: lang.clone(),
                })
            }
        }

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::translate::google::Client;
    use crate::translate::{Lang, Translate, Word};
    use std::env;

    #[test]
    fn translate() {
        //cargo test -- --show-output
        let translate_token = match env::var("LW_TRANSLATE") {
            Ok(t) => t,
            _ => {
                println!("Skip google translate test");
                "".to_string()
            }
        };
        if translate_token == "" {
            return;
        }
        let g = Client::new(translate_token.as_str());
        match g.translate(
            &Word {
                word: "word".to_string(),
                lang: Lang {
                    lang: "en".to_string(),
                },
            },
            &Lang {
                lang: "ru".to_string(),
            },
        ) {
            Ok(trs) => {
                trs.iter().for_each(|s| println!("Translate: {}\n", s));
                return;
            }
            Err(e) => assert!(false, "{}", e),
        }
    }
}