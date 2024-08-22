use std::thread::sleep;
use std::time;


use log::{info};
use rand::{Rng, thread_rng};


pub struct Transaction {
    pub id: String,
}

impl Transaction {
    pub fn new() -> Self {
        let id = Self::generate_id(32);
        Self {
            id: id
        }
    }

    fn generate_id(length: usize) -> String {
        let mut rng = thread_rng();
        let bytes: Vec<u8> = (0..length / 2).map(|_| rng.gen::<u8>()).collect();
        bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
    }

    pub fn run(&self) {
        info!("Started: {}", &self.id);
        let mut rng = rand::thread_rng();
        let dur: u64 = rng.gen_range(500..5000);
        sleep(time::Duration::from_millis(dur));
        info!("Finnished: {}", &self.id);
    }
}
