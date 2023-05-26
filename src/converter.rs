use chrono::{DateTime, Local};
use clap::ValueEnum;

pub mod epoch_millis;
pub mod rfc3339;
pub mod string;

#[derive(ValueEnum, Clone, Debug)]
pub enum TimeType {
    /// epoch millis
    E,
    /// RFC3339
    R,
    /// YYYY-MM-DD HH:mm:ss
    S,
}

impl TimeType {
    pub fn to_date_time(&self, time: String) -> DateTime<Local> {
        match self {
            TimeType::E => epoch_millis::from(time.parse::<i64>().unwrap()),
            TimeType::R => rfc3339::from(time),
            TimeType::S => string::from(time),
        }
    }

    pub fn to_formatted(&self, time: DateTime<Local>) -> Formatted {
        match self {
            TimeType::E => Formatted::EpochMillis(epoch_millis::format(time)),
            TimeType::R => Formatted::RFC3339(rfc3339::format(time)),
            TimeType::S => Formatted::String(string::format(time)),
        }
    }
}

pub fn handle(time: String, it: TimeType, ot: TimeType) -> String {
    let parsed_time = it.to_date_time(time);
    return ot.to_formatted(parsed_time).to_string();
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
