# Position Bar

The **Position Bar** provides a compact, single-line display of your printer's position with interactive controls for movement and homing.

## Overview

The position bar shows real-time coordinates and allows you to quickly move axes or home your printer with simple mouse clicks.

```
📍 ✓X:150.5  ✓Y:150.5  ✓Z:15.50  │  🏠 Home All
```

## Display Format

The position bar shows:
- **📍** — Position indicator icon
- **✓/✗** — Homed status for each axis (✓ = homed, ✗ = not homed)
- **X/Y/Z** — Current coordinates with floating point precision
- **🏠 Home All** — Clickable button to home all axes

### Color Coding

- **Green (✓)** — Axis is homed
- **Red (✗)** — Axis is not homed
- **Cyan** — Coordinate values (bright when homed, dark gray when not)
- **Yellow** — Home All button
- **Yellow + Bold + Underlined** — Active editing mode

## Interactive Features

### Click to Move Axes

Click on any coordinate to edit and move that axis:

1. **Click** anywhere on the coordinate (e.g., entire `X:150.5` section)
2. **Type** the new coordinate (supports decimals and negative values)
3. **Press Enter** to send the move command
4. **Press Escape** to cancel

The entire coordinate section is clickable for easier interaction:
- Click anywhere in `X:150.5` to edit X position
- Click anywhere in `Y:150.5` to edit Y position
- Click anywhere in `Z:15.50` to edit Z position

Example:
```
Click on X:150.5 → Type "200" → Press Enter
Sends: G0 X200.00 F3000
```

### Home All Axes

Click the **🏠 Home All** button to home all axes at once:
- Click anywhere on the button (emoji or text)
- Sends the G28 command
- Homes X, Y, and Z axes sequentially
- Updates homed status indicators when complete
- Console shows: "🏠 Homing all axes..."

### Keyboard Shortcuts

When editing a position:
- **0-9** — Enter digits
- **.** — Decimal point (one per value)
- **-** — Negative sign (only at start)
- **Backspace** — Delete last character
- **Enter** — Apply the change and send G-code
- **Escape** — Cancel editing

## Position Commands

The position bar uses standard G-code commands:

### Move Commands
```gcode
G0 X200.00 F3000    # Move X to 200mm at 3000mm/min
G0 Y150.50 F3000    # Move Y to 150.5mm
G0 Z25.00 F3000     # Move Z to 25mm
```

### Home Commands
```gcode
G28              # Home all axes
G28 X            # Home X only
G28 Y            # Home Y only
G28 Z            # Home Z only
```

## Toggle Visibility

Control position bar visibility:
- **`l` key** — Toggle **L**ocation bar on/off (globally across all tabs)
- **`p` key** — Switch to **P**osition tab
- **Mouse** — Click on tabs in the footer to switch views

The position bar appears at the top of all tabs when visible:
- Main Dashboard
- Console
- Jobs
- Position Control
- Help

## Position Ranges

The position bar validates coordinates based on typical printer ranges:
- **X/Y axes**: 0-400mm (configurable per printer)
- **Z axis**: 0-400mm (configurable per printer)
- **Negative values**: Supported for printers with negative coordinates

When you enter an out-of-range value, you'll see an error message:
```
Error: Position must be between 0 and 400mm
```

## Visual Feedback

### Normal Display
```
📍 ✓X:150.5  ✓Y:150.5  ✓Z:15.50  │  🏠 Home All
```

### Editing X Position
```
📍 ✓X:[200]  ✓Y:150.5  ✓Z:15.50  │  🏠 Home All
```
The edited value appears in brackets with yellow highlighting.

### After Home Command
```
Info: 🏠 Homing all axes...
```

## Integration with Other Features

The position bar works seamlessly with other Moonriver features:

### Temperature Bar
When both temperature and position bars are visible:
```
🌡 E:210.5°/210°  🛌60.0°/60°  C:45.2°/45°  μC:42.3°  🌀75%(3500rpm)
📍 ✓X:150.5  ✓Y:150.5  ✓Z:15.50  │  🏠 Home All
```

### Console Output
Movement commands and responses appear in the console:
```
Command: G0 X200.00 F3000
Info: Moving X to 200.00mm
```

### System Panel
The system panel shows detailed position information:
```
Position:  X: 200.00  Y: 150.50  Z: 15.50
```

## Best Practices

### Before Moving
1. **Ensure axes are homed** (look for green ✓ indicators)
2. **Check current position** to avoid collisions
3. **Know your printer's limits** (bed size, Z height)

### Safe Movement
- **Move Z up** before moving X/Y to avoid nozzle collisions
- **Use smaller increments** when approaching limits
- **Watch the printer** during movement

### After Homing
- Homed status updates automatically via Moonraker WebSocket
- Position displays show accurate coordinates
- Safe to perform precise movements

## Troubleshooting

### Position Not Updating
- Check Moonraker connection (header status)
- Verify axes are homed
- Look for error messages in console

### Can't Click Coordinates
- Ensure position bar is visible (press `p`)
- Check mouse is clicking on the coordinate values
- Verify not in temperature editing mode

### Movement Fails
- Check axes are homed first (home all with 🏠)
- Verify coordinates are within valid range
- Ensure printer is not in error state

## Advanced Usage

### Relative vs Absolute Positioning

The position bar always shows **absolute** coordinates (G90 mode). When you click and enter a value, it sends an absolute G0 command.

To use relative positioning:
```gcode
G91          # Switch to relative mode
G0 X10       # Move 10mm in positive X
G90          # Switch back to absolute mode
```

### Custom Feed Rates

The position bar uses F3000 (3000mm/min) by default. To use custom feed rates:
```gcode
G0 X200 F5000    # Move at 5000mm/min
```

### Multi-Axis Moves

For moves involving multiple axes, use the console:
```gcode
G0 X200 Y200 Z50 F3000
```

## Next Steps

- [Temperature Bar](/features/temperature-bar) - Monitor and control temperatures
- [Console](/guide/interactive-mode#console) - Send custom G-code commands
- [Position Control Tab](/guide/interactive-mode#position-control) - Additional movement controls
