use log::{debug, error, info};
use serialport::ClearBuffer;
use serialport::SerialPort;
use std::io;
use std::io::Write;
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

    pub fn read(&mut self) -> Vec<String> {
        let mut serial_buf: Vec<u8> = vec![0; 1000];
        match self.port.read(&mut serial_buf.as_mut_slice()) {
            Ok(n) => {
                return Self::cutting_bytes_if_crlf(&mut serial_buf)
                    .iter()
                    .map(|&s| std::str::from_utf8(s).unwrap().to_string())
                    .collect::<Vec<String>>();
            }
            Err(e) => {
                error!("SHIELD: Failed to read line: {}", e);
            }
        }
        return vec![];
    }

    fn cutting_bytes_if_crlf(bytes: &[u8]) -> Vec<&[u8]> {
        let mut result: Vec<&[u8]> = vec![];
        let mut cmp: usize = 0;
        for i in cmp..bytes.len() {
            if i > 0 && bytes[i] == b'\n' && bytes[i - 1] == b'\r' {
                result.push(&bytes[cmp..i]);
            }
            cmp = cmp + i;
        }
        result
    }

    pub fn read_lines(&mut self, line_number: usize) -> Vec<String> {
        let mut vec_string = vec![];
        while vec_string.len() < line_number {
            self.read().iter().for_each(|v| vec_string.push(v.clone()));
        }
        vec_string
    }

    pub fn clear(&self, buffer: ClearBuffer) {
        self.port
            .clear(buffer)
            .expect("Failed to discard input buffer");
    }
}
