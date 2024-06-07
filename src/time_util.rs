use chrono::{DateTime, Utc};

pub fn seconds_since(date: &str) -> u64 {
    let now = Utc::now();
    let utc_date: DateTime<Utc> = date.parse().unwrap();
    let duration = now.signed_duration_since(utc_date);
    duration.to_std().unwrap().as_secs()
}
