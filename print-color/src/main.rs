use core::fmt;
use std::thread::sleep;
use std::time;

use log::{info, LevelFilter};
use systemd_journal_logger::JournalLog;


enum Message {
    Green,
    Red,
    Blue,
}


impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Red => writeln!(f, "Red"),
            Self::Green => writeln!(f, "Green"),
            Self::Blue => writeln!(f, "Blue"),
        }
    }
}


fn main() {
    JournalLog::new().unwrap().install().unwrap();
    log::set_max_level(LevelFilter::Info);
    
    let messages = vec![Message::Green, Message::Red, Message::Blue];

    loop {
        messages.iter().for_each(|m| info!("{}", m));
        sleep(time::Duration::from_secs(3));
    }

}
