use log::{debug, error, info};
use serialport::SerialPort;
use std::io::{Read, Write};

pub struct SerialClient {
    port: Box<dyn serialport::SerialPort>,
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
        let port = serialport::new(port_path.to_string(), baudrate).open();

        match port {
            Ok(port) => {
                info!("GSMHAT: Connected to {}", port_path);
                return port;
            }
            Err(e) => {
                panic!("GSMHAT: Failed to connect to {}: {}", port_path, e);
            }
        }
    }

    pub fn disconnect(self) {
        info!("GSMHAT: Disconnected from {}", self.port.name().unwrap());
        drop(self.port);
    }

    pub fn send_to_hat(&mut self, string: String) -> bool {
        if !self.write_locked {
            self.write_locked = true;
            match self.port.write(format!("{}\n", &string.clone()).as_bytes()) {
                Ok(_) => {
                    debug!("GSMHAT: Sent {}", string);
                    self.serial_datas.push(string);
                    self.write_locked = false;
                    return true;
                }
                Err(e) => {
                    error!("GSMHAT: Failed to send {}: {}", string, e);
                    self.write_locked = false;
                    return false;
                }
            };
        } else {
            debug!(
                "GSMHAT: Command not sent, waiting for serial unlock: {}",
                string
            );
            return false;
        }
    }

    fn push_serial_data(&mut self, string: &String) {
        self.serial_datas.push(string.clone());
    }

    pub fn read_incoming_raw_data(&mut self) {
        let mut serial_data_raw: Vec<u8> = vec![];
        let mut serial_buf: Vec<u8> = vec![0; 32];
        match self.port.read(serial_buf.as_mut_slice()) {
            Err(e) => {
                error!("GSMHAT: Error reading incoming data: {}", e);
            }
            Ok(_) => {
                serial_buf.iter().for_each(|byte| {
                    if b'\n' == *byte {
                        let string = &String::from_utf8(serial_data_raw.clone()).unwrap();
                        serial_data_raw = vec![];
                        self.push_serial_data(string);
                        debug!("GSMHAT: Received data: {:?}", string);
                    } else {
                        serial_data_raw.push(*byte);
                    }
                });
            }
        }
    }
}
