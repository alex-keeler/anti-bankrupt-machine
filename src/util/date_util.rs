use chrono::{Datelike, NaiveDate};

pub fn get_date_range(year: i32, month: u32) -> Result<(NaiveDate, NaiveDate), String> {

    let (end_year, end_month) = match month {
        12 => (year + 1, 1),
        _ => (year, month + 1),
    };

    let begin_date = NaiveDate::from_ymd_opt(year, month, 1).ok_or("Cannot parse begin date")?;
    let end_date = NaiveDate::from_ymd_opt(end_year, end_month, 1).ok_or("Cannot parse end date")?;

    Ok((begin_date, end_date))
}

pub fn months_between(d1: NaiveDate, d2: NaiveDate) -> i32 {
    (d2.year() - d1.year()) * 12 + (d2.month() as i32 - d1.month() as i32)
}