use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use std::thread::sleep;
use std::time::Duration;

use crate::storage::Word;
use crate::telegram::client;
use crate::user::user::UserWords;

use crate::telegram::commands::Command;
use log::{error, warn};
use rand;
use rand::Rng;
use tokio::runtime::Builder;

pub fn updates_processing(user_words: Arc<RwLock<UserWords>>, token: String) {
    let mut cli = client::Client::new(&token);
    let rt = Builder::new_current_thread().enable_all().build().unwrap();
    loop {
        let updates = match rt.block_on(cli.get_updates(60)) {
            Ok(updates) => updates,
            Err(e) => {
                error!("Telegram updates error: {}", e);
                sleep(Duration::from_secs(5));
                continue;
            }
        };
        for update in updates {
            let message = match update.message {
                Some(msg) => msg,
                None => match update.edited_message {
                    Some(msg) => msg,
                    None => continue,
                },
            };
            let cmd: Command = match message.text.parse() {
                Ok(cmd) => cmd,
                Err(e) => {
                    warn!(
                        "Can't parse command from: {}. {}. {}",
                        message.chat.id, message.text, e
                    );
                    let r = rt.block_on(cli.send_msg(&client::Answer {
                        chat_id: message.chat.id,
                        text: format!("Can't parse command: {}", e),
                        reply_to_message_id: message.message_id,
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
                        user_w.add_lang(message.chat.id, &lang)
                    };
                    match r {
                        Ok(()) => list_langs_answer(user_words.clone(), &message),
                        Err(e) => Err(e),
                    }
                }
                Command::DeleteLang(lang) => {
                    let r = {
                        let mut user_w = user_words.write().unwrap();
                        user_w.delete_lang(message.chat.id, &lang)
                    };
                    match r {
                        Ok(()) => list_langs_answer(user_words.clone(), &message),
                        Err(e) => Err(e),
                    }
                }
                Command::AddWord(word) => {
                    let r = {
                        let mut user_w = user_words.write().unwrap();
                        user_w.add_word(message.chat.id, &word)
                    };
                    match r {
                        Ok(()) => list_words_answer(user_words.clone(), &message, &word.word),
                        Err(e) => Err(e),
                    }
                }
                Command::DeleteWord(word) => {
                    let mut user_w = user_words.write().unwrap();
                    match user_w.delete_word(message.chat.id, &word) {
                        Ok(()) => Ok(client::Answer::from_message("Word deleted", &message)),
                        Err(e) => Err(e),
                    }
                }
                Command::ListWords(pattern) => {
                    list_words_answer(user_words.clone(), &message, &pattern)
                }
                Command::ListLangs => list_langs_answer(user_words.clone(), &message),
                Command::ListRandomWords(n) => {
                    let mut user_w = user_words.write().unwrap();
                    match user_w.list_words(message.chat.id, None) {
                        Ok(mut trs) => {
                            let mut trs_s: Vec<String> = vec![];
                            let mut words: Vec<Word> = vec![];
                            let mut len = trs.len() / 2;
                            trs.sort_by_key(|k| k.last_seen);
                            if len < n as usize {
                                len = n as usize;
                            }
                            let mut uniq: HashSet<usize> = HashSet::new();
                            while trs_s.len() < n as usize {
                                let s: usize = rand::thread_rng().gen_range(0..len);
                                if uniq.contains(&s) {
                                    continue;
                                }
                                uniq.insert(s);
                                trs_s.push(format!("{}\n", &trs[s]));
                                words.push(trs[s].word.clone())
                            }
                            if !words.is_empty() {
                                if let Err(e) = user_w.update_last_seen(message.chat.id, words) {
                                    error!("Can't update last seen for: {}. {}", message.chat.id, e)
                                }
                            }
                            let mut msg = trs_s.concat();
                            if msg == "" {
                                msg = "No words".to_string()
                            }
                            Ok(client::Answer::from_message(&msg, &message))
                        }
                        Err(e) => Err(e),
                    }
                }
            };
            let answer = match answer_res {
                Ok(answer) => answer,
                Err(e) => {
                    error!(
                        "Can't process command from: {}. {}. {}",
                        message.chat.id, message.text, e
                    );
                    let answer = client::Answer {
                        chat_id: message.chat.id,
                        text: format!("Can't process command: {}", e),
                        reply_to_message_id: message.message_id,
                    };
                    answer
                }
            };
            if let Err(e) = rt.block_on(cli.send_msg(&answer)) {
                error!(
                    "Can't send telegram answer to: '{}'. {}. {}",
                    message.text, answer, e
                )
            }
        }
    }
}

fn list_words_answer(
    user_words: Arc<RwLock<UserWords>>,
    message: &client::Message,
    pattern: &str,
) -> Result<client::Answer, Box<dyn std::error::Error>> {
    let user_w = user_words.read().unwrap();
    match user_w.list_words(message.chat.id, Some(pattern)) {
        Ok(trs) => {
            let trs_s: Vec<String> = trs.iter().map(|tr| format!("{}\n", tr)).collect();
            let mut msg = trs_s.concat();
            if msg == "" {
                msg = "No words".to_string()
            }
            Ok(client::Answer::from_message(&msg, &message))
        }
        Err(e) => Err(e),
    }
}

fn list_langs_answer(
    user_words: Arc<RwLock<UserWords>>,
    message: &client::Message,
) -> Result<client::Answer, Box<dyn std::error::Error>> {
    let user_w = user_words.read().unwrap();
    let langs = user_w.list_langs(message.chat.id)?;
    let langs_s: Vec<String> = langs.iter().map(|l| format!(" {} ", l.lang)).collect();
    let mut msg = langs_s.concat();
    if msg == "" {
        msg = "No langs".to_string()
    }
    Ok(client::Answer::from_message(&msg, &message))
}
