mod at_commands;
pub mod regex;

use std::{thread, time::Duration};

use self::at_commands::{CGNSINF, CGNSPWR};
use crate::serial::SerialClient;
use at_commands_crate::builder::CommandBuilder;
use log::{debug, error};


pub struct GNSSInfos {
    pub gnss_run_status: bool,
    pub fix_status: bool,
    pub datetime: String,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub speed: f32,
    pub course: f32,
    pub fix: u8,
    pub hdop: f32,
    pub vdop: f32,
    pub pdop: f32,
    pub gps_satellites_in_view: u8,
    pub gps_satellites_used: u8,
    pub glonass_satellites_used: u8,
    pub carrier_to_noise_ratio: f64,
    pub hpa: f32,
    pub vpa: f32,
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
        let set = CommandBuilder::create_set(&mut buffer, true)
            .named(CGNSPWR)
            .with_int_parameter(bool as i32)
            .finish()
            .unwrap();

        self.client.send(set);
    }

    pub fn is_gnss_fix(&mut self) -> bool {
        let mut buffer = [0; 10];
        let execute =CommandBuilder::create_execute(&mut buffer, true)
            .named(CGNSINF)
            .finish()
            .unwrap();

        self.client.send(&execute);
        time::Duration::from_millis(1000);
        
        let gnss_infos: GNSSInfo = self.client.read_incoming_raw_data();
        

        if fix_status == 1 { true } else { false };

        //check second digit of the response to know the fix status
        // if 1 GNSS is ready
        // else retry periodically


        true
    }

    pub fn get_gnss_info(&mut self) -> Vec<str> {
        let coordinates: Vec<str> = [];
        let mut buffer = [0; 10];
        let execute =CommandBuilder::create_execute(&mut buffer, true)
            .named(CGNSINF)
            .finish()
            .unwrap();

        self.client.send(&execute);

        self.client.read_incoming_raw_data();

        true
    }
}
