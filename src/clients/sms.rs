use std::{thread, time};
use crate::serial::SerialClient;

pub struct SMSClient {
    serial_hat: SerialClient,
}

impl SMSClient {
    pub fn new (serial_hat: SerialClient) -> Self {
        Self {
            serial_hat,
        }
    }

    pub fn send_sms(&mut self, message: &str, number: &str) -> bool {
        self.serial_hat.send_to_hat("AT+CMGF=1".to_string());
        self.serial_hat.send_to_hat(format!("AT+CMGS=\"{}\"", number.to_string()));
        self.serial_hat.send_to_hat(format!("{}\x1A", message.to_string()));
        thread::sleep(time::Duration::from_millis(10));
        self.serial_hat.read_incoming_raw_data();
        println!("{:?}", self.serial_hat.serial_datas);
        return self.serial_hat.serial_datas == ["\r", "OK\r", ">\r", "+CMGS: 1\r", "\r", "OK\r"];
    }
}

