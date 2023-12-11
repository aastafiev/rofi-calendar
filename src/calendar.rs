use chrono::prelude::*;
use std::{env, fmt};

pub enum FirstWeekDay {
    Sun,
    Mon,
}

#[derive(Debug)]
pub struct Calendar {
    cal: Vec<[u32; 7]>,
    date: NaiveDate,
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
                    (i, j)
                        if (i, j, false) == (row_i as u8, col_i as u32, self.highlight_today) =>
                    {
                        format!("{:>2} ", e)
                    }
                    _ => format!("{:width$}", format!("{:>2}", e)),
                };
                match e {
                    0 => {
                        write!(f, "{:width$}", format!("{:>2}", ""))
                            .expect("Error writing to buffer");
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

impl Calendar {
    pub fn show_ym(&self) -> String {
        let format = env::var("FORMAT_DATE").unwrap_or("%B %Y".into());
        // format!("{:^20}", self.date.format(&format))
        format!("{}", self.date.format(&format))
    }

    pub fn from_ym(
        date: NaiveDate,
        first_week_day: &FirstWeekDay,
        show_weeks: &bool,
        highlight_today: bool,
    ) -> Self {
        let month_first_day = date.with_day(1).expect(&format!(
            "Can't define beginnig of month. Source date: {}",
            date
        ));
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
        week_nums.push(month_first_day.iso_week().week());
        let mut row_i = 0;
        let mut selected_row = 1;
        let mut selected_col = 1;
        for d in month_first_day.iter_days() {
            if d > month_last_day {
                cal.push(week);
                break;
            }
            if idx != 0 && idx % 7 == 0 {
                week_nums.push(d.iso_week().week());
                row_i += 1;
                cal.push(week);
                week = [0; 7];
            }
            week[(idx % 7) as usize] = d.day();
            if d == date {
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
