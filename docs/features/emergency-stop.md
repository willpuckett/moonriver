# Emergency Stop

Moonriver provides quick access to emergency stop functionality for immediate printer shutdown.

## M112 Command

Type `M112` to immediately send an emergency stop signal:

```bash
> M112
🚨 EMERGENCY STOP TRIGGERED 🚨
```

## What Happens

When you execute `M112`:

1. **Command buffer is cleared** - All queued commands are discarded
2. **Emergency stop signal sent** - Klipper receives immediate stop command
3. **Motors disabled** - All stepper motors are immediately disabled
4. **Heaters shut off** - All heating elements are turned off
5. **Printer halted** - Printer enters an error state

## After Emergency Stop

After triggering M112, the printer must be reset:

### Firmware Restart

```bash
> FIRMWARE_RESTART
```

This resets Klipper and clears the emergency stop state.

### Full Restart

Alternatively, restart the entire Klipper service:

```bash
# On the printer host (via SSH)
sudo systemctl restart klipper
```

## When to Use

Use emergency stop when:

- **Collision imminent** - Toolhead about to crash
- **Fire hazard** - Burning smell or visible smoke
- **Runaway heating** - Temperature rising uncontrollably
- **Strange noises** - Grinding, clicking, or unusual sounds
- **Layer shift** - Print visibly misaligned
- **Spaghetti detected** - Print has failed catastrophically

## Alternative Methods

### Web Interface

Most web interfaces (Mainsail, Fluidd) have an emergency stop button.

### Physical Button

Some printers have a physical emergency stop button wired directly to the control board.

### SSH

```bash
ssh pi@printer.local
moonriver --host localhost --port 7125 M112
```

## Response Time

Moonriver sends the M112 command immediately:

- **No delay** - Command sent as soon as you press Enter
- **Priority handling** - Bypasses command queue
- **Direct WebSocket** - No HTTP overhead

Typical response time: **< 100ms**

## Prevention

While emergency stop is available, prevention is better:

### Before Printing

```bash
> G28              # Home all axes
> QUERY_ENDSTOPS   # Verify endstops working
> M105             # Check temperatures normal
```

### During Printing

Monitor:
- Temperature graphs
- Layer time estimates
- First layer adhesion
- Any unusual sounds

### Regular Maintenance

```bash
> G28, BED_MESH_CALIBRATE, SAVE_CONFIG
```

Keep printer well-maintained to reduce emergency stop needs.

## Safety Tips

1. **Don't panic** - Take a breath, then execute M112
2. **Stay nearby** - Especially during first layer
3. **Monitor temperatures** - Watch for thermal runaway
4. **Check belts** - Loose belts cause layer shifts
5. **Verify paths** - Preview G-code before printing

## Recovery Procedure

After emergency stop:

1. **Assess the situation**
   ```bash
   > FIRMWARE_RESTART
   ```

2. **Check for damage**
   - Inspect toolhead, bed, wiring
   - Look for melted plastic
   - Check belt tension

3. **Test basic functions**
   ```bash
   > M105           # Check temperature reporting
   > QUERY_ENDSTOPS # Verify endstops
   ```

4. **Re-home if safe**
   ```bash
   > G28
   ```

5. **Resume or restart print** (if appropriate)

## Scripting Safety

When using Moonriver in scripts, include error handling:

```bash
#!/bin/bash

trap 'moonriver --host printer.local --port 7125 M112' ERR

# Your commands here
moonriver --host printer.local --port 7125 G28
```

This automatically triggers emergency stop if any command fails.

## Multiple Printers

Emergency stop a specific printer:

```bash
# Stop printer1
moonriver --host printer1.local --port 7125 M112

# Stop all printers
for printer in printer{1..5}.local; do
    moonriver --host $printer --port 7125 M112
done
```

## Non-Emergency Stops

For non-emergency situations:

### Pause Print

```bash
> PAUSE
```

Pauses the current print without stopping completely.

### Cancel Print

```bash
> CANCEL_PRINT
```

Cancels the print but keeps printer operational.

### Stop Heating

```bash
> M104 S0    # Turn off extruder
> M140 S0    # Turn off bed
```

Stops heating without emergency shutdown.

## Remember

**M112 is for emergencies only.** For normal operations, use `PAUSE`, `CANCEL_PRINT`, or regular commands.

## Next Steps

- [Interactive Mode](/guide/interactive-mode) - Normal printer control
- [Quick Start](/guide/quick-start) - Common commands
- [Safety Best Practices](https://www.klipper3d.org/Safety.html) - Klipper safety guide
