use actix::prelude::*;

use tsbot::timer::Timer;

fn main() -> Result<(), std::io::Error> {
    let system = System::new("test");

    Timer { interval: 60 * 30 }.start();

    system.run()
}
