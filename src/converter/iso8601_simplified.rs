// ISO8601からtimezoneを省略したフォーマット
use chrono::{DateTime, Local};

pub fn from(time: String) -> DateTime<Local> {
    let tz = Local::now().format("%z").to_string();

    return DateTime::parse_from_str(&format!("{time} {tz}"), "%Y-%m-%d %H:%M:%S %z")
        .unwrap()
        .with_timezone(&Local);
}

pub fn format(time: DateTime<Local>) -> String {
    return time.format("%Y-%m-%d %H:%M:%S").to_string();
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn test_from_zero() {
        let tz = Local::now().format("%z").to_string();
        print!("{}", tz);
        // let result = from("1970-01-01 09:00:00".to_owned());
        // let expected = Local.timestamp_opt(0, 0).unwrap();
        // assert_eq!(result, expected);
    }

    #[test]
    fn test_from_string() {
        let result = from("2023-05-18 00:00:00".to_owned());
        let expected = Local.with_ymd_and_hms(2023, 05, 18, 00, 00, 00).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_format_zero() {
        let result = format(Local.timestamp_opt(0, 0).unwrap());
        let expected = "1970-01-01 09:00:00";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_format_string() {
        let result = format(Local.with_ymd_and_hms(2023, 05, 18, 00, 00, 00).unwrap());
        let expected = "2023-05-18 00:00:00";
        assert_eq!(result, expected);
    }
}
