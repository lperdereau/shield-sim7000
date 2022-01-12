mod at_commands;

use std::{thread, time::Duration};

use self::at_commands::{CMGF, CMGS, MAX_SEND_SMS_SIZE};
use crate::serial::SerialClient;
use at_commands_crate::builder::CommandBuilder;
use log::error;

pub struct SMSClient {
    client: SerialClient,
}

impl SMSClient {
    pub fn new(client: SerialClient) -> Self {
        Self { client }
    }

    pub fn enable(&mut self, bool: bool) {
        let mut buffer = [0; 128];
        let result = CommandBuilder::create_set(&mut buffer, true)
            .named(CMGF)
            .with_int_parameter(bool as i32)
            .finish()
            .unwrap();
        self.client.send(result);
    }

    pub fn send_sms(&mut self, message: &str, number: &str) -> bool {
        let mut buffer = [0; 1000];

        let first_end: usize = 12 + number.len();
        if message.len() > MAX_SEND_SMS_SIZE {
            error!(
                "Message is too long ({} chars), max is {}",
                message.len(),
                MAX_SEND_SMS_SIZE
            );
            return false;
        }

        CommandBuilder::create_set(&mut buffer[..first_end], true)
            .named(CMGS)
            .with_string_parameter(number)
            .finish()
            .unwrap();

        if !self.client.send(&buffer[..first_end]) {
            return false;
        }

        thread::sleep(Duration::from_millis(1000));
        let second_end: usize = first_end + MAX_SEND_SMS_SIZE + 1;
        CommandBuilder::create_execute(&mut buffer[first_end..second_end], false)
            .named(&message[..MAX_SEND_SMS_SIZE])
            .finish_with(b"\x1a")
            .unwrap();

        if !self.client.send(&buffer[first_end..second_end]) {
            return false;
        }
        true
    }
}
