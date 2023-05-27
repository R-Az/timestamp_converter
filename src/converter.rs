use chrono::{DateTime, Local};
use clap::ValueEnum;

pub mod epoch_millis;
pub mod iso8601_simplified;
pub mod rfc3339;

#[derive(ValueEnum, Clone, Debug)]
pub enum TimeFormat {
    /// epoch millis 1684805123000
    E,
    /// RFC3339 2023-05-18T00:00:00+09:00
    R,
    /// ISO8601 simplified YYYY-MM-DD hh:mm:ss (ISO8601からtimezoneを省略したフォーマット)
    S,
}

impl TimeFormat {
    pub fn handle(&self, time: DateTime<Local>) -> String {
        match self {
            TimeFormat::E => epoch_millis::format(time).to_string(),
            TimeFormat::R => rfc3339::format(time),
            TimeFormat::S => iso8601_simplified::format(time),
        }
    }
}

pub fn handle(time: String, format: TimeFormat) -> String {
    let parsed_time = parse_time(time);
    return format.handle(parsed_time);
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

    let s = iso8601_simplified::from(time);

    return s;
}
