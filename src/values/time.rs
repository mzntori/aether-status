use chrono::prelude::*;

use crate::status::{Markup, StatusData};

/// Returns status data containing the local time in the format `HH:MM:SS`.
pub fn time_local(local: &DateTime<Local>) -> StatusData {
    let time: NaiveTime = local.time();

    StatusData {
        name: "time_local".to_string(),
        color: None,
        markup: Markup::Pango,
        full_text: time.format("%H:%M:%S").to_string(),
    }
}

/// Returns status data containing the local date in the format `YY-mm-DD`.
pub fn date_local(local: &DateTime<Local>) -> StatusData {
    let date: NaiveDate = local.date_naive();

    StatusData {
        name: "date_local".to_string(),
        color: None,
        markup: Markup::Pango,
        full_text: date.format("%Y-%m-%d").to_string(),
    }
}

/// Returns status data containing the local time and date in the format `YY-mm-DD HH:MM:SS`.
pub fn datetime_local(local: &DateTime<Local>) -> StatusData {
    let time: NaiveTime = local.time();
    let date: NaiveDate = local.date_naive();

    StatusData {
        name: "datetime_local".to_string(),
        color: None,
        markup: Markup::Pango,
        full_text: format!(
            "{} {}",
            date.format("%Y-%m-%d").to_string(),
            time.format("%H:%M:%S").to_string()
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datetime() {
        let time = Local::now();

        dbg!(time_local(&time));
        dbg!(date_local(&time));
    }
}
