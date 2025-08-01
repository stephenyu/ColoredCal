use clap::Parser;
use chrono::{Datelike, Local};

mod calendar;
use calendar::{Calendar, DisplayMode};

#[derive(Parser)]
#[command(
    name = "ccal",
    about = "Enhanced calendar with weekend coloring and weekday-only option",
    version = "0.1.0"
)]
struct Args {
    /// Show only weekdays (Monday-Friday)
    #[arg(short = 'w', long = "weekdays")]
    weekdays_only: bool,

    /// Display months only. No args = current month only. One arg = current + N months after. Two args = N months before, current, N months after.
    #[arg(short = 'm', long = "months", num_args = 0..=2)]
    months: Option<Vec<i32>>,

    /// Year to display (defaults to current year)
    year: Option<i32>,
}

fn main() {
    let args = Args::parse();
    
    let mode = if args.weekdays_only {
        DisplayMode::WeekdaysOnly
    } else {
        DisplayMode::Full
    };

    if let Some(months_args) = args.months {
        // Month display mode - hide year
        let now = Local::now();
        let current_year = now.year();
        let current_month = now.month() as i32;
        
        let (months_before, months_after) = match months_args.len() {
            0 => (0, 0), // Just current month
            1 => (0, months_args[0]), // Current + N months after
            2 => (months_args[0], months_args[1]), // N before + current + N after
            _ => unreachable!(), // clap ensures 0..=2 args
        };

        let calendar = Calendar::new(current_year, mode);
        calendar.display_months(current_month, months_before, months_after);
    } else {
        // Full year display mode - show year
        let year = args.year.unwrap_or_else(|| Local::now().year());
        let calendar = Calendar::new(year, mode);
        calendar.display();
    }
}