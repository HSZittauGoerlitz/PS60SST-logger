mod logger;
mod ps60sst_config;
mod ps60sst_server;

use logger::Logger;
use ps60sst_config::PS60SSTconfig;
use ps60sst_server::PS60SSTserver;
use std::path::Path;
use std::sync::mpsc;

pub trait SerialDevice {
    fn connect(&mut self) -> bool;
    fn listen(&mut self);
}

fn main() {
    let cfg = PS60SSTconfig::load_json("config.json");

    let (tx, rx) = mpsc::channel::<String>();

    let mut ps60srv = PS60SSTserver::new(cfg.com_port, tx);

    let connected = ps60srv.connect();

    if connected {
        let mut logger = Logger::new(Path::new(&cfg.f_loc), &cfg.f_name);

        std::thread::Builder::new()
            .name("PS60SST communication".to_owned())
            .spawn(move || ps60srv.listen())
            .unwrap();

        loop {
            match rx.recv() {
                Err(e) => {
                    println!("Error receiving message from ps60sst thread: \
                             {:?}", e);
                },
                Ok(data) => logger.log(&data)
            };
        }
    }
}
