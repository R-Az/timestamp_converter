use crate::Args;
use chrono::{DateTime, Local};
use clap::ValueEnum;
use core::panic;

pub mod epoch_millis;
pub mod iso8601_simplified;
pub mod rfc3339;

#[derive(ValueEnum, Clone, Debug, PartialEq)]
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

pub fn handle(args: Args) -> String {
    let convert_args = convert_args(args);
    let parsed_time = parse_time(convert_args.time.clone());
    let formatted = convert_args.format.handle(parsed_time);
    return generate_result(convert_args, formatted);
}

fn parse_time(time: String) -> DateTime<Local> {
    let e = time.parse::<i64>();
    if e.is_ok() {
        let er = epoch_millis::parse(e.unwrap());
        if er.is_ok() {
            return er.unwrap();
        }
    }
    let r = rfc3339::parse(time.clone());
    if r.is_ok() {
        return r.unwrap();
    }

    let s = iso8601_simplified::parse(time.clone());
    if s.is_ok() {
        return s.unwrap();
    }

    panic!("parse error. illegal format. {}", time)
}
#[cfg(test)]
mod tests_parse_time {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_parse_time_rfc3339() {
        let result = parse_time("2023-05-18T00:00:00+09:00".to_owned());
        let expected = Local.with_ymd_and_hms(2023, 05, 18, 00, 00, 00).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_time_iso8601_simplified() {
        let result = parse_time("2023-05-18 00:00:00".to_owned());
        let expected = Local.with_ymd_and_hms(2023, 05, 18, 00, 00, 00).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_time_epoch_millis() {
        let result = parse_time("1684335600000".to_owned());
        let expected = Local.with_ymd_and_hms(2023, 05, 18, 00, 00, 00).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn test_parse_time_err() {
        parse_time("xxxxxxx".to_owned());
    }
}

#[derive(Debug, PartialEq, Clone)]
struct ConvertArgs {
    time: String,
    label: String,
    format: TimeFormat,
    only_formatted: bool,
}

fn convert_args(args: Args) -> ConvertArgs {
    let is_time_empty = args.time.is_none();

    let time = if is_time_empty {
        iso8601_simplified::format(Local::now())
    } else {
        args.time.unwrap()
    };

    let label: &str = if is_time_empty { " now" } else { "" };

    return ConvertArgs {
        time: time,
        label: label.to_string(),
        format: args.format,
        only_formatted: args.only_formatted,
    };
}

#[cfg(test)]
mod tests_convert_args {
    use super::*;

    #[test]
    fn test_convert_args_now() {
        let result = convert_args(Args {
            time: None,
            format: TimeFormat::S,
            only_formatted: false,
        });
        let expected = ConvertArgs {
            time: iso8601_simplified::format(Local::now()),
            label: " now".to_string(),
            format: TimeFormat::S,
            only_formatted: false,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_convert_args_time() {
        let result = convert_args(Args {
            time: Some("2023-05-18 00:00:00".to_string()),
            format: TimeFormat::E,
            only_formatted: false,
        });
        let expected = ConvertArgs {
            time: "2023-05-18 00:00:00".to_string(),
            label: "".to_string(),
            format: TimeFormat::E,
            only_formatted: false,
        };
        assert_eq!(result, expected);
    }
}

fn generate_result(args: ConvertArgs, formatted: String) -> String {
    if args.only_formatted {
        return formatted;
    }
    return format!("{}{} -> {}", args.time, args.label, formatted);
}

#[cfg(test)]
mod tests_generate_result {
    use super::*;

    #[test]
    fn test_normal() {
        let result = generate_result(
            ConvertArgs {
                time: "2023-05-18 00:00:00".to_string(),
                label: " now".to_string(),
                format: TimeFormat::E,
                only_formatted: false,
            },
            "1684335600000".to_string(),
        );
        let expected = "2023-05-18 00:00:00 now -> 1684335600000".to_string();
        assert_eq!(result, expected);
    }
    #[test]
    fn test_only_formatted() {
        let result = generate_result(
            ConvertArgs {
                time: "2023-05-18 00:00:00".to_string(),
                label: " now".to_string(),
                format: TimeFormat::E,
                only_formatted: true,
            },
            "1684335600000".to_string(),
        );
        let expected = "1684335600000".to_string();
        assert_eq!(result, expected);
    }
}
