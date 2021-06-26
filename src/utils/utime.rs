use chrono::prelude::*;

pub fn time_until_monday_in_millis() -> i64 {

    let local: DateTime<Local> = Local::now();
    let mon: NaiveDateTime = NaiveDate::from_isoywd(local.year(), local.iso_week().week() + 1, Weekday::Mon).and_hms(0, 0, 0);
    let mon: i64 = mon.timestamp_millis();
    
    mon - local.timestamp_millis()

}

pub fn time_until_midnight_in_millis() -> i64 {

    let local: DateTime<Local> = Local::now();
    let mon: NaiveDateTime = NaiveDate::from_isoywd(local.year(), local.iso_week().week(), local.weekday()).and_hms(23, 59, 59);
    let mon: i64 = mon.timestamp_millis();
    
    mon - local.timestamp_millis()

}