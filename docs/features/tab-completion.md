# Tab Completion

Moonriver provides intelligent tab completion for G-code commands and Klipper macros.

## Basic Usage

Press `Tab` while typing to see available completions:

```bash
> G<Tab>
G0   G1   G2   G3   G4   G10  G11  G28  G29  G90  G91  G92

> M1<Tab>
M104  M105  M106  M107  M109  M110  M111  M112  M113  M114  M115  M117  M118  M119  M120  M121
```

## G-code Commands

All standard G-code commands are available for completion:

### Movement
- G0, G1 - Linear move
- G2, G3 - Arc move
- G28 - Home
- G29 - Bed leveling

### Temperature
- M104 - Set extruder temperature
- M105 - Get temperature
- M109 - Set extruder temperature and wait
- M140 - Set bed temperature
- M190 - Set bed temperature and wait

### And many more...

## Klipper Macros

Tab completion includes your custom Klipper macros:

```bash
> PRINT_<Tab>
PRINT_START  PRINT_END  PRINT_PAUSE  PRINT_RESUME

> BED_<Tab>
BED_MESH_CALIBRATE  BED_MESH_PROFILE  BED_MESH_CLEAR
```

Moonriver automatically fetches available macros from your Klipper configuration when you connect.

## Partial Matching

You don't need to type from the beginning:

```bash
> mesh<Tab>
BED_MESH_CALIBRATE  BED_MESH_PROFILE  BED_MESH_CLEAR

> temp<Tab>
GET_TEMPERATURE  SET_HEATER_TEMPERATURE  TEMPERATURE_WAIT
```

## Case Insensitivity

Completion works regardless of case:

```bash
> g28<Tab>
G28

> m105<Tab>
M105
```

## Multiple Matches

When multiple options match, all are shown:

```bash
> M10<Tab>
M104  M105  M106  M107  M109
```

Continue typing to narrow down:

```bash
> M105<Tab>
M105  # Complete match, press Enter to execute
```

## How It Works

When you connect to Moonraker, Moonriver:

1. Fetches available G-code help via `printer.gcode.help`
2. Parses user-defined macros
3. Builds a completion database
4. Provides instant completions as you type

## Tips

- **Press Tab twice** to see all available completions
- **Type a few letters** before Tab for faster filtering
- **Use Tab liberally** - it's fast and helps you discover commands

## Customization

The completion system automatically adapts to your printer configuration. Custom macros you add to `printer.cfg` are immediately available for completion after connecting.

## Next Steps

- [Syntax Highlighting](/features/syntax-highlighting) - Visual feedback as you type
- [Command History](/features/command-history) - Reuse previous commands
