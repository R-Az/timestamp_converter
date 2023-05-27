use chrono::{DateTime, Local};
use clap::ValueEnum;

pub mod epoch_millis;
pub mod rfc3339;
pub mod string;

#[derive(ValueEnum, Clone, Debug)]
pub enum TimeFormat {
    /// epoch millis
    E,
    /// RFC3339
    R,
    /// YYYY-MM-DD HH:mm:ss
    S,
}

impl TimeFormat {
    pub fn format(&self, time: DateTime<Local>) -> String {
        match self {
            TimeFormat::E => epoch_millis::format(time).to_string(),
            TimeFormat::R => rfc3339::format(time),
            TimeFormat::S => string::format(time),
        }
    }
}

pub fn handle(time: String, ot: TimeFormat) -> String {
    let parsed_time = parse_time(time);
    return ot.format(parsed_time);
}

fn parse_time(time: String) -> DateTime<Local> {
    let e = time.parse::<i64>();
    if e.is_ok() {
        let er = epoch_millis::from(e.unwrap());
        return er;
    }
    let r = rfc3339::from(time.clone());
    if r.is_ok() {
        return r.unwrap();
    }

    let s = string::from(time);

    return s;
}
