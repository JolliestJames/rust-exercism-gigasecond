extern crate chrono;

use chrono::{DateTime, Utc, Duration};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let one_billion = 10_i64.pow(9);
    let one_billion_seconds = Duration::seconds(one_billion);
    let one_billion_seconds_after_start: DateTime<Utc> = start + one_billion_seconds;

    return one_billion_seconds_after_start;
}
