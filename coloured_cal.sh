#!/bin/bash

# coloured-cal.sh - Enhanced calendar with weekend colouring and weekday-only option
# Usage: ./coloured-cal.sh [-w] [year]
# -w: Show only weekdays (Monday-Friday)  
# year: Year to display (defaults to current year)

# Colour codes
GREY='\033[90m'    # Dark grey for weekends
TODAY_HIGHLIGHT='\033[30;47m'  # Black text on white background for today
RESET='\033[0m'    # Reset to normal

# Parse arguments
WEEKDAYS_ONLY=false
YEAR=$(date +%Y)

# Get current date info for highlighting
CURRENT_YEAR=$(date +%Y)
CURRENT_MONTH=$(date +%-m)  # Remove leading zero
CURRENT_DAY=$(date +%-d)    # Remove leading zero

while [[ $# -gt 0 ]]; do
    case $1 in
        -w)
            WEEKDAYS_ONLY=true
            shift
            ;;
        *)
            if [[ $1 =~ ^[0-9]{4}$ ]]; then
                YEAR=$1
            else
                echo "Usage: $0 [-w] [year]"
                echo "  -w: Show only weekdays (Monday-Friday)"
                echo "  year: Year to display (defaults to current year)"
                exit 1
            fi
            shift
            ;;
    esac
done

if [[ "$WEEKDAYS_ONLY" == true ]]; then
    # Weekdays only version - replace weekend columns with spaces to maintain alignment
    cal -y "$YEAR" | awk -v current_year="$CURRENT_YEAR" -v current_month="$CURRENT_MONTH" -v current_day="$CURRENT_DAY" -v display_year="$YEAR" '
    BEGIN {
        # Track which quarter (row of months) we are in
        quarter = 1  # 1=Jan-Mar, 2=Apr-Jun, 3=Jul-Sep, 4=Oct-Dec
    }
    
    # Year header line
    /^[[:space:]]*[0-9]{4}[[:space:]]*$/ {
        print
        next
    }
    
    # Month names line - determine which quarter we are in
    /January|February|March/ { quarter = 1; print; next }
    /April|May|June/ { quarter = 2; print; next }
    /July|August|September/ { quarter = 3; print; next }
    /October|November|December/ { quarter = 4; print; next }
    
    # Day header line (Su Mo Tu We Th Fr Sa)
    /Su Mo Tu We Th Fr Sa/ {
        # Replace weekend headers with spaces
        gsub(/Su/, "  ")
        gsub(/Sa/, "  ")
        print
        next
    }
    
    # Data lines (containing day numbers)
    /^[[:space:]]*[0-9]/ {
        result = ""
        line_len = length($0)
        
        # Calculate which months are in current quarter
        month1 = (quarter - 1) * 3 + 1
        month2 = (quarter - 1) * 3 + 2  
        month3 = (quarter - 1) * 3 + 3
        
        # Check if we should highlight today (only if displaying current year)
        should_highlight = (display_year == current_year)
        
        # Process character by character
        for (i = 1; i <= line_len; i++) {
            char = substr($0, i, 1)
            # Determine position within each 22-character month block
            pos_in_month = ((i - 1) % 22) + 1
            month_section = int((i - 1) / 22) + 1  # 1, 2, or 3
            
            # Determine which month this character belongs to
            current_month_num = 0
            if (month_section == 1) current_month_num = month1
            else if (month_section == 2) current_month_num = month2  
            else if (month_section == 3) current_month_num = month3
            
            # Sunday column (positions 1-3) and Saturday column (positions 19-21)
            if ((pos_in_month >= 1 && pos_in_month <= 3) || (pos_in_month >= 19 && pos_in_month <= 21)) {
                # Replace weekend digits with spaces, keep existing spaces
                if (char ~ /[0-9]/) {
                    result = result " "
                } else {
                    result = result char
                }
            } else {
                # Check if this is today'\''s date that needs highlighting
                if (char ~ /[0-9]/ && should_highlight && current_month_num == current_month) {
                    # Only check if this is the start of a number (not preceded by a digit)
                    prev_char = (i > 1) ? substr($0, i-1, 1) : ""
                    
                    if (prev_char !~ /[0-9]/) {
                        # Extract the full number (could be 1 or 2 digits)
                        day_str = ""
                        j = i
                        while (j <= line_len && substr($0, j, 1) ~ /[0-9]/) {
                            day_str = day_str substr($0, j, 1)  
                            j++
                        }
                        if (int(day_str) == current_day) {
                            # Highlight today'\''s date - need to handle this specially
                            if (length(day_str) == 1) {
                                result = result "\033[30;47m" char "\033[0m"
                            } else if (i == j - length(day_str)) {
                                # First digit of 2-digit number
                                result = result "\033[30;47m" char
                            } else {
                                # Second digit of 2-digit number  
                                result = result char "\033[0m"
                            }
                        } else {
                            result = result char
                        }
                    } else {
                        result = result char
                    }
                } else {
                    result = result char
                }
            }
        }
        print result
        next
    }
    
    # Empty lines and other lines
    {
        print
    }'
    
