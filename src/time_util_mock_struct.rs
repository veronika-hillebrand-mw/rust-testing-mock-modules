use chrono::{DateTime, Utc};

pub trait Clock {
    fn now(&self) -> DateTime<Utc>;
}

pub struct ChronoClock;
impl Clock for ChronoClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

pub fn seconds_since(date: &str, clock: impl Clock) -> u64 {
    let now = clock.now();
    let utc_date: DateTime<Utc> = date.parse().unwrap();
    let duration = now.signed_duration_since(utc_date);
    duration.to_std().unwrap().as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct MockClock {
        pub timestamp: DateTime<Utc>,
    }

    impl Clock for MockClock {
        fn now(&self) -> DateTime<Utc> {
            self.timestamp
        }
    }

    #[test]
    fn test_seconds_since() {
        let mock_now = "2024-06-07T00:00:03.000Z";
        let date = "2024-06-07T00:00:00.000Z";
        assert_eq!(
            seconds_since(
                date,
                MockClock {
                    timestamp: DateTime::parse_from_rfc3339(mock_now)
                        .unwrap()
                        .with_timezone(&Utc)
                }
            ),
            3
        );
    }
}
