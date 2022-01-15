use log::{debug, error, info, warn};
use serialport::SerialPort;
use std::time::Duration;
use std::{thread, time};
use std::io::{self, Read, Write};


pub struct SerialClient {
    pub port: Box<dyn serialport::SerialPort>,
    pub write_locked: bool,
    pub serial_datas: Vec<String>,
}

impl SerialClient {
    pub fn new(baudrate: u32, port_path: &str) -> Self {
        let port = Self::connect(baudrate, port_path.to_string());
        Self {
            port,
            write_locked: false,
            serial_datas: vec![],
        }
    }

    fn connect(baudrate: u32, port_path: String) -> Box<dyn SerialPort> {
        let port = serialport::new(port_path.to_string(), baudrate)
            .timeout(Duration::from_millis(10))
            .open();

        match port {
            Ok(port) => {
                info!("SHIELD: Connected to {}", port_path);
                port
            }
            Err(e) => {
                panic!("SHIELD: Failed to connect to {}: {}", port_path, e);
            }
        }
    }

    pub fn disconnect(self) {
        info!("SHIELD: Disconnected from {}", self.port.name().unwrap());
        drop(self.port);
    }

    pub fn send(&mut self, bytes: &[u8]) -> bool {
        let string = match std::str::from_utf8(&bytes[..bytes.len() - 2]) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        if !self.write_locked {
            self.write_locked = true;
            match self.port.write(bytes) {
                Ok(_) => {
                    debug!("SHIELD: Sent {}", string);
                    self.write_locked = false;
                    true
                }
                Err(e) => {
                    error!("SHIELD: Failed to send {}: {}", string, e);
                    self.write_locked = false;
                    false
                }
            }
        } else {
            debug!(
                "SHIELD: Command not sent, waiting for serial unlock: {}",
                string
            );
            false
        }
    }

    fn push_serial_data(&mut self, string: String) {
        self.serial_datas.push(string);
    }

    pub fn read_incoming_raw_data(&mut self) -> Vec<String> {
        let mut serial_buf: Vec<u8> = vec![0; 1000];
        let mut data_vec: Vec<String> = vec![];
        match self.port.read(serial_buf.as_mut_slice()) {
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
            Ok(_) => {
                match self.parse_at_response_to_string(serial_buf) {
                    Some(v) => {
                        println!("V: {}", v);
                        data_vec.push(v);
                    }
                    None => { warn!("SHIELD: No data found in serial buffer"); }
                }
            }
        }
        return data_vec;
    }

    pub fn parse_at_response_to_string(&mut self, buf: Vec<u8>) -> Option<String> {
        let mut string: String = String::new();
        let mut input = buf.iter();
        while let Some(byte) = input.next() {
            if *byte == b'\n' {
                debug!("N: Received data: {:?}", string);
                string.push(*byte as char);
                return Some(string)
            } else {
                string.push(*byte as char);
            }
        }
        None
    }
}
