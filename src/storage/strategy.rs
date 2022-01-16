use std::collections::HashSet;

use crate::storage::{Translate, User, Word};
use crate::translate::Lang;

pub trait UserUpdateStrategy {
    fn apply(&self, user: &User) -> User;
}

pub struct DeleteWord {
    pub word: String,
}

impl UserUpdateStrategy for DeleteWord {
    fn apply(&self, user: &User) -> User {
        let mut u = user.clone();
        let mut saved: Vec<Translate> = vec![];
        for tr in user.translates.iter() {
            if tr.word.word == self.word {
                continue;
            }
            saved.push(tr.clone())
        }
        u.translates = saved;

        u
    }
}

pub struct AddTranslate {
    pub tran: Translate,
}

impl UserUpdateStrategy for AddTranslate {
    fn apply(&self, user: &User) -> User {
        let mut u = user.clone();
        let mut found = false;
        for (i, tr) in user.translates.iter().enumerate() {
            if tr == &self.tran {
                found = true;
                break;
            }
            if tr.word == self.tran.word {
                found = true;
                let mut hset: HashSet<Word> = HashSet::new();
                for t in &tr.translates {
                    hset.insert(t.clone());
                }
                for t in &self.tran.translates {
                    if hset.contains(t) {
                        continue;
                    }
                    u.translates[i].translates.push(t.clone())
                }
                break;
            }
        }
        if !found {
            u.translates.push(self.tran.clone())
        }

        u
    }
}

pub struct AddLang {
    pub lang: Lang,
}

impl UserUpdateStrategy for AddLang {
    fn apply(&self, user: &User) -> User {
        let mut u = user.clone();
        for _ in user.langs.iter().filter(|l| l.lang == self.lang.lang) {
            return u;
        }
        u.langs.push(self.lang.clone());

        u
    }
}

// TODO: may be need a strategy to delete all words on this language
pub struct DeleteLang {
    pub lang: Lang,
}

