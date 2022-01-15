mod at_commands;
mod regex;

use at_commands_crate::builder::CommandBuilder;
use at_commands_crate::parser::CommandParser;
use chrono::offset::Utc;
use chrono::{DateTime, NaiveDateTime};
use chrono::{FixedOffset, TimeZone};
use serialport::ClearBuffer;
use std::time::Duration;
use std::{thread, time};
use std::io::BufReader;
use std::io::BufRead;

use self::at_commands::{CGNSINF, CGNSPWR};
use self::regex::GNSS_REGEX;
use crate::serial::SerialClient;
use log::debug;
use regex_crate::Regex;

#[derive(Debug)]
pub struct GNSSInfos {
    pub gnss_run_status: Option<i32>,
    pub fix_status: Option<i32>,
    pub datetime: Option<DateTime<Utc>>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude: Option<f64>,
    pub speed: Option<f32>,
    pub course: Option<f32>,
    pub fix: Option<u8>,
    pub hdop: Option<f32>,
    pub vdop: Option<f32>,
    pub pdop: Option<f32>,
    pub gps_satellites_in_view: Option<u8>,
    pub gps_satellites_used: Option<u8>,
    pub glonass_satellites_used: Option<u8>,
    pub carrier_to_noise_ratio: Option<f64>,
    pub hpa: Option<f32>,
    pub vpa: Option<f32>,
}

impl GNSSInfos {
    pub fn regex_capture_to_gnssinfo(input: &str) -> Self {
        let re = Regex::new(GNSS_REGEX).unwrap();
        let caps = re.captures(input).unwrap();
        re.capture_names()
            .flatten()
            .filter_map(|n| Some((n, caps.name(n)?.as_str())));
        let mut datetime: Option<DateTime<Utc>> = None;

        if let Ok(str) = caps["datetime_utc"].parse::<String>() {
            datetime = Some(Utc.from_utc_datetime(
                &NaiveDateTime::parse_from_str(str.as_str(), "%Y%m%d%H%M%S%.3f").unwrap(),
            ));
        }
        Self {
            gnss_run_status: caps["gnss_run_status"].parse::<i32>().ok(),
            fix_status: caps["fix_status"].parse::<i32>().ok(),
            datetime: datetime,
            latitude: caps["lat"].parse().ok(),
            longitude: caps["long"].parse().ok(),
            altitude: caps["altitude"].parse().ok(),
            speed: caps["speed_over_gnd"].parse().ok(),
            course: caps["course_over_gnd"].parse().ok(),
            fix: caps["fix_mode"].parse().ok(),
            hdop: caps["hdop"].parse().ok(),
            pdop: caps["pdop"].parse().ok(),
            vdop: caps["vdop"].parse().ok(),
            gps_satellites_in_view: caps["gnss_satellites_in_view"].parse().ok(),
            gps_satellites_used: caps["gnss_satellites_used"].parse().ok(),
            glonass_satellites_used: caps["glonass_satellites_used"].parse().ok(),
            carrier_to_noise_ratio: caps["carrier_to_noise_ratio"].parse().ok(),
            hpa: caps["hpa"].parse().ok(),
            vpa: caps["vpa"].parse().ok(),
        }
    }
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
        let mut buffer = [0; 128];
        let execute = CommandBuilder::create_execute(&mut buffer, true)
            .named(CGNSINF)
            .finish()
            .unwrap();

        thread::sleep(Duration::from_millis(1000));
        self.client.port.clear(ClearBuffer::Input).expect("Failed to discard input buffer");
        self.client.send(execute);
        
        let mut reader = BufReader::new(self.client.port.as_mut());
        let mut my_str = String::new();
        reader.read_line(&mut my_str).unwrap();
        println!("Command : {:?}", my_str);
        reader.read_line(&mut my_str).unwrap();
        println!("Command : {:?}", my_str);
        true
    }
}
