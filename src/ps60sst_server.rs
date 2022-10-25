use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};
use std::sync::mpsc::Sender;
use std::time::Duration;
use std::thread::sleep;

use crate::SerialDevice;

pub struct PS60SSTserver {
    port_name: String,
    port: Option<Box<dyn SerialPort>>,
    tx: Sender<String>
}

impl PS60SSTserver {
    pub fn new(port_name: String,
               tx: Sender<String>)
      -> PS60SSTserver
    {
        PS60SSTserver{
            port_name,
            port: None,
            tx
        }
    }

    fn send_value(&self, msg_buffer: &[u8]) {
        match std::str::from_utf8(msg_buffer) {
            Ok(data) => {
                match self.tx.send(data.to_owned()) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Error sending new measurement \
                                  to main process ({:?})", e)
                        }
                }
            },
            Err(e) => {
                println!("Error reading serial data ({:?})", e);
            }
        }
    }
}

impl SerialDevice for PS60SSTserver {
    fn connect(&mut self) -> bool
    {
        let success;
        self.port = match serialport::new(&self.port_name, 9600)
          .flow_control(FlowControl::None)
          .parity(Parity::None)
          .data_bits(DataBits::Eight)
          .stop_bits(StopBits::One)
          .timeout(Duration::from_millis(200))
          .open() {
            Err(e) => {
                print!("Opening port {:?} for PS 60 SST failed.\nError: {}",
                  self.port_name, e);
                success = false;
                None
            },
            Ok(p) => {
                success = true;
                println!("Connected - Port: \
                         {:?}, baud: 9600, timeout: 0,2s\n---",
                          self.port_name);
                Some(p)
            },
        };

        success
    }

    fn listen(&mut self) {
        let mut port_buffer: [u8; 256] = [0; 256];

        loop {
            match self.port.as_deref_mut().unwrap().read(&mut port_buffer) {
                Ok(len) => {
                    self.send_value(&port_buffer[..len]);
                },
                Err(e) => {
                    if e.kind() != std::io::ErrorKind::TimedOut {
                        println!("Error reading data: {}", e);
                    }
                }
            };
            // wait for new msg
            sleep(Duration::from_millis(100));
        }
    }
}
