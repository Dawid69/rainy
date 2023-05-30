use chrono::prelude::{DateTime, Local};

//Time functions

/// Return current time
pub fn current() -> DateTime<Local> {
    let local: DateTime<Local> = Local::now();
    local
}

/// return unix timestamp
pub fn unix_ts() -> i64 {
    current().timestamp()
}

/// return unix timestamp nanos prescision
pub fn unix_ts_ns() -> i64 {
    current().timestamp_nanos()
}
