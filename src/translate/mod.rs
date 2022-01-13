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
