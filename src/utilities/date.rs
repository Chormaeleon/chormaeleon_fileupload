use chrono::NaiveDateTime;

pub fn format_datetime_human_readable(date: &NaiveDateTime) -> String {
    date.format("%d.%m.%Y, %H:%M Uhr").to_string()
}

pub fn format_datetime_human_readable_seconds(date: &NaiveDateTime) -> String {
    date.format("%d.%m.%Y, %H:%M:%S Uhr").to_string()
}
