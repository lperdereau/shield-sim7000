use log::{debug, error, info};
use serialport::ClearBuffer;
use serialport::SerialPort;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::time::Duration;
use std::vec;

pub struct SerialClient {
    port: Box<dyn serialport::SerialPort>,
    pub serial_datas: Vec<String>,
}

impl SerialClient {
    pub fn new(baudrate: u32, port_path: &str) -> Self {
        let port = Self::connect(baudrate, port_path.to_string());
        Self {
            port,
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
        let result = self.port.write(bytes);
        match result {
            Ok(_) => {
                debug!("SHIELD: Sent {}", string);
                true
            }
            Err(e) => {
                error!("SHIELD: Failed to send {}: {}", string, e);
                false
            }
        }
    }

    pub fn read_line(&mut self) -> String {
        let mut reader = BufReader::new(self.port.try_clone().unwrap());
        let mut string = String::new();
        loop {
            match self.port.bytes_to_read() {
                Ok(0) => {
                    break;
                }
                Ok(_) => {
                    match reader.read_line(&mut string) {
                        Ok(_) => {
                            if string.len() > 0 {
                                info!("SHIELD: Received {}", string);
                                break;
                            }
                        }
                        Err(e) => {
                            debug!("SHIELD: Failed to read line: {}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("SHIELD: Failed to read: {}", e);
                }
            }
        }
        return string;
    }

    pub fn read_lines(&mut self, line_number: usize) -> Vec<String> {
        let mut vec_string = vec![];
        for _ in 0..line_number - 1 {
            vec_string.push(self.read_line());
        }
        vec_string
    }

    pub fn clear(&self, buffer: ClearBuffer) {
        self.port
            .clear(buffer)
            .expect("Failed to discard input buffer");
    }
}
