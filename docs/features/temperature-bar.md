# Temperature Bar

The **Temperature Bar** provides a compact, single-line display of all printer temperatures with interactive controls for setting targets.

## Overview

The temperature bar shows real-time temperatures and allows you to quickly change setpoints with simple mouse clicks.

```
🌡 E:210.5°/210°  🛌60.0°/60°  C:45.2°/45°  μC:42.3°  🌀75%(3500rpm)
```

## Display Format

The temperature bar shows:
- **🌡** — Temperature indicator icon
- **E:** — Extruder (hotend) temperature: current°/target°
- **🛌** — Bed temperature: current°/target°
- **C:** — Chamber temperature: current°/target° (if equipped)
- **μC:** — MCU (controller) temperature (if available)
- **🌀** — Fan speed percentage and RPM (if available)

## Color Coding

Temperatures are color-coded based on proximity to target:

### Heater Colors
- **Green** — At target (within 2°C)
- **Yellow** — Approaching target (within 5°C)
- **Cyan** — Heating/cooling (within 15°C)
- **White** — Far from target (>15°C)
- **Dark Gray** — No target set (idle)

### MCU Temperature Colors
- **Green** — Cool (<50°C)
- **Yellow** — Warm (50-70°C)
- **Red** — Hot (70-85°C)
- **Magenta** — Very hot (>85°C)

### Fan Colors
- **Dark Gray** — Off (<1%)
- **Cyan** — Low speed (1-50%)
- **Green** — High speed (>50%)

## Interactive Features

### Click to Set Temperature

Click on any temperature setpoint to edit it:

1. **Click** on the target temperature (e.g., `/210°` for extruder)
2. **Type** the new target temperature (0-300°C)
3. **Press Enter** to apply the change
4. **Press Escape** to cancel

The entire temperature section is clickable for easier interaction:
- Click anywhere in `E:210.5°/210°` to edit extruder
- Click anywhere in `🛌60.0°/60°` to edit bed

Example:
```
Click on /210° → Type "250" → Press Enter
Sends: M104 S250
```

### Click to Set Fan Speed

Click on any fan speed percentage to edit it:

1. **Click** on the fan speed (e.g., `🌀75%`)
2. **Type** the new speed percentage (0-100)
3. **Press Enter** to apply the change
4. **Press Escape** to cancel

Example:
```
Click on 🌀75% → Type "50" → Press Enter
Sends: M106 S127 (for part fan)
or SET_FAN_SPEED FAN=name SPEED=0.50 (for named fans)
```

The fan speed updates immediately in the UI for responsive feedback, while the command is sent to the printer.

### Exclusive Editing

Only one temperature or fan speed can be edited at a time. Clicking on a new item automatically cancels any current edit, preventing confusion.

### Keyboard Shortcuts

When editing a temperature:
- **0-9** — Enter digits only (no decimals needed for setpoints)
- **Backspace** — Delete last digit
- **Enter** — Apply the change and send G-code
- **Escape** — Cancel editing

When editing a fan speed:
- **0-9** — Enter digits (0-100 for percentage)
- **Backspace** — Delete last digit
- **Enter** — Apply the change and send G-code
- **Escape** — Cancel editing

### Visual Feedback

When editing, the value is shown in brackets with yellow highlighting:
```
🌡 E:210.5°/[250°]  🛌60.0°/60°  🌀[50%]
```

## Temperature Commands

The temperature bar uses standard G-code commands:

### Extruder
```gcode
M104 S250    # Set extruder target to 250°C (don't wait)
M109 S250    # Set extruder target to 250°C (wait)
M104 S0      # Turn off extruder
```

### Bed
```gcode
M140 S60     # Set bed target to 60°C (don't wait)
M190 S60     # Set bed target to 60°C (wait)
M140 S0      # Turn off bed
```

### Check Temperatures
```gcode
M105         # Report all temperatures
```

## Toggle Visibility

Control temperature bar visibility:
- **`t` key** — Toggle **T**emperature bar on/off (globally across all tabs)
- **Mouse** — Click on tabs in the footer to switch views

The temperature bar appears at the top of all tabs when visible:
- Main Dashboard
- Console
- Position Control
- Jobs
- Help

## Temperature Ranges

The temperature bar validates temperatures:
- **Minimum**: 0°C (off)
- **Maximum**: 300°C (safety limit)

When you enter an invalid temperature, you'll see an error:
```
Error: Temperature must be between 0 and 300°C
```

