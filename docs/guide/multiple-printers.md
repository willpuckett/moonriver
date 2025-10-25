# Multiple Printers

Moonriver is designed to work efficiently with multiple printers using tools
like GNU Parallel.

## GNU Parallel

[GNU Parallel](https://www.gnu.org/software/parallel/) is a shell tool for
executing jobs in parallel.

### Installation

```bash
# Debian/Ubuntu
sudo apt-get install parallel

# macOS
brew install parallel

# Fedora/RHEL
sudo dnf install parallel
```

## Basic Parallel Usage

### Run on Multiple Printers

```bash
# Home all printers in parallel
parallel moonriver --host {} --port 7125 G28 ::: printer1 printer2 printer3
```

### Check Temperatures

```bash
# Check temperature on all printers
parallel moonriver --host {} --port 7125 M105 ::: \
    192.168.1.101 \
    192.168.1.102 \
    192.168.1.103
```

### Execute Multiple Commands

```bash
# Home and check temperature
parallel moonriver --host {} --port 7125 "G28, M105" ::: \
    ender.local \
    prusa.local \
    voron.local
```

## Parallel Options

### Control Parallelism

```bash
# Run maximum 2 jobs in parallel
parallel -j 2 moonriver --host {} --port 7125 G28 ::: printer{1..10}

# Run all jobs in parallel
parallel -j 0 moonriver --host {} --port 7125 G28 ::: printer{1..10}
```

### With Progress

```bash
# Show progress bar
parallel --bar moonriver --host {} --port 7125 G28 ::: printer{1..10}

# Show which printer is being processed
parallel --tag moonriver --host {} --port 7125 G28 ::: printer1 printer2 printer3
```

### Error Handling

```bash
# Continue even if some jobs fail
parallel --keep-order --halt never moonriver --host {} --port 7125 G28 ::: printer{1..10}

# Stop all if one fails
parallel --halt now,fail=1 moonriver --host {} --port 7125 G28 ::: printer{1..10}
```

## Advanced Examples

### Printer Array

```bash
#!/bin/bash

# Define printer array
PRINTERS=(
    "printer1.local"
    "printer2.local"
    "printer3.local"
    "printer4.local"
    "printer5.local"
)

# Home all printers
echo "Homing all printers..."
parallel -j 0 --bar moonriver --host {} --port 7125 G28 ::: "${PRINTERS[@]}"

# Check temperatures
echo "Checking temperatures..."
parallel -j 0 --tag moonriver --host {} --port 7125 M105 ::: "${PRINTERS[@]}"
```

### Maintenance Script

```bash
#!/bin/bash
# Daily maintenance for all printers

PRINTERS=(printer{1..10}.local)

echo "Starting daily maintenance..."

# Step 1: Home all printers
parallel -j 0 --bar --tag moonriver --host {} --port 7125 G28 ::: "${PRINTERS[@]}"

# Step 2: Run bed leveling
parallel -j 2 --tag moonriver --host {} --port 7125 BED_MESH_CALIBRATE ::: "${PRINTERS[@]}"

# Step 3: Save configuration
parallel -j 0 moonriver --host {} --port 7125 SAVE_CONFIG ::: "${PRINTERS[@]}"

echo "Maintenance complete!"
```

### Pre-Print Setup

```bash
#!/bin/bash
# Prepare multiple printers for printing

PRINTERS=(
    "printer1.local:200:60"
    "printer2.local:210:70"
    "printer3.local:200:60"
)

prepare_printer() {
    IFS=':' read -r host extruder_temp bed_temp <<< "$1"
    
    echo "Preparing $host..."
    moonriver --host "$host" --port 7125 \
        "G28, M104 S$extruder_temp, M140 S$bed_temp, M109 S$extruder_temp, M190 S$bed_temp"
    
    echo "$host ready!"
}

export -f prepare_printer

parallel -j 0 --tag prepare_printer ::: "${PRINTERS[@]}"
```

### Status Monitoring

```bash
#!/bin/bash
# Monitor all printers continuously

PRINTERS=(printer{1..5}.local)

while true; do
    clear
    echo "=== Printer Status $(date) ==="
    echo
    
    parallel -j 0 --tag moonriver --host {} --port 7125 M105 ::: "${PRINTERS[@]}"
    
    sleep 5
done
```

## Load Balancing

Distribute work across printers:

```bash
#!/bin/bash
# Print multiple files across available printers

FILES=(model1.gcode model2.gcode model3.gcode model4.gcode)
PRINTERS=(printer1 printer2 printer3 printer4)

# Assign each file to a printer
for i in "${!FILES[@]}"; do
    file="${FILES[$i]}"
    printer="${PRINTERS[$((i % ${#PRINTERS[@]}))]}"
    
    echo "Assigning $file to $printer"
    # Your print start logic here
done
```

## Parallel with Input File

Create a printer list file:

```
# printers.txt
printer1.local
printer2.local
printer3.local
printer4.local
printer5.local
```

Use it:

```bash
# Home all printers from file
parallel -a printers.txt moonriver --host {} --port 7125 G28

# With multiple commands
parallel -a printers.txt moonriver --host {} --port 7125 "G28, M105"
```

## Example Script

See the included example script:

```bash
# View the example
cat examples/parallel_printers.sh

# Make it executable
chmod +x examples/parallel_printers.sh

# Run it (after editing for your printers)
./examples/parallel_printers.sh
```

## Tips

### Limit Parallelism

Don't overwhelm your network:

```bash
# Max 5 printers at a time
parallel -j 5 moonriver --host {} --port 7125 G28 ::: printer{1..20}.local
```

### Tag Output

See which printer each message is from:

```bash
parallel --tag moonriver --host {} --port 7125 M105 ::: printer{1..5}
```

Output:

```
printer1	T:20.5 /0.0 B:21.1 /0.0
printer2	T:21.2 /0.0 B:20.8 /0.0
printer3	T:19.9 /0.0 B:21.5 /0.0
```

### Timeout Protection

Set timeouts to prevent hanging:

```bash
parallel --timeout 60 moonriver --host {} --port 7125 G28 ::: printer{1..10}
```

### Dry Run

Test without executing:

```bash
parallel --dry-run moonriver --host {} --port 7125 G28 ::: printer{1..5}
```

## Without GNU Parallel

If GNU Parallel isn't available, use shell background jobs:

```bash
#!/bin/bash

PRINTERS=(printer1 printer2 printer3)

# Start all jobs in background
for printer in "${PRINTERS[@]}"; do
    moonriver --host "$printer" --port 7125 G28 &
done

# Wait for all to complete
wait

echo "All printers homed!"
```

## Next Steps

- [Scripting Mode](/guide/scripting-mode) - More automation examples
- [Configuration](/guide/configuration) - Set up aliases
- [Examples](https://github.com/willpuckett/moonriver/tree/main/examples) - More
  scripts
