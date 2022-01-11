use log::{debug, error, info};
use serialport::SerialPort;
use std::io::{Read, Write};

pub struct SerialClient {
    port: Box<dyn serialport::SerialPort>,
    pub write_locked: bool,
    pub serial_datas: Vec<u8>,
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
        let port = serialport::new(port_path.to_string(), baudrate).open();

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

    fn push_serial_data(&mut self, byte: &u8) {
        self.serial_datas.push(*byte);
    }

    pub fn read_incoming_raw_data(&mut self) {
        let mut serial_data_raw: Vec<u8> = vec![];
        let mut serial_buf: Vec<u8> = vec![0; 32];
        match self.port.read(serial_buf.as_mut_slice()) {
            Err(e) => {
                error!("SHIELD: Error reading incoming data: {}", e);
            }
            Ok(_) => {
                serial_buf.iter().for_each(|byte| {
                    if b'\n' == *byte {
                        let string = &String::from_utf8(serial_data_raw.clone()).unwrap();
                        debug!("SHIELD: Received data: {:?}", string);

                        self.push_serial_data(byte);
                        serial_data_raw = vec![];
                    } else {
                        serial_data_raw.push(*byte);
                    }
                });
            }
        }
    }
}
