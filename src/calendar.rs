use crate::rofi::{run_rofi, BoxResult};
use chrono::{prelude::*, Months};
use std::{env, fmt};

pub enum FirstWeekDay {
    Sun,
    Mon,
}

#[derive(Debug)]
pub struct Calendar {
    cal: Vec<[u32; 7]>,
    date: NaiveDateTime,
    weekdays: [String; 7],
    week_nums: Vec<u32>,
    pub show_weeks: bool,
    pub selected_row: u8,
    pub selected_col: u32,
    pub highlight_today: bool,
}

impl fmt::Display for Calendar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = 3;
        if self.show_weeks {
            write!(f, "{:width$} ", " ")?;
        }
        self.weekdays.iter().for_each(|e| {
            write!(f, "{:width$}", e).expect("Error writing to buffer");
        });
        write!(f, "{}", "\n")?;

        for (row_i, row) in self.cal.iter().enumerate() {
            if self.show_weeks {
                write!(f, "{:width$} ", self.week_nums[row_i]).expect("Error writing buffer");
            };
            row.iter().enumerate().for_each(|(col_i, e)| {
                let symb = match (self.selected_row - 1, self.selected_col - 1) {
                    (i, j) if (i, j, true) == (row_i as u8, col_i as u32, self.highlight_today) => {
                        format!("<b><u>{:>2}</u></b> ", e)
                    }
                    (i, j) if (i, j, false) == (row_i as u8, col_i as u32, self.highlight_today) => {
                        format!("{:>2} ", e)
                    }
                    _ => format!("{:width$}", format!("{:>2}", e)),
                };
                match e {
                    0 => {
                        write!(f, "{:width$}", format!("{:>2}", "")).expect("Error writing to buffer");
                    }
                    _ => {
                        write!(f, "{}", symb).expect("Error writing to buffer");
                    }
                };
            });
            write!(f, "{}", "\n")?;
        }
        Ok(())
    }
}

fn week_number(day: NaiveDate, first_week_day: &FirstWeekDay) -> u32 {
    match first_week_day {
        FirstWeekDay::Mon => day.iso_week().week(),
        FirstWeekDay::Sun => {
            let week_number_sun: u32 = day.format("%U").to_string().parse().unwrap();
            let is_not_sunday =
                (NaiveDate::from_ymd_opt(day.year() + 1, 1, 1).unwrap().weekday() != Weekday::Sun) as u32;
            (week_number_sun + is_not_sunday) % 53
        }
    }
}

impl Calendar {
    pub fn show_ym(&self) -> String {
        let format = env::var("FORMAT_DATE").unwrap_or("%B %Y".into());
        // format!("{:^20}", self.date.format(&format))
        format!("{}", self.date.format(&format))
    }

    pub fn from_ym(
        date: NaiveDateTime,
        first_week_day: &FirstWeekDay,
        show_weeks: &bool,
        highlight_today: bool,
    ) -> Self {
        let month_first_day = date
            .with_day(1)
            .expect(&format!("Can't define beginnig of month. Source date: {}", date))
            .date();
        let year = date.year();
        let month = date.month();
        let month_last_day = NaiveDate::from_ymd_opt(year, month + 1, 1)
            .unwrap_or_else(|| {
                NaiveDate::from_ymd_opt(year + 1, 1, 1)
                    .expect(&format!("Can't define end of month. Source date: {}", date))
            })
            .pred_opt()
            .expect(&format!("Can't define end of month. Source date: {}", date));
        let mut idx = match first_week_day {
            FirstWeekDay::Sun => month_first_day.weekday().num_days_from_sunday(),
            FirstWeekDay::Mon => month_first_day.weekday().num_days_from_monday(),
        };

        debug!("Month first date: {}", month_first_day);
        debug!("Week day: {}", month_first_day.weekday());
        debug!("Number from: {}", idx);
        debug!("Month last date: {}", month_last_day);

        let mut cal: Vec<[u32; 7]> = vec![];
        let mut week: [u32; 7] = [0; 7];
        let mut week_nums: Vec<u32> = vec![];
        week_nums.push(week_number(month_first_day, first_week_day));
        let mut row_i = 0;
        let mut selected_row = 1;
        let mut selected_col = 1;
        for d in month_first_day.iter_days() {
            if d > month_last_day {
                cal.push(week);
                break;
            }
            if idx != 0 && idx % 7 == 0 {
                week_nums.push(week_number(d, first_week_day));
                row_i += 1;
                cal.push(week);
                week = [0; 7];
            }
            week[(idx % 7) as usize] = d.day();
            if d == date.date() {
                selected_row = row_i + 1;
                selected_col = (idx % 7) + 1;
            }
            idx += 1;
        }
        Self {
            cal,
            date,
            weekdays: match first_week_day {
                FirstWeekDay::Sun => [
                    "Su".to_owned(),
                    "Mo".to_owned(),
                    "Tu".to_owned(),
                    "We".to_owned(),
                    "Th".to_owned(),
                    "Fr".to_owned(),
                    "Sa".to_owned(),
                ],
                FirstWeekDay::Mon => [
                    "Mo".to_owned(),
                    "Tu".to_owned(),
                    "We".to_owned(),
                    "Th".to_owned(),
                    "Fr".to_owned(),
                    "Sa".to_owned(),
                    "Su".to_owned(),
                ],
            },
            week_nums,
            show_weeks: *show_weeks,
            selected_row,
            selected_col,
            highlight_today,
        }
    }
}

