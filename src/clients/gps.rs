use std::f64::consts::PI;

const EARTH_RADIUS: i32 = 6378137;

struct GPS {
    pub gnss_status: u8,
    pub gnss_fix: u8,
    pub utc_date: String,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub speed: f32,
    pub course: f32,
    pub hdop: f32,
    pub vdop: f32,
    pub pdop: f32,
    pub gps_satalites: u8,
    pub gps_satelites: u8,
    pub signal: f32,
}

impl GPS {
    pub fn calculateDeltaP(position1: GPS, position2: GPS) {
        phi1 = position1.latitude * PI / 180.0;
        phi2 = position2.latitude * PI / 180.0;
        delta_phi = (position2.latitude - position1.latitude) * PI / 180.0;
        delta_lambda = (position2.longitude - position1.longitude) * PI / 180.0;
        a = (sin(delta_phi / 2.0) * sin(delta_phi / 2.0)) + (cos(phi1) * cos(phi2) * sin(delta_lambda / 2.0) * sin(delta_lambda / 2.0));
        c = 2.0 * atan2(sqrt(a), sqrt(1.0 - a));
        EARTH_RADIUS * c
    }
}