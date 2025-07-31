use clap::Parser;
use chrono::{Datelike, Local};

mod calendar;
use calendar::{Calendar, DisplayMode};

#[derive(Parser)]
#[command(
    name = "colored-cal",
    about = "Enhanced calendar with weekend coloring and weekday-only option",
    version = "0.1.0"
)]
struct Args {
    /// Show only weekdays (Monday-Friday)
    #[arg(short = 'w', long = "weekdays")]
    weekdays_only: bool,

    /// Year to display (defaults to current year)
    year: Option<i32>,
}

fn main() {
    let args = Args::parse();
    
    let year = args.year.unwrap_or_else(|| Local::now().year());
    let mode = if args.weekdays_only {
        DisplayMode::WeekdaysOnly
    } else {
        DisplayMode::Full
    };

    let calendar = Calendar::new(year, mode);
    calendar.display();
}