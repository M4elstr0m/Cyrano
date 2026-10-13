use crate::components::TextInput;
use cyrano_core::cv_data::{DateRange, YearMonth};
use dioxus::prelude::*;

fn year_text(year: Option<u16>) -> String {
    year.filter(|year| *year != 0)
        .map(|year| year.to_string())
        .unwrap_or_default()
}

fn month_text(month: Option<u8>) -> String {
    month.map(|month| month.to_string()).unwrap_or_default()
}

fn parse_year(text: &str) -> Option<u16> {
    text.trim().parse().ok()
}

fn parse_month(text: &str) -> Option<u8> {
    text.trim()
        .parse()
        .ok()
        .filter(|month| (1..=12).contains(month))
}

#[component]
pub fn DateRangeFields(
    start_year: u16,
    start_month: Option<u8>,
    end_year: Option<u16>,
    end_month: Option<u8>,
    onchange: EventHandler<DateRange>,
) -> Element {
    let emit = move |start: (u16, Option<u8>), end: (Option<u16>, Option<u8>)| {
        onchange.call(DateRange {
            start: YearMonth {
                year: start.0,
                month: start.1,
            },
            end: end.0.map(|year| YearMonth { year, month: end.1 }),
        });
    };

    rsx! {
        div { class: "grid grid-cols-2 gap-3",
            TextInput {
                label: "Start year",
                value: year_text(Some(start_year)),
                oninput: move |text: String| {
                    emit((parse_year(&text).unwrap_or_default(), start_month), (end_year, end_month))
                },
            }
            TextInput {
                label: "Start month",
                value: month_text(start_month),
                oninput: move |text: String| {
                    emit((start_year, parse_month(&text)), (end_year, end_month))
                },
            }
            TextInput {
                label: "End year (empty if ongoing)",
                value: year_text(end_year),
                oninput: move |text: String| {
                    emit((start_year, start_month), (parse_year(&text), end_month))
                },
            }
            TextInput {
                label: "End month",
                value: month_text(end_month),
                oninput: move |text: String| {
                    emit((start_year, start_month), (end_year, parse_month(&text)))
                },
            }
        }
    }
}
