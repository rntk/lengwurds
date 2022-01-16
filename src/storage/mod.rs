pub mod strategy;

use std::error;
use std::fmt;
use std::fs;
use std::io;

use crate::translate::Lang;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug, Eq, Hash)]
pub struct Word {
    pub word: String,
    pub lang: Lang,
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.word, self.lang)
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct Translate {
    pub translates: Vec<Word>,
    pub word: Word,
}

impl fmt::Display for Translate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = format!(
            "{}\t{}\n",
            self.word.lang.lang.to_uppercase(),
            self.word.word,
        );
        for w in &self.translates {
            s.push_str(format!("{}\t{}\n", w.lang.lang.to_uppercase(), w.word).as_str());
        }

        write!(f, "{}", s)
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct User {
    pub translates: Vec<Translate>,
    pub langs: Vec<Lang>,
    pub id: i64,
}

impl User {
    pub fn new(id: i64) -> User {
        User {
            id: id,
            translates: vec![],
            langs: vec![],
        }
    }
}

pub struct Storage {
    db: Vec<User>,
    path: String,
}

impl Storage {
    pub fn new(path: &str) -> Result<Storage, Box<dyn error::Error>> {
        let raw_json = match fs::read_to_string(path) {
            Ok(raw_json) => raw_json,
            Err(e) => {
                if io::ErrorKind::NotFound != e.kind() {
                    return Err(Box::new(e));
                }
                fs::File::create(path)?;
                String::from("")
            }
        };
        if raw_json.trim() == "" {
            return Ok(Storage {
                db: vec![],
                path: path.to_string(),
            });
        }
        let db: Vec<User> = serde_json::from_str(raw_json.as_str())?;

        Ok(Storage {
            db: db,
            path: path.to_string(),
        })
    }

    pub fn save(&self) -> Result<(), Box<dyn error::Error>> {
        let b = serde_json::to_string(&self.db.to_vec())?;
        fs::write(&self.path, b)?;
        Ok(())
    }

    pub fn get(&self, user_id: i64) -> Option<User> {
        for user in self.db.iter().filter(|u| u.id == user_id) {
            return Some(user.clone());
        }

        None
    }

    pub fn upsert(
        &mut self,
        user_id: i64,
        strat: impl strategy::UserUpdateStrategy,
    ) -> Result<(), Box<dyn error::Error>> {
        let mut pos: i32 = -1;
        for (i, u) in self.db.iter().enumerate() {
            if u.id == user_id {
                pos = i as i32;
                break;
            }
        }
        if pos >= 0 {
            self.db[pos as usize] = strat.apply(&self.db[pos as usize])
        } else {
            let user = User::new(user_id);
            self.db.push(strat.apply(&user))
        }

        self.save()
    }

    // unused
    /*pub fn delete(&mut self, user: User) -> Result<(), Box<dyn error::Error>> {
        for (i, u) in self.db.iter().enumerate() {
            if u.id == user.id {
                self.db.swap_remove(i);
                break;
            }
        }

        self.save()
    }*/
}