pub fn rofi_calendar(
    date: NaiveDateTime,
    first_week_day: &FirstWeekDay,
    show_weeks: &bool,
    highlight_today: bool,
) -> BoxResult<()> {
    let cal = Calendar::from_ym(date, first_week_day, show_weeks, highlight_today);
    let output = run_rofi(cal)?;
    let today = Local::now().naive_local();
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
            rofi_calendar(day, first_week_day, show_weeks, day.date() == today.date())?;
        }
        None => {}
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;

    fn iter_compare<T>(a: &[T], b: &[T]) -> bool
    where
        T: PartialEq,
    {
        (a.len() == a.len()) && a.iter().zip(b.iter()).all(|(aa, bb)| aa == bb)
    }

    fn cal_compare(va: &Vec<[u32; 7]>, vb: &Vec<[u32; 7]>) -> bool {
        (va.len() == vb.len())
            && va
                .iter()
                .zip(vb)
                .all(|(a, b)| (a.len() == b.len()) && a.iter().zip(b.iter()).all(|(aa, bb)| aa == bb))
    }

    #[test]
    fn test_calendar_sun() {
        let etalon_cal = vec![
            [0, 1, 2, 3, 4, 5, 6],
            [7, 8, 9, 10, 11, 12, 13],
            [14, 15, 16, 17, 18, 19, 20],
            [21, 22, 23, 24, 25, 26, 27],
            [28, 29, 30, 31, 0, 0, 0],
        ];
        let etalon_weekdays = [
            "Su".to_owned(),
            "Mo".to_owned(),
            "Tu".to_owned(),
            "We".to_owned(),
            "Th".to_owned(),
            "Fr".to_owned(),
            "Sa".to_owned(),
        ];
        let etalon_week_nums = vec![1, 2, 3, 4, 5];
        let etalon_selected_row = 1;
        let etalon_selected_col = 6;
        let etalon_show_weeks = true;
        let etalon_highlight_today = false;

        let day = NaiveDateTime::parse_from_str("2024-01-05 00:00", "%Y-%m-%d %H:%M").unwrap();
        let cal = super::Calendar::from_ym(day, &super::FirstWeekDay::Sun, &true, false);

        assert!(cal_compare(&etalon_cal, &cal.cal));
        assert!(iter_compare(&etalon_weekdays, &cal.weekdays));
        assert!(iter_compare(&etalon_week_nums, &cal.week_nums));
        assert_eq!(etalon_selected_row, cal.selected_row);
        assert_eq!(etalon_selected_col, cal.selected_col);
        assert_eq!(etalon_show_weeks, cal.show_weeks);
        assert_eq!(etalon_highlight_today, cal.highlight_today);
    }

    #[test]
    fn test_calendar_mon() {
        let etalon_cal = vec![
            [1, 2, 3, 4, 5, 6, 7],
            [8, 9, 10, 11, 12, 13, 14],
            [15, 16, 17, 18, 19, 20, 21],
            [22, 23, 24, 25, 26, 27, 28],
            [29, 30, 31, 0, 0, 0, 0],
        ];
        let etalon_weekdays = [
            "Mo".to_owned(),
            "Tu".to_owned(),
            "We".to_owned(),
            "Th".to_owned(),
            "Fr".to_owned(),
            "Sa".to_owned(),
            "Su".to_owned(),
        ];
        let etalon_week_nums = vec![1, 2, 3, 4, 5];
        let etalon_selected_row = 1;
        let etalon_selected_col = 5;
        let etalon_show_weeks = true;
        let etalon_highlight_today = false;

        let day = NaiveDateTime::parse_from_str("2024-01-05 00:00", "%Y-%m-%d %H:%M").unwrap();
        let cal = super::Calendar::from_ym(day, &super::FirstWeekDay::Mon, &true, false);

        assert!(cal_compare(&etalon_cal, &cal.cal));
        assert!(iter_compare(&etalon_weekdays, &cal.weekdays));
        assert!(iter_compare(&etalon_week_nums, &cal.week_nums));
        assert_eq!(etalon_selected_row, cal.selected_row);
        assert_eq!(etalon_selected_col, cal.selected_col);
        assert_eq!(etalon_show_weeks, cal.show_weeks);
        assert_eq!(etalon_highlight_today, cal.highlight_today);
    }
}
