use chrono::{DateTime, Utc};
use mockall::automock;

#[automock]
pub trait Clock {
    fn now(&self) -> DateTime<Utc>;
}

pub struct ChronoClock;
impl Clock for ChronoClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

pub fn seconds_since(date: &str, clock: &dyn Clock) -> u64 {
    let now = clock.now();
    let utc_date: DateTime<Utc> = date.parse().unwrap();
    let duration = now.signed_duration_since(utc_date);
    duration.to_std().unwrap().as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seconds_since() {
        let date = "2024-06-07T00:00:00.000Z";
        let mock_now = "2024-06-07T00:00:03.000Z";
        let mut mock = MockClock::new();
        mock.expect_now().returning(|| {
            DateTime::parse_from_rfc3339(mock_now)
                .unwrap()
                .with_timezone(&Utc)
        });
        assert_eq!(3, seconds_since(date, &mock));
    }
}
