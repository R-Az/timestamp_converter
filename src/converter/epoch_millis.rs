use chrono::{DateTime, Local, LocalResult, TimeZone};

pub fn parse(epoch_millis: i64) -> Result<DateTime<Local>, String> {
    match Local.timestamp_millis_opt(epoch_millis) {
        LocalResult::Single(x) => Ok(x),
        LocalResult::None => Err("Err".to_owned()),
        LocalResult::Ambiguous(_, _) => Err("Err".to_owned()),
    }
}
pub fn format(time: DateTime<Local>) -> i64 {
    return time.timestamp_millis();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_unix_epoch() {
        let result = parse(0);
        let expected = Local.timestamp_opt(0, 0).unwrap();
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_parse() {
        let result = parse(1684805123000);
        let expected = Local.with_ymd_and_hms(2023, 05, 23, 10, 25, 23).unwrap();
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_parse_err() {
        let result = parse(1000000000000000000);
        assert!(result.is_err());
    }

    #[test]
    fn test_format_unix_epoch() {
        // テストケース: epoch_millis = 0 (Unixエポック)
        let expected_dt = 0;
        assert_eq!(format(Local.timestamp_opt(0, 0).unwrap()), expected_dt);
    }

    #[test]
    fn test_format() {
        // テストケース = DateTime 2023年5月23日 午前10時25分23秒 Asia/Tokyo (1684805123000 )
        let expected_dt = 1684805123000;
        assert_eq!(
            format(Local.with_ymd_and_hms(2023, 05, 23, 10, 25, 23).unwrap()),
            expected_dt
        );
    }
}
