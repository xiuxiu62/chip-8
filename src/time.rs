use std::time::{SystemTime, UNIX_EPOCH};

pub fn time_nanos() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}
