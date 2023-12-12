#[macro_use]
extern crate log;

pub mod calendar;
pub mod rofi;

use calendar::FirstWeekDay;
use chrono::Local;
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
    let block_button: u8 = env::var("BLOCK_BUTTON").unwrap_or("0".into()).parse().unwrap();
    let today = Local::now().naive_local();

    match block_button {
        1 | 2 | 3 => {
            let show_weeks: bool = env::var("SHOW_WEEK").unwrap_or("true".into()).parse().unwrap_or(true);
            let is_monday: bool = env::var("MONDAY").unwrap_or("true".into()).parse().unwrap_or(true);
            let first_week_day = match is_monday {
                true => FirstWeekDay::Mon,
                false => FirstWeekDay::Sun,
            };

            rofi_calendar(today, &first_week_day, &show_weeks, true)?;
        }
        _ => {}
    }

    let label = env::var("LABEL").unwrap_or(" ".into());
    let datefmt = env::var("DATEFMT").unwrap_or("%a, %e %b %Y  %H:%M".into());
    let shortfmt = env::var("SHORTFMT").unwrap_or("%a, %e %b %Y".into());
    debug!("Date fmt: {}", datefmt);
    debug!("Short date fmt: {}", shortfmt);
    println!("{}{}", label, today.format(&datefmt));
    println!("{}{}", label, today.format(&shortfmt));
    Ok(())
}
