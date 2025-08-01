use chrono::{Datelike, Local, NaiveDate, Weekday};
use colored::*;

// Configurable spacing between months (number of spaces)
const SPACE_BETWEEN_MONTHS: usize = 3;

// Month names constant to avoid repetition
const MONTH_NAMES: [&str; 12] = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December"
];

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
        Self::with_today(year, mode, today)
    }
    
    pub fn with_today(year: i32, mode: DisplayMode, today: NaiveDate) -> Self {
        Self { year, mode, today: Some(today) }
    }
    
    #[allow(dead_code)] // Used in tests
    pub fn today(&self) -> Option<NaiveDate> {
        self.today
    }
    
    fn month_width(&self) -> usize {
        match self.mode {
            DisplayMode::Full => 20,
            DisplayMode::WeekdaysOnly => 14,
        }
    }
    
    fn day_header(&self) -> &str {
        match self.mode {
            DisplayMode::Full => "Su Mo Tu We Th Fr Sa",
            DisplayMode::WeekdaysOnly => "Mo Tu We Th Fr",
        }
    }

    pub fn display(&self) {
        print!("{}", self.format_year());
    }
    
    pub fn format_year(&self) -> String {
        let mut output = String::new();
        
        // Calculate total width based on display mode
        let month_width = self.month_width();
        let total_width = month_width * 3 + SPACE_BETWEEN_MONTHS * 2;
        
        output.push_str(&format!("{:^width$}\n", self.year, width = total_width));
        output.push('\n');

        // Display calendar in quarters (3 months per row)
        for quarter in 0..4 {
            output.push_str(&self.format_quarter(quarter));
            if quarter < 3 {
                output.push('\n');
            }
        }
        
        output
    }

    pub fn display_months(&self, current_month: i32, months_before: i32, months_after: i32) {
        print!("{}", self.format_months(current_month, months_before, months_after));
    }
    
    pub fn format_months(&self, current_month: i32, months_before: i32, months_after: i32) -> String {
        let mut output = String::new();
        
        // Calculate which months to display
        let months_to_display = self.calculate_months_to_display(current_month, months_before, months_after);
        
        // Format months in groups of up to 3 per row
        for chunk in months_to_display.chunks(3) {
            output.push_str(&self.format_month_row(chunk));
            if chunk.len() == 3 && months_to_display.len() > 3 {
                output.push('\n');
            }
        }
        
        output
    }

    fn calculate_months_to_display(&self, current_month: i32, months_before: i32, months_after: i32) -> Vec<(i32, i32)> {
        let mut months = Vec::new();
        
        // Add months before
        for i in (1..=months_before).rev() {
            let (year, month) = self.subtract_months(self.year, current_month, i);
            months.push((year, month));
        }
        
        // Add current month
        months.push((self.year, current_month));
        
        // Add months after
        for i in 1..=months_after {
            let (year, month) = self.add_months(self.year, current_month, i);
            months.push((year, month));
        }
        
        months
    }

    fn add_months(&self, year: i32, month: i32, add: i32) -> (i32, i32) {
        let total_months = (year - 1) * 12 + (month - 1) + add;
        let new_year = total_months / 12 + 1;
        let new_month = total_months % 12 + 1;
        (new_year, new_month)
    }

    fn subtract_months(&self, year: i32, month: i32, subtract: i32) -> (i32, i32) {
        let total_months = (year - 1) * 12 + (month - 1) - subtract;
        let new_year = total_months / 12 + 1;
        let new_month = total_months % 12 + 1;
        (new_year, new_month)
    }

    fn format_month_row(&self, months: &[(i32, i32)]) -> String {
        let mut output = String::new();
        let month_width = self.month_width();
        
        let current_year = Local::now().year();
        
        // Month headers
        let month_header = months
            .iter()
            .map(|(year, month)| {
                let month_name = MONTH_NAMES[(*month - 1) as usize];
                if *year == current_year {
                    // Current year - never show year
                    format!("{:^width$}", month_name, width = month_width)
                } else {
                    // Different year - show as 2-digit year
                    let two_digit_year = *year % 100;
                    format!("{:^width$}", format!("{} {}", month_name, two_digit_year), width = month_width)
                }
            })
            .collect::<Vec<_>>()
            .join(&" ".repeat(SPACE_BETWEEN_MONTHS));
        output.push_str(&format!("{}\n", month_header));

        // Day headers
        let day_header_str = self.day_header();
        let headers = vec![day_header_str; months.len()].join(&" ".repeat(SPACE_BETWEEN_MONTHS));
        output.push_str(&format!("{}\n", headers));

        // Generate month grids
        let month_grids: Vec<Vec<Vec<String>>> = months
            .iter()
            .map(|(year, month)| self.generate_month_grid_for_year_month(*year, *month as usize))
            .collect();

        // Generate rows
        let max_rows = month_grids.iter().map(|grid| grid.len()).max().unwrap_or(0);
        for row in 0..max_rows {
            let row_parts: Vec<String> = month_grids
                .iter()
                .map(|grid| {
                    if row < grid.len() {
                        self.format_grid_row(&grid[row])
                    } else {
                        " ".repeat(month_width)
                    }
                })
                .collect();
            output.push_str(&format!("{}\n", row_parts.join(&" ".repeat(SPACE_BETWEEN_MONTHS))));
        }
        
        output
    }

    fn generate_month_grid_for_year_month(&self, year: i32, month: usize) -> Vec<Vec<String>> {
        let mut grid = Vec::new();
        
        let first_day = NaiveDate::from_ymd_opt(year, month as u32, 1).unwrap();
        let days_in_month = self.days_in_month_for_year(year, month);
        
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
            let weekday = self.get_weekday_for_day_and_year(year, month, current_day);
            if matches!(self.mode, DisplayMode::WeekdaysOnly) && self.is_weekend(weekday) {
                current_day += 1;
                continue;
            }
            
            // Format the day
            let day_str = self.format_day_for_year_month(year, month, current_day, weekday);
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

    fn get_weekday_for_day_and_year(&self, year: i32, month: usize, day: u32) -> Weekday {
        NaiveDate::from_ymd_opt(year, month as u32, day)
            .unwrap()
            .weekday()
    }

    fn format_day_for_year_month(&self, year: i32, month: usize, day: u32, weekday: Weekday) -> String {
        let day_str = format!("{:2}", day);
        
        // Check if this is today
        if let Some(today) = self.today {
            if today.year() == year && today.month() == month as u32 && today.day() == day {
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

    fn days_in_month_for_year(&self, year: i32, month: usize) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.is_leap_year_for(year) {
                    29
                } else {
                    28
                }
            }
            _ => panic!("Invalid month: {}", month),
        }
    }

    fn is_leap_year_for(&self, year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    fn format_quarter(&self, quarter: usize) -> String {
        let mut output = String::new();
        let months: Vec<usize> = (0..3).map(|i| quarter * 3 + i + 1).collect();
        
        // Month headers - adjust width based on mode
        let month_width = self.month_width();
        
        let month_header = months
            .iter()
            .map(|&m| format!("{:^width$}", MONTH_NAMES[m - 1], width = month_width))
            .collect::<Vec<_>>()
            .join(&" ".repeat(SPACE_BETWEEN_MONTHS));
        output.push_str(&format!("{}\n", month_header));

        // Day headers
        let day_header_str = self.day_header();
        let headers = vec![day_header_str; 3].join(&" ".repeat(SPACE_BETWEEN_MONTHS));
        output.push_str(&format!("{}\n", headers));

        // Generate month grids
        let month_grids: Vec<Vec<Vec<String>>> = months
            .iter()
            .map(|&month| self.generate_month_grid(month))
            .collect();

        // Generate rows
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
            output.push_str(&format!("{}\n", row_parts.join(&" ".repeat(SPACE_BETWEEN_MONTHS))));
        }
        
        output
    }

    fn generate_month_grid(&self, month: usize) -> Vec<Vec<String>> {
        // Delegate to the more flexible method using self.year
        self.generate_month_grid_for_year_month(self.year, month)
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
    
    fn is_weekend(&self, weekday: Weekday) -> bool {
        weekday == Weekday::Sat || weekday == Weekday::Sun
    }
    
    fn format_grid_row(&self, row: &[String]) -> String {
        let row_str = row.join(" ");
        match self.mode {
            DisplayMode::Full => format!("{:20}", row_str),
            DisplayMode::WeekdaysOnly => format!("{:14}", row_str), // Reduced width, no extra padding
        }
    }

    #[allow(dead_code)] // Used in tests
    fn days_in_month(&self, month: usize) -> u32 {
        self.days_in_month_for_year(self.year, month)
    }

    #[allow(dead_code)] // Used in tests
    fn is_leap_year(&self) -> bool {
        self.is_leap_year_for(self.year)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_leap_year_calculation() {
        let cal_2024 = Calendar::new(2024, DisplayMode::Full);
        assert!(cal_2024.is_leap_year());
        
        let cal_2023 = Calendar::new(2023, DisplayMode::Full);
        assert!(!cal_2023.is_leap_year());
        
        let cal_1900 = Calendar::new(1900, DisplayMode::Full);
        assert!(!cal_1900.is_leap_year());
        
        let cal_2000 = Calendar::new(2000, DisplayMode::Full);
        assert!(cal_2000.is_leap_year());
    }

    #[test]
    fn test_days_in_month() {
        let cal = Calendar::new(2024, DisplayMode::Full);
        assert_eq!(cal.days_in_month(2), 29); // Leap year February
        assert_eq!(cal.days_in_month(4), 30); // April
        assert_eq!(cal.days_in_month(1), 31); // January
        
        let cal_non_leap = Calendar::new(2023, DisplayMode::Full);
        assert_eq!(cal_non_leap.days_in_month(2), 28); // Non-leap year February
    }

    // TODO: These tests demonstrate current testability issues:
    
    #[test]
    fn test_calendar_with_specific_today_date() {
        // FIXED: Now we can inject a specific "today" date for testing
        let specific_date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        let cal = Calendar::with_today(2024, DisplayMode::Full, specific_date);
        
        // Test that the injected date is used
        assert_eq!(cal.today(), Some(specific_date));
        
        // Test day formatting with known today date
        let day_str = cal.format_day_for_year_month(2024, 3, 15, chrono::Weekday::Fri);
        assert!(day_str.contains("15")); // Should contain the day number
        // Note: Actual highlighting test would require capturing the colored output
    }
    
    #[test]  
    fn test_generate_month_output_as_string() {
        // FIXED: Now we can test formatted output directly
        let specific_date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        let cal = Calendar::with_today(2024, DisplayMode::Full, specific_date);
        
        // Test full year formatting
        let year_output = cal.format_year();
        assert!(year_output.contains("2024"));
        assert!(year_output.contains("January"));
        assert!(year_output.contains("December"));
        assert!(year_output.contains("Su Mo Tu We Th Fr Sa"));
        
        // Test month range formatting  
        let months_output = cal.format_months(3, 1, 1); // Feb, Mar, Apr
        assert!(months_output.contains("February"));
        assert!(months_output.contains("March"));
        assert!(months_output.contains("April"));
        
        // Test weekdays-only mode
        let cal_weekdays = Calendar::with_today(2024, DisplayMode::WeekdaysOnly, specific_date);
        let weekdays_output = cal_weekdays.format_year();
        assert!(weekdays_output.contains("Mo Tu We Th Fr"));
        assert!(!weekdays_output.contains("Su"));
        assert!(!weekdays_output.contains("Sa"));
    }
    
    #[test]
    fn test_formatting_methods() {
        let cal_full = Calendar::new(2024, DisplayMode::Full);
        let cal_weekdays = Calendar::new(2024, DisplayMode::WeekdaysOnly);
        
        // Test month width calculation
        assert_eq!(cal_full.month_width(), 20);
        assert_eq!(cal_weekdays.month_width(), 14);
        
        // Test day header generation
        assert_eq!(cal_full.day_header(), "Su Mo Tu We Th Fr Sa");
        assert_eq!(cal_weekdays.day_header(), "Mo Tu We Th Fr");
        
        // Test month names constant
        assert_eq!(MONTH_NAMES[0], "January");
        assert_eq!(MONTH_NAMES[11], "December");
    }
    
    #[test]
    fn test_month_calculations() {
        let cal = Calendar::new(2024, DisplayMode::Full);
        
        // Test add_months logic
        assert_eq!(cal.add_months(2024, 12, 1), (2025, 1)); // Year boundary
        assert_eq!(cal.add_months(2024, 6, 3), (2024, 9));  // Same year
        
        // Test subtract_months logic  
        assert_eq!(cal.subtract_months(2025, 1, 1), (2024, 12)); // Year boundary
        assert_eq!(cal.subtract_months(2024, 6, 2), (2024, 4));   // Same year
    }
    
    #[test]
    fn test_edge_cases() {
        let specific_date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(); // Leap day
        let cal = Calendar::with_today(2024, DisplayMode::Full, specific_date);
        
        // Test leap year February
        assert_eq!(cal.days_in_month_for_year(2024, 2), 29);
        assert_eq!(cal.days_in_month_for_year(2023, 2), 28);
        
        // Test century years (edge case for leap year)
        assert!(!cal.is_leap_year_for(1900)); // Not leap (divisible by 100)
        assert!(cal.is_leap_year_for(2000));  // Leap (divisible by 400)
        
        // Test weekday calculations
        let friday = cal.get_weekday_for_day_and_year(2024, 3, 15);
        assert_eq!(friday, chrono::Weekday::Fri);
        
        // Test weekend detection
        assert!(cal.is_weekend(chrono::Weekday::Sat));
        assert!(cal.is_weekend(chrono::Weekday::Sun));
        assert!(!cal.is_weekend(chrono::Weekday::Mon));
    }
}