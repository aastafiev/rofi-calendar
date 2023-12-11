#[macro_use]
extern crate log;

pub mod calendar;
pub mod rofi;

use calendar::FirstWeekDay;
// use calendar::{Calendar, FirstWeekDay};
// use chrono::{Local, Months};
use chrono::Local;
// use rofi::BoxResult;
use rofi::{rofi_calendar, BoxResult};
use std::{env, io::Write};

fn main() -> BoxResult<()> {
    env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} {} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();

    let show_weeks: bool = env::var("WEEK")
        .unwrap_or("true".into())
        .parse()
        .unwrap_or(true);
    let is_monday: bool = env::var("MONDAY")
        .unwrap_or("true".into())
        .parse()
        .unwrap_or(true);
    let first_week_day = match is_monday {
        true => FirstWeekDay::Mon,
        false => FirstWeekDay::Sun,
    };

    let today = Local::now().naive_local().date();
    // let today = Local::now()
    //     .naive_local()
    //     .date()
    //     .checked_add_months(Months::new(1))
    //     .unwrap();
    // let cal = Calendar::from_ym(today, &first_week_day, &show_weeks);
    // println!("{}", cal);
    rofi_calendar(today, &first_week_day, &show_weeks, true)?;
    Ok(())
}
