use cyrano_core::cv_data::{DateRange, YearMonth};

pub fn optional(value: String) -> Option<String> {
    if value.is_empty() { None } else { Some(value) }
}

pub fn with_item<T>(items: &mut [T], index: usize, change: impl FnOnce(&mut T)) {
    if let Some(item) = items.get_mut(index) {
        change(item);
    }
}

pub fn move_item<T>(items: &mut [T], index: usize, delta: i32) {
    let target = index as i32 + delta;

    if target >= 0 && (target as usize) < items.len() {
        items.swap(index, target as usize);
    }
}

pub fn new_range() -> DateRange {
    DateRange {
        start: YearMonth {
            year: 0,
            month: None,
        },
        end: None,
    }
}
