mod at_commands;

use std::{thread, time::Duration};

use self::at_commands::{CMGF, CMGS};
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
        if message.len() > 998 {
            error!("Message is too long ({} chars), max is 998", message.len());
            return false;
        }
        let mut buffer = [0; 1000];
        CommandBuilder::create_set(&mut buffer, true)
            .named(CMGS)
            .with_string_parameter(number);
        if !self.client.send(&buffer) {
            return false;
        }

        CommandBuilder::create_execute(&mut buffer, false)
            .named(message)
            .finish()
            .unwrap();

        if !self.client.send(&buffer) {
            return false;
        }
        thread::sleep(Duration::from_millis(4000));
        self.client.read_incoming_raw_data();
        true
    }
}