## Additional Sensors

### MCU Temperature

Shows the microcontroller and additional temperature sensors (dynamically discovered):
```
μC:42.3°  HOST:38.5°  Chamber:45.2°
```

Moonriver automatically discovers and subscribes to:
- MCU temperature sensors
- Host (Raspberry Pi) temperature
- Custom temperature sensors (e.g., `temperature_sensor chamber`)
- Temperature fans

This is useful for monitoring:
- Controller board temperature
- Ambient temperature inside electronics enclosure
- Chamber temperature for materials like ABS
- Detecting cooling issues

### Fan Status

Shows fan speed and RPM (if tachometer is connected):
```
🌀Part:75%(3500rpm)  Hotend:100%  Chamber:50%
```

Multiple fans are automatically discovered and displayed:
- Part cooling fan (labeled "Part")
- Hotend fan
- Controller fan
- Temperature-controlled fans
- Custom fans

Each fan's speed is individually clickable for control.

### Dynamic Sensor Discovery

Moonriver uses Moonraker's `printer.objects.list` API to automatically discover all available:
- Temperature sensors (including custom ones)
- Fans (part cooling, heater fans, controller fans)
- MCU temperature sensors

No manual configuration needed - if it's in your `printer.cfg`, it will appear in Moonriver!

## Real-Time Updates

The temperature bar updates automatically via WebSocket:
- **Live temperatures** — Updates every 0.5-1 second
- **Target changes** — Reflects immediately after setting
- **Power levels** — Shows heater power percentage
- **Fan changes** — Updates when fan speed changes

## PID Tuning

Monitor temperature stability during PID tuning:

```gcode
PID_CALIBRATE HEATER=extruder TARGET=250
```

Watch the temperature display:
- Oscillations become smaller over time
- Final settling shows good PID values
- Color changes from cyan → yellow → green indicate approaching target

## Preheat Profiles

Common temperature presets:

### PLA
```
Extruder: 210°C
Bed: 60°C
```

### PETG
```
Extruder: 240°C
Bed: 80°C
```

### ABS
```
Extruder: 250°C
Bed: 100°C
Chamber: 50°C (if equipped)
```

## Integration with Console

Temperature changes also appear in the console:

```
Command: M104 S250
Info: Extruder target set to 250°C
```

This provides a full history of temperature changes during your session.

## Best Practices

### Heating Sequence
1. **Heat bed first** — Prevents warping during heat-up
2. **Then heat extruder** — Avoid filament cooking in idle nozzle
3. **Wait for both** — Use M190 and M109 in print start macros

### Safety
- **Never exceed 300°C** — Moonriver enforces this limit
- **Turn off when done** — Set targets to 0°C
- **Monitor during heating** — Watch for thermal runaway

### Efficiency
- **Preheat while preparing** — Save time by heating early
- **Batch prints** — Keep temperatures stable between jobs
- **Cool down naturally** — Don't force cooling with fans

## Troubleshooting

### Temperature Not Updating
- Check Moonraker connection (header shows "Connected")
- Verify heaters are configured in `printer.cfg`
- Look for error messages in console

### Can't Set Temperature
- Ensure printer is not in error state
- Check that axes are homed (required by some firmwares)
- Verify heater is defined and working

### Temperature Fluctuating
- May need PID tuning
- Check for drafts or cooling issues
- Verify thermistor connection

### Wrong Temperature Shown
- Thermistor may be loose or damaged
- Check wiring to heated bed/hotend
- Verify thermistor type in `printer.cfg`

## Advanced Usage

### Temperature Curves

Monitor heating curves in the console by periodically checking M105:

```
> M105
T:45.2 /210.0 B:23.5 /60.0
> M105
T:78.3 /210.0 B:31.2 /60.0
> M105
T:124.5 /210.0 B:45.8 /60.0
```

### Multi-Extruder Support

For printers with multiple extruders:
```
🌡 E0:210.5°/210°  E1:210.2°/210°  🛌60.0°/60°
```

Each extruder is clickable individually.

### Temperature Wait Commands

Use in print start macros:

```gcode
M190 S60     # Wait for bed to reach 60°C
M109 S210    # Wait for extruder to reach 210°C
```

The temperature bar shows progress as temperatures approach targets.

## Next Steps

- [Position Bar](/features/position-bar) - Control printer movement
- [Console](/guide/interactive-mode#console) - Send custom commands
- [System Panel](/guide/interactive-mode#system-panel) - View detailed status
