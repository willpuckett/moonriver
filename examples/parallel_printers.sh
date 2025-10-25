#!/bin/bash

# Example script to demonstrate Moonriver usage with GNU Parallel
# This shows how to send commands to multiple printers simultaneously

# Array of printer hosts
PRINTERS=(
    "printer1.local"
    "printer2.local"
    "printer3.local"
)

# Function to send command to a printer
send_command() {
    local host=$1
    local command=$2
    moonriver --host "$host" --port 7125 "$command"
}

export -f send_command

# Example: Home all printers in parallel
echo "Homing all printers..."
parallel -j 0 send_command {} "G28" ::: "${PRINTERS[@]}"

# Example: Check temperature on all printers
echo "Checking temperatures..."
parallel -j 0 send_command {} "M105" ::: "${PRINTERS[@]}"

# Example: Run a macro on all printers
echo "Running PRINT_START macro..."
parallel -j 0 send_command {} "PRINT_START" ::: "${PRINTERS[@]}"
