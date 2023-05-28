// ISO8601からtimezoneを省略したフォーマット
use chrono::{DateTime, Local, ParseError};

pub fn parse(time: String) -> Result<DateTime<Local>, ParseError> {
    let tz = Local::now().format("%z").to_string();

    let p = DateTime::parse_from_str(&format!("{time} {tz}"), "%Y-%m-%d %H:%M:%S %z")?;
    return Ok(p.with_timezone(&Local));
}

#[cfg(test)]
mod tests_parse {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_zero() {
        let result = parse("1970-01-01 09:00:00".to_owned());
        let expected = Local.timestamp_opt(0, 0).unwrap();
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_normal() {
        let result = parse("2023-05-18 00:00:00".to_owned());
        let expected = Local.with_ymd_and_hms(2023, 05, 18, 00, 00, 00).unwrap();
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_err() {
        let result = parse("xxxxx".to_owned());
        assert!(result.is_err());
    }
}

pub fn format(time: DateTime<Local>) -> String {
    return time.format("%Y-%m-%d %H:%M:%S").to_string();
}

#[cfg(test)]
mod tests_format {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_zero() {
        let result = format(Local.timestamp_opt(0, 0).unwrap());
        let expected = "1970-01-01 09:00:00";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_normal() {
        let result = format(Local.with_ymd_and_hms(2023, 05, 18, 00, 00, 00).unwrap());
        let expected = "2023-05-18 00:00:00";
        assert_eq!(result, expected);
    }
}
