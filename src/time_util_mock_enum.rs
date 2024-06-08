use chrono::{DateTime, Utc};

pub trait Clock {
    fn now() -> DateTime<Utc>;
}

pub enum ChronoClock {}
impl Clock for ChronoClock {
    fn now() -> DateTime<Utc> {
        Utc::now()
    }
}

pub fn seconds_since<T: Clock>(date: &str) -> u64 {
    let now = T::now();
    let utc_date: DateTime<Utc> = date.parse().unwrap();
    let duration = now.signed_duration_since(utc_date);
    duration.to_std().unwrap().as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    pub enum MockClock {}

    impl Clock for MockClock {
        fn now() -> DateTime<Utc> {
            let mock_now = "2024-06-07T00:00:03.000Z";
            DateTime::parse_from_rfc3339(mock_now)
                .unwrap()
                .with_timezone(&Utc)
        }
    }

    #[test]
    fn test_seconds_since() {
        let date = "2024-06-07T00:00:00.000Z";
        assert_eq!(seconds_since::<MockClock>(date), 3);
    }
}
