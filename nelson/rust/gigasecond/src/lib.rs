use chrono::{DateTime, Duration, Utc};
use std::ops::Add;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    const GIGASECOND: i64 = 1000000000;
    let delta = Duration::seconds(GIGASECOND);
    start.add(delta)
}
