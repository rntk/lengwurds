use std::sync::{Arc, RwLock};

use crate::telegram::client;
use crate::user::user::UserWords;

use crate::telegram::commands::Command;
use log::{error, warn};
use tokio::runtime::Builder;

pub fn updates_processing(user_words: Arc<RwLock<UserWords>>, token: String) {
    let mut cli = client::Client::new(token.as_str());
    let rt = Builder::new_current_thread().enable_all().build().unwrap();
    loop {
        let updates = match rt.block_on(cli.get_updates(60)) {
            Ok(updates) => updates,
            Err(e) => {
                error!("Telegram updates error: {}", e);
                vec![]
            }
        };
        for update in updates {
            let cmd: Command = match update.message.text.parse() {
                Ok(cmd) => cmd,
                Err(e) => {
                    warn!(
                        "Can't parse command from: {}. {}. {}",
                        update.message.chat.id, update.message.text, e
                    );
                    let r = rt.block_on(cli.send_msg(&client::Answer {
                        chat_id: update.message.chat.id,
                        text: format!("Can't parse command: {}", e),
                        reply_to_message_id: update.message.message_id,
                    }));
                    if let Err(e) = r {
                        error!("Can't send telegram error message: {}", e);
                    };
                    continue;
                }
            };
            let answer_res = match cmd {
                Command::AddLang(lang) => {
                    let r = {
                        let mut user_w = user_words.write().unwrap();
                        user_w.add_lang(update.message.chat.id, &lang)
                    };
                    match r {
                        Ok(()) => list_langs_answer(user_words.clone(), &update),
                        Err(e) => Err(e),
                    }
                }
                Command::DeleteLang(lang) => {
                    let r = {
                        let mut user_w = user_words.write().unwrap();
                        user_w.delete_lang(update.message.chat.id, &lang)
                    };
                    match r {
                        Ok(()) => list_langs_answer(user_words.clone(), &update),
                        Err(e) => Err(e),
                    }
                }
                Command::AddWord(word) => {
                    let r = {
                        let mut user_w = user_words.write().unwrap();
                        user_w.add_word(update.message.chat.id, &word)
                    };
                    match r {
                        Ok(()) => {
                            list_words_answer(user_words.clone(), &update, word.word.as_str())
                        }
                        Err(e) => Err(e),
                    }
                }
                Command::DeleteWord(word) => {
                    let mut user_w = user_words.write().unwrap();
                    match user_w.delete_word(update.message.chat.id, &word) {
                        Ok(()) => Ok(client::Answer::from_update("Word deleted", &update)),
                        Err(e) => Err(e),
                    }
                }
                Command::ListWords(pattern) => {
                    list_words_answer(user_words.clone(), &update, pattern.as_str())
                }
                Command::ListLangs => list_langs_answer(user_words.clone(), &update),
            };
            let answer = match answer_res {
                Ok(answer) => answer,
                Err(e) => {
                    error!(
                        "Can't process command from: {}. {}. {}",
                        update.message.chat.id, update.message.text, e
                    );
                    let answer = client::Answer {
                        chat_id: update.message.chat.id,
                        text: format!("Can't process command: {}", e),
                        reply_to_message_id: update.message.message_id,
                    };
                    answer
                }
            };
            if let Err(e) = rt.block_on(cli.send_msg(&answer)) {
                error!(
                    "Can't send telegram answer to: '{}'. {}. {}",
                    update.message.text, answer, e
                )
            }
        }
    }
}

fn list_words_answer(
    user_words: Arc<RwLock<UserWords>>,
    update: &client::Update,
    pattern: &str,
) -> Result<client::Answer, Box<dyn std::error::Error>> {
    let user_w = user_words.read().unwrap();
    match user_w.list_words(update.message.chat.id, pattern) {
        Ok(trs) => {
            let trs_s: Vec<String> = trs.iter().map(|tr| format!("{}\n", tr)).collect();
            let mut msg = trs_s.concat();
            if msg == "" {
                msg = "No words".to_string()
            }
            Ok(client::Answer::from_update(msg.as_str(), &update))
        }
        Err(e) => Err(e),
    }
}

fn list_langs_answer(
    user_words: Arc<RwLock<UserWords>>,
    update: &client::Update,
) -> Result<client::Answer, Box<dyn std::error::Error>> {
    let user_w = user_words.read().unwrap();
    let langs = user_w.list_langs(update.message.chat.id)?;
    let langs_s: Vec<String> = langs.iter().map(|l| format!(" {} ", l.lang)).collect();
    let mut msg = langs_s.concat();
    if msg == "" {
        msg = "No langs".to_string()
    }
    Ok(client::Answer::from_update(msg.as_str(), &update))
}
