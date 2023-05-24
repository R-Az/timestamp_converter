use std::env;

use chrono::{DateTime, Local};
use clap::{Parser, ValueEnum};
mod converter;

#[derive(Parser, Debug)]
#[clap(
    name = env!("CARGO_BIN_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The time
    #[arg(value_name = "time", help = "format timestamp")]
    time: String,

    /// time type from
    #[arg(short, long)]
    #[clap(value_enum, default_value_t=TimeType::S)]
    output_type: TimeType,

    /// time type to
    #[arg(short, long)]
    #[clap(value_enum, default_value_t=TimeType::E)]
    input_type: TimeType,
}

#[derive(ValueEnum, Clone, Debug)]
enum TimeType {
    /// epoch millis
    E,
    /// RFC3339
    R,
    /// YYYY-MM-DD HH:mm:ss
    S,
}

impl TimeType {
    fn to_date_time(&self, time: String) -> DateTime<Local> {
        match self {
            TimeType::E => converter::epoch_millis::from(time.parse::<i64>().unwrap()),
            TimeType::R => converter::rfc3339::from(time),
            TimeType::S => converter::string::from(time),
        }
    }

    fn to_formatted(&self, time: DateTime<Local>) -> Formatted {
        match self {
            TimeType::E => Formatted::EpochMillis(converter::epoch_millis::format(time)),
            TimeType::R => Formatted::RFC3339(converter::rfc3339::format(time)),
            TimeType::S => Formatted::String(converter::string::format(time)),
        }
    }
}

enum Formatted {
    EpochMillis(i64),
    RFC3339(String),
    String(String),
}

impl Formatted {
    fn to_string(&self) -> String {
        match self {
            Formatted::EpochMillis(time) => time.to_string(),
            Formatted::RFC3339(time) => time.to_string(),
            Formatted::String(time) => time.to_string(),
        }
    }
}

fn main() {
    let args = Args::parse();

    let origin_time = args.time.clone();
    let parsed_time = args.input_type.to_date_time(origin_time);

    let formatted = args.output_type.to_formatted(parsed_time);

    println!("{}", env!("CARGO_BIN_NAME"));
    println!("{} -> {}", args.time, formatted.to_string());
}
