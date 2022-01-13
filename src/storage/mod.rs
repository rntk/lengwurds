use std::error;
use std::fs;
use std::io;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Serialize, Clone)]
pub struct Word {
    word: String,
    lang: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Translate {
    translates: Vec<Word>,
}

#[derive(Deserialize, Serialize)]
pub struct User {
    translates: Vec<Translate>,
    id: i64,
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
            db,
            path: path.to_string(),
        })
    }

    pub fn save(&self) -> Result<(), Box<dyn error::Error>> {
        let b = serde_json::to_string(&self.db)?;
        fs::write(&self.path, b)?;
        Ok(())
    }

    pub fn add(
        &mut self,
        user_id: i64,
        translate: &Translate,
    ) -> Result<(), Box<dyn error::Error>> {
        let mut pos: i32 = -1;
        for (i, user) in self.db.iter().enumerate() {
            if user.id == user_id {
                pos = i as i32;
                break;
            }
        }
        if pos >= 0 {
            unimplemented!("unimplemented");
        } else {
            self.db.push(User {
                id: user_id,
                translates: vec![translate.clone()],
            })
        }

        self.save()
    }
}
