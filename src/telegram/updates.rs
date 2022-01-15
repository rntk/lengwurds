use std::sync::{Arc, RwLock};

use crate::storage;
use crate::telegram::client;
use crate::user::user::UserWords;

use crate::telegram::commands::Command;
use log::{error, info, warn};
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
        let mut user_w = user_words.write().unwrap();
        for update in updates {
            let cmd: Command = match update.message.text.parse() {
                Ok(cmd) => cmd,
                Err(e) => {
                    warn!("Can't parse command: {}. {}", update.message.text, e);
                    cli.send_msg(&client::Answer {
                        chat_id: update.message.chat.id,
                        text: "OK".to_string(),
                        reply_to_message_id: update.message.message_id,
                    });
                    continue;
                }
            };
            /*let res = match cmd {
                Command::AddLang(lang) => user_w.add_lang(update.message.chat.id, &lang),
                Command::DeleteLang(lang) => user_w.delete_lang(update.message.chat.id, &lang),
                Command::AddWord(word) => user_w.add_word(update.message.chat.id, &word),
                Command::DeleteWord(word) => user_w.delete_word(update.message.chat.id, &word),
                Command::ListWords(pattern) => {
                    user_w.list_words(update.message.chat.id, pattern.as_str())
                }
                Command::ListLangs => user_w.list_langs(update.message.chat.id),
            };*/
            /*let answer = match res {
                Ok() => client::Answer{
                    chat_id: update.message.chat.id,
                    text: "OK".to_string(),
                    reply_to_message_id: update.message.message_id
                },
                Err(e) => {
                    client::Answer{
                        chat_id: update.message.chat.id,
                        text: "OK".to_string(),
                        reply_to_message_id: update.message.message_id
                    }
                }
            }*/
        }
    }
}

/*fn process_update(user_words: Arc<UserWords>, update: &client::Update) -> Result {
    info!("{}", update.message.text)
}*/
