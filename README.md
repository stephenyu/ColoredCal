# Colored Calendar 🦀

A fast, modern Rust implementation of an enhanced calendar tool that improves upon the standard Unix `cal` command with colored weekends and weekday-only display options.

## Features

- **Colored Weekends**: Displays Saturday and Sunday in grey for better visual distinction
- **Weekdays-Only Mode**: Shows only Monday through Friday when using the `-w` flag  
- **Today's Date Highlighting**: Highlights today's date with inverted colors (when displaying current year)
- **Year Selection**: Display any year (defaults to current year)
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
- `-h, --help`: Print help information
- `-V, --version`: Print version information
- `YEAR`: Year to display (defaults to current year)

### Examples

```bash
# Display current year with colored weekends
ccal

# Display 2024 with colored weekends  
ccal 2024

# Display current year with weekdays only
ccal -w

# Display 2025 with weekdays only
ccal -w 2025

# Using cargo run during development
cargo run -- -w 2024
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

The Rust implementation provides two display modes:

1. **Full Mode** (default): Shows complete calendar with weekend days colored in grey
2. **Weekdays-Only Mode** (`-w` flag): Displays only Monday through Friday, removing weekend columns entirely

The calendar uses the `chrono` crate for accurate date calculations and `colored` for terminal styling, ensuring consistent behavior across different systems.

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