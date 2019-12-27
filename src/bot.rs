use actix::prelude::*;
use url::percent_encoding::{utf8_percent_encode, SIMPLE_ENCODE_SET};

use crate::scraper::Scraper;
use crate::telegram;
use crate::types::*;

#[derive(Default)]
pub struct TelegramBot {
    pub token: String,
    pub master: String,
    pub last_update_id: Option<u64>,
}

impl Supervised for TelegramBot {}

impl SystemService for TelegramBot {
    fn service_started(&mut self, _ctx: &mut Context<Self>) {
        println!("Telegram bot service started");
    }
}

impl Actor for TelegramBot {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();
        self.master = std::env::var("TELEGRAM_BOT_MASTER").unwrap();

        let scraper = Scraper::from_registry();

        ctx.run_interval(std::time::Duration::from_secs(5), move |act, _ctx| {
            let method_params = format!(
                "getUpdates?timeout=1{}",
                match act.last_update_id {
                    // prepare offset, which is last update id + 1
                    Some(id) => format!("&offset={}", id + 1),
                    None => "".to_owned(),
                }
            );

            let res: MyResult<telegram::Response> = try {
                let url = telegram::get_url(&act.token, &method_params)?;
                let mut res = reqwest::get(url)
                    .or_else(|e| Err(format!("Failed to fetch getUpdates with error: {}", e)))?;
                res.json()
                    .or_else(|e| Err(format!("Failed to parse telegram res body: {}", e)))?
            };

            match res {
                Ok(res) if res.ok => {
                    let mut should_update = false;

                    for update in res.result {
                        act.last_update_id = Some(update.update_id);

                        let _: Option<()> = try {
                            let message = update.message?;
                            let from = message.from?;
                            let text = message.text?;
                            if from.id.to_string() == act.master && text == "get" {
                                should_update = true;
                            }
                        };
                    }

                    if should_update {
                        println!("Should update based on user message");
                        scraper.do_send(Msg::User);
                    }
                }
                Ok(res) => {
                    scraper.do_send(Msg::Error(format!(
                        "Telegram API problem, res.ok was not true: {:?}",
                        res.description
                    )));
                }
                Err(err) => {
                    scraper.do_send(Msg::Error(err));
                }
            }
        });
    }
}

impl Handler<RunResult> for TelegramBot {
    type Result = ();

    fn handle(&mut self, res: RunResult, _ctx: &mut Context<Self>) {
        println!("Handling run result: {:?}", res);

        let method_params = format!(
            "sendMessage?chat_id={}&text={}",
            self.master,
            utf8_percent_encode(&res.text, SIMPLE_ENCODE_SET)
        );

        let res: MyResult<()> = try {
            let url = telegram::get_url(&self.token, &method_params)?;
            let _ = reqwest::get(url).or_else(|e| Err(format!("Failed to send message: {}", e)))?;
        };

        if let Err(e) = res {
            println!("Error sending telegram message: {}", e);
        }
    }
}
