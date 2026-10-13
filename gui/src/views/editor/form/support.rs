use cyrano_core::cv_data::{DateRange, YearMonth};

pub fn optional(value: String) -> Option<String> {
    if value.is_empty() { None } else { Some(value) }
}

pub fn with_item<T>(items: &mut [T], index: usize, change: impl FnOnce(&mut T)) {
    if let Some(item) = items.get_mut(index) {
        change(item);
    }
}

pub fn move_to<T>(items: &mut Vec<T>, from: usize, to: usize) {
    if from < items.len() && to < items.len() {
        let item = items.remove(from);
        items.insert(to, item);
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
