use chrono::{Datelike, Local, NaiveDate};
use colored::*;

#[derive(Clone, Copy)]
pub enum DisplayMode {
    Full,
    WeekdaysOnly,
}

pub struct Calendar {
    year: i32,
    mode: DisplayMode,
    today: Option<NaiveDate>,
}

impl Calendar {
    pub fn new(year: i32, mode: DisplayMode) -> Self {
        let today = Local::now().date_naive();
        let today = if today.year() == year {
            Some(today)
        } else {
            None
        };

        Self { year, mode, today }
    }

    pub fn display(&self) {
        println!("{:^66}", self.year);
        println!();

        // Display calendar in quarters (3 months per row)
        for quarter in 0..4 {
            self.display_quarter(quarter);
            if quarter < 3 {
                println!();
            }
        }
    }

    fn display_quarter(&self, quarter: usize) {
        let months: Vec<usize> = (0..3).map(|i| quarter * 3 + i + 1).collect();
        
        // Month names
        let month_names = [
            "January", "February", "March", "April", "May", "June",
            "July", "August", "September", "October", "November", "December"
        ];
        
        let month_header = months
            .iter()
            .map(|&m| format!("{:^20}", month_names[m - 1]))
            .collect::<Vec<_>>()
            .join("  ");
        println!("{}", month_header);

        // Day headers
        let day_header = match self.mode {
            DisplayMode::Full => "Su Mo Tu We Th Fr Sa",
            DisplayMode::WeekdaysOnly => "   Mo Tu We Th Fr   ",
        };
        let headers = vec![day_header; 3].join("  ");
        println!("{}", headers);

        // Calendar body
        self.display_month_data(&months);
    }

    fn display_month_data(&self, months: &[usize]) {
        let mut month_calendars: Vec<Vec<String>> = Vec::new();
        
        for &month in months {
            month_calendars.push(self.generate_month_lines(month));
        }

        let max_lines = month_calendars.iter().map(|cal| cal.len()).max().unwrap_or(0);

        for line_idx in 0..max_lines {
            let line_parts: Vec<String> = month_calendars
                .iter()
                .map(|cal| {
                    if line_idx < cal.len() {
                        cal[line_idx].clone()
                    } else {
                        match self.mode {
                            DisplayMode::Full => " ".repeat(20),
                            DisplayMode::WeekdaysOnly => " ".repeat(20),
                        }
                    }
                })
                .collect();
            
            println!("{}", line_parts.join("  "));
        }
    }

    fn generate_month_lines(&self, month: usize) -> Vec<String> {
        let mut lines = Vec::new();
        
        let first_day = NaiveDate::from_ymd_opt(self.year, month as u32, 1).unwrap();
        let days_in_month = self.days_in_month(month);
        
        // Calculate starting position (0 = Sunday, 6 = Saturday)
        let start_weekday = first_day.weekday().num_days_from_sunday() as usize;
        
        let mut current_day = 1;


        // Fill the calendar
        while current_day <= days_in_month {
            let mut line = match self.mode {
                DisplayMode::Full => vec!["  ".to_string(); 7],
                DisplayMode::WeekdaysOnly => vec!["  ".to_string(); 5],
            };

            for day_of_week in 0..7 {
                if current_day > days_in_month {
                    break;
                }

                let should_skip_weekend = match self.mode {
                    DisplayMode::WeekdaysOnly => day_of_week == 0 || day_of_week == 6, // Skip Sun/Sat
                    DisplayMode::Full => false,
                };

                if should_skip_weekend {
                    if current_day == 1 && day_of_week <= start_weekday {
                        // Do nothing, haven't started yet
                    } else if current_day > 1 || day_of_week >= start_weekday {
                        current_day += 1;
                    }
                    continue;
                }

                if current_day == 1 && day_of_week < start_weekday {
                    // Empty space before month starts
                    continue;
                }

                let col_idx = match self.mode {
                    DisplayMode::Full => day_of_week,
                    DisplayMode::WeekdaysOnly => {
                        if day_of_week == 0 { continue; } // Skip Sunday
                        if day_of_week == 6 { continue; } // Skip Saturday
                        day_of_week - 1 // Convert to Mon(0)-Fri(4)
                    }
                };

                let day_str = format!("{:2}", current_day);
                
                if let Some(today) = self.today {
                    if today.month() == month as u32 && today.day() == current_day as u32 {
                        // Highlight today
                        line[col_idx] = format!("{}", day_str.black().on_white());
                    } else if matches!(self.mode, DisplayMode::Full) && (day_of_week == 0 || day_of_week == 6) {
                        // Weekend coloring (only in full mode)
                        line[col_idx] = format!("{}", day_str.bright_black());
                    } else {
                        line[col_idx] = day_str;
                    }
                } else if matches!(self.mode, DisplayMode::Full) && (day_of_week == 0 || day_of_week == 6) {
                    // Weekend coloring when not current year
                    line[col_idx] = format!("{}", day_str.bright_black());
                } else {
                    line[col_idx] = day_str;
                }

                current_day += 1;
            }

            let line_str = match self.mode {
                DisplayMode::Full => line.join(" "),
                DisplayMode::WeekdaysOnly => format!("  {}  ", line.join(" ")),
            };
            
            lines.push(format!("{:20}", line_str));
        }

        lines
    }

    fn days_in_month(&self, month: usize) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.is_leap_year() {
                    29
                } else {
                    28
                }
            }
            _ => panic!("Invalid month: {}", month),
        }
    }

    fn is_leap_year(&self) -> bool {
        (self.year % 4 == 0 && self.year % 100 != 0) || (self.year % 400 == 0)
    }
}