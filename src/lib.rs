extern crate chrono;

use chrono::{DateTime, Utc, Duration};

const ONE_BILLION: i64 = 1_000_000_000;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let one_billion_seconds = Duration::seconds(ONE_BILLION);
    start + one_billion_seconds
}