else
    # Default version with coloured weekends and spacing between months
    # Track which quarter we're in for today highlighting
    quarter=1
    
    cal -y "$YEAR" | while IFS= read -r line; do
        # Track which quarter (row of months) we're displaying
        if [[ "$line" =~ January|February|March ]]; then
            quarter=1  
        elif [[ "$line" =~ April|May|June ]]; then
            quarter=2
        elif [[ "$line" =~ July|August|September ]]; then
            quarter=3
        elif [[ "$line" =~ October|November|December ]]; then
            quarter=4
        fi
        
        # Check if this is a data line with numbers
        if [[ "$line" =~ ^[[:space:]]*[0-9] ]]; then
            # Calculate which months are in current quarter
            month1=$(( (quarter - 1) * 3 + 1 ))
            month2=$(( (quarter - 1) * 3 + 2 ))
            month3=$(( (quarter - 1) * 3 + 3 ))
            
            # Process character by character to identify weekend positions and today's date
            result=""
            line_len=${#line}
            
            for (( pos=0; pos<line_len; pos++ )); do
                char="${line:$pos:1}"
                
                # Determine which month column we're in (0-21, 22-43, 44-65)
                month_section=$((pos / 22))
                col_in_month=$((pos % 22))
                
                # Determine which month this position represents
                case $month_section in
                    0) current_month_num=$month1 ;;
                    1) current_month_num=$month2 ;;
                    2) current_month_num=$month3 ;;
                    *) current_month_num=0 ;;
                esac
                
                # Check if this character is part of today's date
                is_today=false
                if [[ "$char" =~ [0-9] ]] && [[ "$YEAR" == "$CURRENT_YEAR" ]] && [[ $current_month_num -eq $CURRENT_MONTH ]]; then
                    # Only check if this is the start of a number (not preceded by a digit)
                    prev_char=""
                    if [[ $pos -gt 0 ]]; then
                        prev_char="${line:$((pos-1)):1}"
                    fi
                    
                    if [[ ! "$prev_char" =~ [0-9] ]]; then
                        # Extract the full number (handle 1 or 2 digits)
                        day_str=""
                        temp_pos=$pos
                        while [[ $temp_pos -lt $line_len ]] && [[ "${line:$temp_pos:1}" =~ [0-9] ]]; do
                            day_str+="${line:$temp_pos:1}"
                            temp_pos=$((temp_pos + 1))
                        done
                        
                        if [[ ${day_str#0} -eq $CURRENT_DAY ]]; then
                            is_today=true
                        fi
                    fi
                fi
                
                # Apply appropriate styling
                if [[ $is_today == true ]]; then
                    # Today's date - black text on white background
                    result+="${TODAY_HIGHLIGHT}${char}${RESET}"
                elif [[ "$char" =~ [0-9] ]] && { [[ $col_in_month -le 2 ]] || [[ $col_in_month -ge 18 && $col_in_month -le 20 ]]; }; then
                    # Weekend columns: Su (0-2) and Sa (18-20)
                    result+="${GREY}${char}${RESET}"
                else
                    result+="${char}"
                fi
                
                # Add space between months (after positions 21 and 43, but not after 65)
                if [[ $pos -eq 21 || $pos -eq 43 ]]; then
                    result+=" "
                fi
            done
            
            echo -e "$result"
        else
            # Non-data line (headers, month names, etc.) - add spacing between months
            if [[ "$line" =~ (January|February|March|April|May|June|July|August|September|October|November|December|Su Mo Tu We Th Fr Sa) ]]; then
                # Add space between month sections for headers
                modified_line=$(echo "$line" | sed 's/\(.\{22\}\)/\1 /g' | sed 's/ $//')
                echo "$modified_line"
            else
                echo "$line"
            fi
        fi
    done
fi