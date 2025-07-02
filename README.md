# Coloured Calendar

A bash script that enhances the standard Unix `cal` command with colored weekends and weekdays-only display options.

## Features

- **Colored Weekends**: Displays Saturday and Sunday in grey for better visual distinction
- **Weekdays-Only Mode**: Shows only Monday through Friday when using the `-w` flag
- **Today's Date Highlighting**: Highlights today's date with black text on white background (when displaying current year)
- **Year Selection**: Display any year (defaults to current year)
- **Clean Output**: Maintains the familiar `cal` command layout with enhancements

## Usage

```bash
./coloured_cal.sh [-w] [year]
```

### Options

- `-w`: Show only weekdays (Monday-Friday)
- `year`: Year to display (defaults to current year)

### Examples

```bash
# Display current year with colored weekends
./coloured_cal.sh

# Display 2024 with colored weekends  
./coloured_cal.sh 2024

# Display current year with weekdays only
./coloured_cal.sh -w

# Display 2025 with weekdays only
./coloured_cal.sh -w 2025
```

## Installation

1. Clone or download the script
2. Make it executable:
   ```bash
   chmod +x coloured_cal.sh
   ```
3. Run it from the current directory or add to your PATH

## Requirements

- Bash shell
- Standard Unix `cal` command
- AWK (for weekdays-only mode)

## Color Scheme

- **Weekdays**: Default terminal color
- **Weekends**: Grey (`243`)
- **Today's Date**: Black text on white background (`\033[30;47m`)

## How It Works

The script operates in two modes:

1. **Default Mode**: Uses the standard `cal` output and colors weekend columns (Saturday/Sunday) in grey
2. **Weekdays-Only Mode**: Uses AWK to parse and filter the `cal` output, removing weekend columns entirely

## Contributing

Feel free to submit issues or pull requests to improve the script.

## License

This project is open source and available under standard terms. 