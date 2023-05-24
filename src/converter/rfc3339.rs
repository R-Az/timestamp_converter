use chrono::{DateTime, Local};

pub fn from(time: String) -> DateTime<Local> {
    return DateTime::parse_from_rfc3339(&time)
        .unwrap()
        .with_timezone(&Local);
}

pub fn format(time: DateTime<Local>) -> String {
    return time.to_rfc3339();
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn test_from_zero() {
        let result = from("1970-01-01T09:00:00+09:00".to_owned());
        let expected = Local.timestamp_opt(0, 0).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_rfc3339() {
        let result = from("2023-05-18T00:00:00+09:00".to_owned());
        let expected = Local.with_ymd_and_hms(2023, 05, 18, 00, 00, 00).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_format_zero() {
        let result = format(Local.timestamp_opt(0, 0).unwrap());
        let expected = "1970-01-01T09:00:00+09:00";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_format_rf3339() {
        let result = format(Local.with_ymd_and_hms(2023, 05, 18, 00, 00, 00).unwrap());
        let expected = "2023-05-18T00:00:00+09:00";
        assert_eq!(result, expected);
    }
}