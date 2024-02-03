use chrono::{Datelike, NaiveDate};

const MONTH_RANGE: [(&str, i32); 12] = [
    ("January", 1), ("February", 2), ("March", 3), ("April", 4), ("May", 5), ("June", 6),
    ("July", 7), ("August", 8), ("September", 9), ("October", 10), ("November", 11), ("December", 12)
];

pub fn get_months() -> [(&'static str, i32); 12] {
    MONTH_RANGE
}

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