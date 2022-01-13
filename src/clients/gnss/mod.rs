mod at_commands;

use std::{thread, time::Duration};

use self::at_commands::{CGNSINF, CGNSPWR};
use crate::serial::SerialClient;
use at_commands_crate::builder::CommandBuilder;
use log::{debug, error};

pub struct GNSSInfos {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub speed: f64,
    pub course: f64,
    pub satellites: u8,
    pub hdop: f64,
    pub vdop: f64,
    pub pdop: f64,
    pub fix: u8,
    pub time: String,    
}

pub struct GNSSClient {
    client: SerialClient,
}

impl GNSSClient {
    pub fn new(client: SerialClient) -> Self {
        Self { client }
    }

    pub fn enable(&mut self, bool: bool) {
        let mut buffer = [0; 128];
        let result = CommandBuilder::create_set(&mut buffer, true)
            .named(CGNSPWR)
            .with_int_parameter(bool as i32)
            .finish()
            .unwrap();
        self.client.send(result);
    }

    pub fn get_gnss_info(&mut self, message: &str, number: &str) -> bool {
        let mut buffer = [0; 10];
        CommandBuilder::create_execute(&mut buffer, true)
            .named(CGNSINF)
            .finish()
            .unwrap();

        if !self.client.send(&buffer) {
            return false;
        }

        self.client.read_incoming_raw_data();
        true
    }
}
