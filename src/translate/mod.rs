use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct Lang {
    pub lang: String,
}

#[derive(Debug, PartialEq)]
pub struct LangParseError {
    pub description: String,
}

impl fmt::Display for LangParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

const SUPPORTED_LANGS: [&str; 3] = ["ru", "en", "kz"];

impl FromStr for Lang {
    type Err = LangParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut st = s.to_lowercase();
        st = st.trim().to_string();
        for l in SUPPORTED_LANGS {
            if st == l {
                return Ok(Lang { lang: st });
            }
        }

        Err(LangParseError {
            description: "Unsupportet language".to_string(),
        })
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lang)
    }
}

pub struct Translator {
    token: String,
}

impl Translator {
    pub fn new(token: &str) -> Translator {
        Translator {
            token: token.to_string(),
        }
    }
}
