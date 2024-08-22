mod transaction;

use core::fmt;
use std::thread::sleep;
use std::time;

use log::{info, LevelFilter};
use systemd_journal_logger::JournalLog;

use transaction::Transaction;


fn main() {
    JournalLog::new().unwrap().install().unwrap();
    log::set_max_level(LevelFilter::Info);
    
    // let messages = vec![Message::Green, Message::Red, Message::Blue];
    let transaction = Transaction::new();
    transaction.run();


    // loop {
    //     messages.iter().for_each(|m| info!("{}", m));
    //     sleep(time::Duration::from_secs(3));
    // }

}
