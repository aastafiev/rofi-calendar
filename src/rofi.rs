use crate::calendar::{Calendar, FirstWeekDay};
use chrono::{Local, Months, NaiveDate};
use std::{env, error, io, io::prelude::*, process};

pub type BoxResult<T> = Result<T, Box<dyn error::Error>>;

fn run_rofi(cal: Calendar) -> io::Result<process::Output> {
    let y_offset: i32 = env::var("Y_OFFSET")
        .unwrap_or("-20".into())
        .parse()
        .unwrap();
    let window_width = match cal.show_weeks {
        true => 224,
        false => 192,
    };
    let window_theme = format!(
        "window {{width: {0}px; location: southeast; y-offset: {1:+}px;}}",
        window_width, y_offset
    );
    let mut child = process::Command::new("rofi")
        .args(&[
            "-kb-move-char-back",
            "",
            "-kb-custom-1",
            "Left",
            "-kb-move-char-forward",
            "",
            "-kb-custom-2",
            "Right",
            "-kb-move-word-back",
            "",
            "-kb-custom-3",
            "Control+Left",
            "-kb-move-word-forward",
            "",
            "-kb-custom-4",
            "Control+Right",
            "-kb-row-first",
            "",
            "-kb-custom-5",
            "Home",
            "-dmenu",
            "-p",
            &cal.show_ym(),
            "-markup-rows",
            "-no-fixed-num-lines",
            "-m",
            "-5",
            "-selected-row",
            &cal.selected_row.to_string(),
            "-font",
            "Monospace 10",
            "-theme-str",
            &window_theme,
            // "-theme-str",
            // "inputbar {enabled: false;}",
        ])
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(format!("{}", cal).as_bytes())
            .expect("Failed to write to stdin");
    });
    let output = child.wait_with_output()?;
    Ok(output)
}

pub fn rofi_calendar(
    date: NaiveDate,
    first_week_day: &FirstWeekDay,
    show_weeks: &bool,
    highlight_today: bool,
) -> BoxResult<()> {
    let cal = Calendar::from_ym(date, first_week_day, show_weeks, highlight_today);
    let output = run_rofi(cal)?;
    let today = Local::now().naive_local().date();
    let new_date = match output.status.code() {
        Some(10) => Some(date.checked_sub_months(Months::new(1)).unwrap()),
        Some(11) => Some(date.checked_add_months(Months::new(1)).unwrap()),
        Some(12) => Some(date.checked_sub_months(Months::new(12)).unwrap()),
        Some(13) => Some(date.checked_add_months(Months::new(12)).unwrap()),
        Some(14) => Some(today),
        _ => None,
    };
    match new_date {
        Some(day) => {
            rofi_calendar(day, first_week_day, show_weeks, day == today)?;
        }
        None => {}
    }

    Ok(())
}
