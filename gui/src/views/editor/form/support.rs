use std::time::{SystemTime, UNIX_EPOCH};

const SECONDS_PER_YEAR: u64 = 31_556_952;

pub fn optional(value: String) -> Option<String> {
    if value.is_empty() { None } else { Some(value) }
}

pub fn with_item<T>(items: &mut [T], index: usize, change: impl FnOnce(&mut T)) {
    if let Some(item) = items.get_mut(index) {
        change(item);
    }
}

pub fn current_year() -> u16 {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs())
        .unwrap_or_default();

    (1970 + seconds / SECONDS_PER_YEAR) as u16
}
