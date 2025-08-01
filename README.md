# Colored Calendar 🦀

A fast, modern Rust implementation of an enhanced calendar tool that improves upon the standard Unix `cal` command with colored weekends, weekday-only display options, and flexible month viewing.

## Features

- **Colored Weekends**: Displays Saturday and Sunday in grey for better visual distinction
- **Weekdays-Only Mode**: Shows only Monday through Friday when using the `-w` flag  
- **Today's Date Highlighting**: Highlights today's date with inverted colors (when displaying current year)
- **Year Selection**: Display any year (defaults to current year)
- **Month Display Mode**: Show specific months with the `-m` flag instead of full year view
- **Smart Year Display**: Hides current year, shows future years as 2-digit format (e.g., "26" for 2026)
- **Fast Performance**: Built with Rust for blazing-fast execution
- **Clean Output**: Maintains the familiar `cal` command layout with visual enhancements

## Installation

### From Source (Recommended)

1. **Prerequisites**: Install [Rust](https://rustup.rs/) if you haven't already
2. **Clone and build**:
   ```bash
   git clone <repository-url>
   cd ccal
   cargo build --release
   ```
3. **Install globally** (optional):
   ```bash
   cargo install --path .
   ```

### Quick Run (Without Installation)

```bash
cargo run -- [OPTIONS] [YEAR]
```

## Usage

```bash
ccal [OPTIONS] [YEAR]
```

### Options

- `-w, --weekdays`: Show only weekdays (Monday-Friday)
- `-m, --months [BEFORE] [AFTER]`: Display months only (hides year). See examples below for usage patterns.
- `-h, --help`: Print help information
- `-V, --version`: Print version information
- `YEAR`: Year to display (defaults to current year, ignored when using `-m`)

### Month Display Mode (`-m` flag)

The `-m` flag provides flexible month viewing with intelligent year display:

- **Current year months**: Year is hidden (e.g., "August", "September")
- **Future year months**: Year shown as 2-digit format (e.g., "January 26", "February 26")
- **Combines with `-w`**: Works seamlessly with weekdays-only mode

#### `-m` Usage Patterns:

- `-m` (no arguments): Show current month only
- `-m X` (one number): Show current month + X months after
- `-m X Y` (two numbers): Show X months before + current month + Y months after

### Examples

#### Year View (Default)
```bash
# Display current year with colored weekends
ccal

# Display 2024 with colored weekends  
ccal 2024

# Display current year with weekdays only
ccal -w

# Display 2025 with weekdays only
ccal -w 2025
```

#### Month View (`-m` flag)
```bash
# Show current month only
ccal -m

# Show current month + next month
ccal -m 1

# Show current month + next 2 months
ccal -m 2

# Show previous month + current + next month
ccal -m 1 1

# Show 2 months before + current + 1 month after
ccal -m 2 1

# Month view with weekdays only
ccal -m 3 -w

# Show current month through next 5 months (spans years)
ccal -m 5
```

#### Development Usage
```bash
# Using cargo run during development
cargo run -- -w 2024
cargo run -- -m 2 1
```

## Requirements

- **Rust**: 1.70+ (2021 edition)
- **Dependencies**: 
  - `clap` - Command line argument parsing
  - `chrono` - Date and time handling
  - `colored` - Terminal color output

## Color Scheme

- **Weekdays**: Default terminal color
- **Weekends (Saturday/Sunday)**: Grey
- **Today's Date**: Inverted colors (black text on white background)

## How It Works

The Rust implementation provides three display modes:

1. **Full Year Mode** (default): Shows complete year calendar with weekend days colored in grey
2. **Weekdays-Only Mode** (`-w` flag): Displays only Monday through Friday, removing weekend columns entirely
3. **Month Display Mode** (`-m` flag): Shows specific months without year header, with smart year labeling:
   - Current year months: No year shown
   - Future/past years: 2-digit year format (e.g., "26" for 2026)

The calendar uses the `chrono` crate for accurate date calculations and `colored` for terminal styling, ensuring consistent behavior across different systems. Month calculations properly handle year transitions.

## Development

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with arguments
cargo run -- --help
```

### Project Structure

```
src/
├── main.rs        # CLI interface and argument parsing
└── calendar.rs    # Core calendar logic and display
```

## Performance

This Rust implementation is significantly faster than shell script alternatives, with:
- ⚡ Instant startup time
- 🔧 Zero external command dependencies  
- 🎯 Memory-efficient date calculations
- 🌐 Cross-platform compatibility

## Contributing

Feel free to submit issues or pull requests to improve the calendar tool!

## License

This project is open source and available under the MIT OR Apache-2.0 license. 