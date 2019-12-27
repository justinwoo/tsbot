use actix::prelude::*;

use crate::scraper::Scraper;
use crate::types::*;

pub struct Timer {
    pub interval: u64,
}

impl Actor for Timer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let scraper = Scraper::from_registry();

        scraper.do_send(Msg::Timer);

        ctx.run_interval(
            std::time::Duration::from_secs(self.interval),
            move |act, _ctx| {
                println!("Sending from {}", act.interval);
                scraper.do_send(Msg::Timer);
            },
        );
    }
}
