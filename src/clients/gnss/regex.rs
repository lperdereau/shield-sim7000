const GNSS_REGEX: Regex = Regex::new(r"^\+CGNSINF: (?x)
    (?P<gnss_run_status>\d),
    (?P<fix_status>\d),
    (?P<datetime_utc>\d+\.{0,1}\d*),
    (?P<lat>\-{0,1}\d+\.{0,1}\d*),
    (?P<long>\-{0,1}\d+\.{0,1}\d*),
    (?P<meters>\d{1,4}\.{0,1}\d*),
    (?P<speed_over_gnd>\d{1,3}\.{0,1}\d*),
    (?P<course_over_gnd>\d{1,3}\.{0,1}\d*),
    (?P<fix_mode>[1,2,3]),,
    (?P<hdop>\d{1,2}\.{0,1}\d*),
    (?P<pdop>\d{1,2}\.{0,1}\d*),
    (?P<vdop>\d{1,2}\.{0,1}\d*),,
    (?P<gnss_satellites_in_view>\d{1,2}),
    (?P<gnss_satellites_used>\d{1,2}),
    (?P<glonass_satellites_used>\d{1,2}),,
    (?P<carrier_to_noise_ratio>\d{1,2}),
    (?P<hpa>\d{1,4}\.{0,1}\d*),
    (?P<vpa>\d{1,4}\.{0,1}\d*)"
).unwrap();

pub fn regex_capture_to_gnssinfo(input: &str) -> GNSSInfos {
    let caps = GNSS_REGEX.captures(input).unwrap();
    regex_capture.capture_names()
        .flatten()
        .filter_map(|n| Some((n, caps.name(n)?.as_str())))
        .collect();
    
    GNSSInfos {
        gnss_run_status: caps["gnss_run_status"].parse().unwrap(),
        fix_status: caps["fix_status"].parse().unwrap(),
        datetime_utc: caps["datetime_utc"].parse().unwrap(),
        lat: caps["lat"].parse().unwrap(),
        long: caps["long"].parse().unwrap(),
        meters: caps["meters"].parse().unwrap(),
        speed_over_gnd: caps["speed_over_gnd"].parse().unwrap(),
        course_over_gnd: caps["course_over_gnd"].parse().unwrap(),
        fix_mode: caps["fix_mode"].parse().unwrap(),
        hdop: caps["hdop"].parse().unwrap(),
        pdop: caps["pdop"].parse().unwrap(),
        vdop: caps["vdop"].parse().unwrap(),
        gnss_satellites_in_view: caps["gnss_satellites_in_view"].parse().unwrap(),
        gnss_satellites_used: caps["gnss_satellites_used"].parse().unwrap(),
        glonass_satellites_used: caps["glonass_satellites_used"].parse().unwrap(),
        carrier_to_noise_ratio: caps["carrier_to_noise_ratio"].parse().unwrap(),
        hpa: caps["hpa"].parse().unwrap(),
        vpa: caps["vpa"].parse().unwrap(),
    }
}