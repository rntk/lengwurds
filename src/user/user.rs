use std::error;
use std::fmt;
use std::sync::{Arc, RwLock};

use crate::storage;
use crate::storage::{strategy, Storage, Word};
use crate::translate::Lang;
use crate::translate::{google, Translate};

pub struct UserWords {
    storage: Arc<RwLock<Storage>>,
    translator: google::Client,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UserErrorKind {
    NoLang,
    //NoWord,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UserError {
    kind: UserErrorKind,
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            UserErrorKind::NoLang => write!(f, "No added langs"),
            //UserErrorKind::NoWord => write!(f, "Word not found"),
        }
    }
}

impl error::Error for UserError {
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

impl UserWords {
    pub fn new(stor: Arc<RwLock<Storage>>, tran: google::Client) -> UserWords {
        UserWords {
            storage: stor,
            translator: tran,
        }
    }

    pub fn add_word(&mut self, user_id: i64, word: &Word) -> Result<(), Box<dyn error::Error>> {
        let mut stor = self.storage.write().unwrap();
        let langs: Vec<Lang> = match stor.get(user_id) {
            Some(user) => user
                .langs
                .iter()
                .filter(|l| l.lang != word.lang.lang)
                .map(|l| l.clone())
                .collect(),
            None => vec![],
        };
        if langs.is_empty() {
            return Err(Box::new(UserError {
                kind: UserErrorKind::NoLang,
            }));
        }
        let tran = storage::Translate {
            word: word.clone(),
            translates: self.translator.translate_to_langs(&word, langs)?,
        };

        stor.upsert(user_id, strategy::AddTranslate { tran })
    }

    pub fn delete_word(&mut self, user_id: i64, word: &str) -> Result<(), Box<dyn error::Error>> {
        let mut stor = self.storage.write().unwrap();
        stor.upsert(
            user_id,
            strategy::DeleteWord {
                word: word.to_string(),
            },
        )
    }

    pub fn list_words(
        &self,
        user_id: i64,
        pattern: &str,
    ) -> Result<Vec<storage::Translate>, Box<dyn error::Error>> {
        let stor = self.storage.read().unwrap();
        match stor.get(user_id) {
            Some(u) => {
                let p = pattern.to_string().to_lowercase();
                if p != "" {
                    return Ok(u
                        .translates
                        .iter()
                        .filter(|t| t.word.word.contains(&p))
                        .map(|t| t.clone())
                        .collect());
                }
                return Ok(u.translates.to_vec());
            }
            None => Ok(vec![]),
        }
    }

    pub fn add_lang(&mut self, user_id: i64, lang: &Lang) -> Result<(), Box<dyn error::Error>> {
        let mut stor = self.storage.write().unwrap();
        stor.upsert(user_id, strategy::AddLang { lang: lang.clone() })
    }

    pub fn delete_lang(&mut self, user_id: i64, lang: &Lang) -> Result<(), Box<dyn error::Error>> {
        let mut stor = self.storage.write().unwrap();
        stor.upsert(user_id, strategy::DeleteLang { lang: lang.clone() })
    }

    pub fn list_langs(&self, user_id: i64) -> Result<Vec<Lang>, Box<dyn error::Error>> {
        let stor = self.storage.read().unwrap();
        match stor.get(user_id) {
            Some(u) => Ok(u.langs.to_vec()),
            None => Ok(vec![]),
        }
    }
}
