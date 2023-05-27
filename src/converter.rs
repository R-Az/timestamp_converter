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
    pub fn to_formatted(&self, time: DateTime<Local>) -> Formatted {
        match self {
            TimeFormat::E => Formatted::EpochMillis(epoch_millis::format(time)),
            TimeFormat::R => Formatted::RFC3339(rfc3339::format(time)),
            TimeFormat::S => Formatted::String(string::format(time)),
        }
    }
}
pub enum Formatted {
    EpochMillis(i64),
    RFC3339(String),
    String(String),
}

impl Formatted {
    pub fn to_string(&self) -> String {
        match self {
            Formatted::EpochMillis(time) => time.to_string(),
            Formatted::RFC3339(time) => time.to_string(),
            Formatted::String(time) => time.to_string(),
        }
    }
}

pub fn parse_time(time: String) -> DateTime<Local> {
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

pub fn handle(time: String, ot: TimeFormat) -> String {
    let parsed_time = parse_time(time);
    return ot.to_formatted(parsed_time).to_string();
}
