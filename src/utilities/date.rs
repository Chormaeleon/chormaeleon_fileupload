use time::{macros::format_description, OffsetDateTime, PrimitiveDateTime};

pub fn format_datetime_human_readable(date: &PrimitiveDateTime) -> String {
    let format = format_description!("[day].[month].[year] [hour]:[minute] Uhr");
    date.format(&format).unwrap()
}

pub fn format_datetime_human_readable_seconds(date: &PrimitiveDateTime) -> String {
    let format = format_description!("[day].[month].[year] [hour]:[minute]:[second] Uhr");
    date.format(&format).unwrap()
}

pub fn now() -> PrimitiveDateTime {
    let now = OffsetDateTime::now_local().unwrap();
    PrimitiveDateTime::new(now.date(), now.time())
}
