use crate::telegram::client;

use log::{error, info};
use tokio::runtime::Builder;

pub fn updates_fetching(token: String) {
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
            process_update(&update)
        }
    }
}

fn process_update(update: &client::Update) {
    info!("{}", update.message.text)
}
