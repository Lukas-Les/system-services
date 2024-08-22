mod threadpool;
mod transaction;


use log::{LevelFilter};
use systemd_journal_logger::JournalLog;

use transaction::Transaction;
use threadpool::ThreadPool;


fn main() {
    JournalLog::new().unwrap().install().unwrap();
    log::set_max_level(LevelFilter::Info);
    
    let pool = ThreadPool::new(4);
    loop {
        pool.execute(|| {
            let trx = Transaction::new();
            trx.run();
        })
    }
}
