use actix::prelude::*;

use crate::bot::TelegramBot;
use crate::types::*;

#[derive(Default)]
pub struct Scraper;

impl Actor for Scraper {
    type Context = Context<Self>;
}

impl Supervised for Scraper {}

impl SystemService for Scraper {}

impl Handler<Msg> for Scraper {
    type Result = ();

    fn handle(&mut self, msg: Msg, _ctx: &mut Context<Self>) {
        let bot = TelegramBot::from_registry();

        let send_text = |text| {
            bot.do_send(RunResult {
                from_user: match msg {
                    Msg::User => true,
                    _ => false,
                },
                text,
            })
        };

        println!("Handling message: {:?}", msg);

        let scraper = std::process::Command::new("./scraper").output();

        match scraper {
            Err(e) => send_text(format!("Error running scraper: {}", e)),
            Ok(output) => {
                let code = output.status.code();
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);

                println!("Ran scraper ({:?}):\n{}\n{}", code, stdout, stderr);

                if output.status.success() {
                    send_text(stdout.to_string());
                } else {
                    send_text(format!(
                        "Non-success running scraper ({:?}):\n{}\n{}",
                        code, stdout, stderr
                    ));
                }
            }
        }
    }
}
