use std::time::SystemTime;

use time::{format_description::well_known::Rfc2822, OffsetDateTime};

/// Format a date to RFC 2822 format.
pub fn format_date(date: SystemTime) -> String {
    let date = OffsetDateTime::from(date);
    date.format(&Rfc2822)
        .unwrap()
}
