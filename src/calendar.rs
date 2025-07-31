use chrono::{Datelike, Local, NaiveDate, Weekday};
use colored::*;

// Configurable spacing between months (number of spaces)
const SPACE_BETWEEN_MONTHS: usize = 3;

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
        
        // Month headers - adjust width based on mode
        let month_width = match self.mode {
            DisplayMode::Full => 20,
            DisplayMode::WeekdaysOnly => 14, // Reduced width for weekdays-only
        };
        
        let month_header = months
            .iter()
            .map(|&m| format!("{:^width$}", month_names[m - 1], width = month_width))
            .collect::<Vec<_>>()
            .join(&" ".repeat(SPACE_BETWEEN_MONTHS));
        println!("{}", month_header);

        // Day headers - remove extra padding in weekdays-only mode
        let day_header = match self.mode {
            DisplayMode::Full => "Su Mo Tu We Th Fr Sa",
            DisplayMode::WeekdaysOnly => "Mo Tu We Th Fr", // Removed extra spaces
        };
        let headers = vec![day_header; 3].join(&" ".repeat(SPACE_BETWEEN_MONTHS));
        println!("{}", headers);

        // Generate month grids
        let month_grids: Vec<Vec<Vec<String>>> = months
            .iter()
            .map(|&month| self.generate_month_grid(month))
            .collect();

        // Print rows
        let max_rows = month_grids.iter().map(|grid| grid.len()).max().unwrap_or(0);
        for row in 0..max_rows {
            let row_parts: Vec<String> = month_grids
                .iter()
                .map(|grid| {
                    if row < grid.len() {
                        self.format_grid_row(&grid[row])
                    } else {
                        " ".repeat(month_width) // Use dynamic width
                    }
                })
                .collect();
            println!("{}", row_parts.join(&" ".repeat(SPACE_BETWEEN_MONTHS)));
        }
    }

    fn generate_month_grid(&self, month: usize) -> Vec<Vec<String>> {
        let mut grid = Vec::new();
        
        let first_day = NaiveDate::from_ymd_opt(self.year, month as u32, 1).unwrap();
        let days_in_month = self.days_in_month(month);
        
        let cols = match self.mode {
            DisplayMode::Full => 7,
            DisplayMode::WeekdaysOnly => 5,
        };
        
        let mut current_row = vec!["  ".to_string(); cols];
        let mut current_day = 1;
        
        // Calculate starting column
        let start_col = self.get_start_column(first_day);
        let mut col = start_col;
        
        // Fill the grid
        while current_day <= days_in_month {
            if col >= cols {
                // Start new row
                grid.push(current_row);
                current_row = vec!["  ".to_string(); cols];
                col = 0;
            }
            
            // Skip weekends in weekdays-only mode
            let weekday = self.get_weekday_for_day(month, current_day);
            if matches!(self.mode, DisplayMode::WeekdaysOnly) && self.is_weekend(weekday) {
                current_day += 1;
                continue;
            }
            
            // Format the day
            let day_str = self.format_day(month, current_day, weekday);
            current_row[col] = day_str;
            
            current_day += 1;
            col += 1;
        }
        
        // Add the last row if it has content
        if current_row.iter().any(|cell| cell != "  ") {
            grid.push(current_row);
        }
        
        grid
    }
    
    fn get_start_column(&self, first_day: NaiveDate) -> usize {
        match self.mode {
            DisplayMode::Full => first_day.weekday().num_days_from_sunday() as usize,
            DisplayMode::WeekdaysOnly => {
                let weekday = first_day.weekday();
                if weekday == Weekday::Sat || weekday == Weekday::Sun {
                    // If month starts on weekend, start at column 0 (Monday)
                    0
                } else {
                    // Convert to weekday index: Mon=0, Tue=1, ..., Fri=4
                    (weekday.num_days_from_monday() as usize).min(4)
                }
            }
        }
    }
    
    fn get_weekday_for_day(&self, month: usize, day: u32) -> Weekday {
        NaiveDate::from_ymd_opt(self.year, month as u32, day)
            .unwrap()
            .weekday()
    }
    
    fn is_weekend(&self, weekday: Weekday) -> bool {
        weekday == Weekday::Sat || weekday == Weekday::Sun
    }
    
    fn format_day(&self, month: usize, day: u32, weekday: Weekday) -> String {
        let day_str = format!("{:2}", day);
        
        // Check if this is today
        if let Some(today) = self.today {
            if today.month() == month as u32 && today.day() == day {
                return format!("{}", day_str.black().on_white());
            }
        }
        
        // Check if this is a weekend (only color in full mode)
        if matches!(self.mode, DisplayMode::Full) && self.is_weekend(weekday) {
            format!("{}", day_str.bright_black())
        } else {
            day_str
        }
    }
    
    fn format_grid_row(&self, row: &[String]) -> String {
        let row_str = row.join(" ");
        match self.mode {
            DisplayMode::Full => format!("{:20}", row_str),
            DisplayMode::WeekdaysOnly => format!("{:14}", row_str), // Reduced width, no extra padding
        }
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