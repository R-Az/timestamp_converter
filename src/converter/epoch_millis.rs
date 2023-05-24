use chrono::{DateTime, Local, TimeZone};

pub fn from(epoch_millis: i64) -> DateTime<Local> {
    return Local.timestamp_millis_opt(epoch_millis).unwrap();
}

pub fn format(time: DateTime<Local>) -> i64 {
    return time.timestamp_millis();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_unix_epoch() {
        // テストケース: epoch_millis = 0 (Unixエポック)
        let expected_dt = Local.timestamp_opt(0, 0).unwrap();
        assert_eq!(from(0), expected_dt);
    }

    #[test]
    fn test_from_specific_timestamp() {
        // テストケース: epoch_millis = 1684805123000 (2023年5月23日 午前10時25分23秒 Asia/Tokyo)
        let expected_dt = Local.with_ymd_and_hms(2023, 05, 23, 10, 25, 23).unwrap();
        assert_eq!(from(1684805123000), expected_dt);
    }

    #[test]
    fn test_format_unix_epoch() {
        // テストケース: epoch_millis = 0 (Unixエポック)
        let expected_dt = 0;
        assert_eq!(format(Local.timestamp_opt(0, 0).unwrap()), expected_dt);
    }

    #[test]
    fn test_format_specific_timestamp() {
        // テストケース = DateTime 2023年5月23日 午前10時25分23秒 Asia/Tokyo (1684805123000 )
        let expected_dt = 1684805123000;
        assert_eq!(
            format(Local.with_ymd_and_hms(2023, 05, 23, 10, 25, 23).unwrap()),
            expected_dt
        );
    }
}
