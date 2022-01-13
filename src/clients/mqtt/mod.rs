mod at_commands;

use std::{thread, time::Duration};

use self::at_commands::{CNACT, SMCONF, SMCONN, SMPUB};
use crate::serial::SerialClient;
use at_commands_crate::builder::CommandBuilder;
use log::{debug, error};

pub struct MQTTClient {
    client: SerialClient,
}

impl GNSSClient {
    pub fn new(client: SerialClient) -> Self {
        Self { client }
    }

    pub fn set_apn_settings(&mut self, apn: &str) {
        let mut buffer = [0; 128];
        let result = CommandBuilder::create_set(&mut buffer, true)
            .named(CNACT)
            .with_string_parameter(&apn)
            .finish()
            .unwrap();
        self.client.send(result);
    }

    pub fn set_mqtt_server_settings(&mut self, setting_key: &str, setting_value: &str) {
        let mut buffer = [0; 128];
        let result = CommandBuilder::create_set(&mut buffer, true)
            .named(SMCONF)
            .with_string_parameter(&setting_key)
            .with_string_parameter(&setting_value)
            .finish()
            .unwrap();
        self.client.send(result);
    }

    pub fn check_mqtt_server_connection(&mut self) {
        let mut buffer = [0; 128];
        let result = CommandBuilder::create_execute(&mut buffer, true)
            .named(SMCONN)
            .finish()
            .unwrap();
        self.client.send(result);
    }
    
    pub fn send_message_to_mqtt_server(&mut self, topic: &str, message: &str, qos: u8, retain: bool) {
        let mut first_buffer = [0; 128];
        let mut second_buffer = [0; 512];
        
        CommandBuilder::create_set(&mut first_buffer, true)
            .named(SMPUB)
            .with_string_parameter(&topic)
            .with_string_parameter(&message.len())
            .with_int_parameter(qos as i32)
            .with_int_parameter(retain as i32)
            .finish()
            .unwrap();

        if !self.client.send(&first_buffer) {
            return false;
        }

        thread::sleep(Duration::from_millis(5000));
        CommandBuilder::create_execute(&mut second_buffer, true)
            .named(&message)
            .finish_with(b"\x1a")
            .unwrap();
    
        if !self.client.send(&second_buffer) {
            return false;
        }
    }
}
