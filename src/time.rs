use chrono::prelude::*;

pub fn time_local(local: &DateTime<Local>) -> String {
    let time: NaiveTime = local.time();

    time.format("%H:%M:%S").to_string()
}

pub fn date_local(local: &DateTime<Local>) -> String {
    let date: NaiveDate = local.date_naive();

    date.format("%Y-%m-%d").to_string()
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