impl UserUpdateStrategy for DeleteLang {
    fn apply(&self, user: &User) -> User {
        let mut u = user.clone();
        u.langs = user
            .langs
            .iter()
            .filter(|l| l.lang != self.lang.lang)
            .map(|l| l.clone())
            .collect();

        u
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::strategy::{
        AddLang, AddTranslate, DeleteLang, DeleteWord, UserUpdateStrategy,
    };
    use crate::storage::{Translate, User, Word};

    #[test]
    fn add_lang() {
        let strat = AddLang {
            lang: "en".parse().unwrap(),
        };
        let mut test_u: Vec<User> = vec![];
        let mut expect_u: Vec<User> = vec![];
        let id = 1;
        {
            let u_test = User::new(id);
            test_u.push(u_test);

            let mut u_expect = User::new(id);
            u_expect.langs.push("en".parse().unwrap());
            expect_u.push(u_expect)
        }
        {
            let mut u_test = User::new(id);
            u_test.langs.push("en".parse().unwrap());
            test_u.push(u_test);

            let mut u_expect = User::new(id);
            u_expect.langs.push("en".parse().unwrap());
            expect_u.push(u_expect)
        }
        {
            let mut u_test = User::new(id);
            u_test.langs.push("ru".parse().unwrap());
            test_u.push(u_test);

            let mut u_expect = User::new(id);
            u_expect.langs.push("ru".parse().unwrap());
            u_expect.langs.push("en".parse().unwrap());
            expect_u.push(u_expect)
        }
        for (i, tst) in test_u.iter().enumerate() {
            assert_eq!(expect_u[i], strat.apply(tst))
        }
    }

    #[test]
    fn del_lang() {
        let strat = DeleteLang {
            lang: "en".parse().unwrap(),
        };
        let mut test_u: Vec<User> = vec![];
        let mut expect_u: Vec<User> = vec![];
        let id = 1;
        {
            let u_test = User::new(id);
            test_u.push(u_test);

            let u_expect = User::new(id);
            expect_u.push(u_expect)
        }
        {
            let mut u_test = User::new(id);
            u_test.langs.push("en".parse().unwrap());
            test_u.push(u_test);

            let u_expect = User::new(id);
            expect_u.push(u_expect)
        }
        {
            let mut u_test = User::new(id);
            u_test.langs.push("ru".parse().unwrap());
            u_test.langs.push("en".parse().unwrap());
            test_u.push(u_test);

            let mut u_expect = User::new(id);
            u_expect.langs.push("ru".parse().unwrap());
            expect_u.push(u_expect)
        }
        for (i, tst) in test_u.iter().enumerate() {
            assert_eq!(expect_u[i], strat.apply(tst), "Failed test: {}", i)
        }
    }

    #[test]
    fn del_word() {
        let strat = DeleteWord {
            word: "word".to_string(),
        };
        let mut test_u: Vec<User> = vec![];
        let mut expect_u: Vec<User> = vec![];
        let id = 1;
        {
            let u_test = User::new(id);
            test_u.push(u_test);

            let u_expect = User::new(id);
            expect_u.push(u_expect)
        }
        {
            let mut u_test = User::new(id);
            u_test.translates.push(Translate {
                word: Word {
                    word: "word".to_string(),
                    lang: "en".parse().unwrap(),
                },
                translates: vec![Word {
                    word: "слово".to_string(),
                    lang: "ru".parse().unwrap(),
                }],
            });
            test_u.push(u_test);

            let u_expect = User::new(id);
            expect_u.push(u_expect)
        }
        {
            let mut u_test = User::new(id);
            u_test.translates.push(Translate {
                word: Word {
                    word: "word".to_string(),
                    lang: "en".parse().unwrap(),
                },
                translates: vec![Word {
                    word: "слово".to_string(),
                    lang: "ru".parse().unwrap(),
                }],
            });
            u_test.translates.push(Translate {
                word: Word {
                    word: "door".to_string(),
                    lang: "en".parse().unwrap(),
                },
                translates: vec![Word {
                    word: "дверь".to_string(),
                    lang: "ru".parse().unwrap(),
                }],
            });
            test_u.push(u_test);

            let mut u_expect = User::new(id);
            u_expect.translates.push(Translate {
                word: Word {
                    word: "door".to_string(),
                    lang: "en".parse().unwrap(),
                },
                translates: vec![Word {
                    word: "дверь".to_string(),
                    lang: "ru".parse().unwrap(),
                }],
            });
            expect_u.push(u_expect)
        }

        for (i, tst) in test_u.iter().enumerate() {
            assert_eq!(expect_u[i], strat.apply(tst), "Failed test: {}", i)
        }
    }

    #[test]
    fn add_translate() {
        let strat = AddTranslate {
            tran: Translate {
                word: Word {
                    word: "word".to_string(),
                    lang: "en".parse().unwrap(),
                },
                translates: vec![Word {
                    word: "слово".to_string(),
                    lang: "ru".parse().unwrap(),
                }],
            },
        };
        let mut test_u: Vec<User> = vec![];
        let mut expect_u: Vec<User> = vec![];
        let id = 1;
        {
            let u_test = User::new(id);
            test_u.push(u_test);

            let mut u_expect = User::new(id);
            u_expect.translates.push(Translate {
                word: Word {
                    word: "word".to_string(),
                    lang: "en".parse().unwrap(),
                },
                translates: vec![Word {
                    word: "слово".to_string(),
                    lang: "ru".parse().unwrap(),
                }],
            });
            expect_u.push(u_expect)
        }
        {
            let mut u_test = User::new(id);
            u_test.translates.push(Translate {
                word: Word {
                    word: "word".to_string(),
                    lang: "en".parse().unwrap(),
                },
                translates: vec![Word {
                    word: "слово".to_string(),
                    lang: "ru".parse().unwrap(),
                }],
            });
            test_u.push(u_test);

            let mut u_expect = User::new(id);
            u_expect.translates.push(Translate {
                word: Word {
                    word: "word".to_string(),
                    lang: "en".parse().unwrap(),
                },
                translates: vec![Word {
                    word: "слово".to_string(),
                    lang: "ru".parse().unwrap(),
                }],
            });
            expect_u.push(u_expect)
        }
        {
            let mut u_test = User::new(id);
            u_test.translates.push(Translate {
                word: Word {
                    word: "word".to_string(),
                    lang: "en".parse().unwrap(),
                },
                translates: vec![Word {
                    word: "сөз".to_string(),
                    lang: "kk".parse().unwrap(),
                }],
            });
            test_u.push(u_test);

            let mut u_expect = User::new(id);
            u_expect.translates.push(Translate {
                word: Word {
                    word: "word".to_string(),
                    lang: "en".parse().unwrap(),
                },
                translates: vec![
                    Word {
                        word: "сөз".to_string(),
                        lang: "kk".parse().unwrap(),
                    },
                    Word {
                        word: "слово".to_string(),
                        lang: "ru".parse().unwrap(),
                    },
                ],
            });
            expect_u.push(u_expect)
        }

        for (i, tst) in test_u.iter().enumerate() {
            assert_eq!(expect_u[i], strat.apply(tst), "Failed test: {}", i)
        }
    }
}
